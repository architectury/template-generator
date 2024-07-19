// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use clap::Parser;
use cliclack::{confirm, input, intro, multiselect, select};
use miette::{miette, Context, IntoDiagnostic, Result};
use strum::IntoEnumIterator;
use version_resolver::minecraft::MinecraftVersion;
use std::path::PathBuf;

use crate::{Dependencies, GeneratorApp, MappingSet, ProjectType, Subprojects};
use crate::filer::ZipFilerProvider;
use crate::filer::native::{DirectoryFilerProvider, FsZipWriteTarget};

#[derive(Parser)]
#[command(version)]
struct Args {
    /// The project path (default: the current directory)
    output: Option<PathBuf>,
    /// Output a zip instead of a directory
    #[arg(short, long)]
    zip: bool,
}

pub async fn main() -> Result<()> {
    let args = Args::parse();
    if args.zip {
        let (file, default_name) = if let Some(output) = &args.output {
            // If the file was provided, try to derive the mod name from it.
            let name = output.file_name()
                .and_then(|s| s.to_str())
                .map(|s| s.strip_suffix(".zip").unwrap_or(s));
            (FsZipWriteTarget::ZipFile(output.clone()), name)
        } else {
            // If the file wasn't provided, get the current dir and use the default file name inside.
            let dir = std::env::current_dir()
                .into_diagnostic()
                .wrap_err("Couldn't get current directory")?;
            (FsZipWriteTarget::InDirectory(dir), None)
        };
        let app = prompt(default_name)?;
        let filer_provider = ZipFilerProvider(file);
        crate::generator::generate(&app, &filer_provider).await
    } else {
        let dir = if let Some(directory) = args.output {
            directory
        } else {
            std::env::current_dir()
                .into_diagnostic()
                .wrap_err("Couldn't get current directory")?
        };

        if dir.exists() {
            if !dir.is_dir() {
                return Err(miette!("File {} is not a directory", dir.to_string_lossy()));
            }

            // Check that the directory is empty.
            let mut iter = tokio::fs::read_dir(&dir)
                .await
                .into_diagnostic()
                .wrap_err("Could not check if the output directory is empty")?;
            if iter.next_entry().await.into_diagnostic()?.is_some() {
                return Err(miette!("Output directory {} is not empty", dir.to_string_lossy()));
            }
        }

        let default_name = dir.file_name().and_then(|s| s.to_str());
        let app = prompt(default_name)?;
        crate::generator::generate(&app, &DirectoryFilerProvider(&dir)).await
    }
}

fn prompt(default_name: Option<&str>) -> Result<GeneratorApp> {
    intro("Architectury Template Generator").into_diagnostic()?;

    let mut mod_name = input("Mod name");
    if let Some(name) = default_name {
        mod_name = mod_name.default_input(name);
    }
    let mod_name: String = mod_name.interact().into_diagnostic()?;

    let mod_id: String = input("Mod ID")
        .default_input(&crate::mod_ids::to_mod_id(&mod_name))
        .validate_interactively(ModIdValidate)
        .interact().into_diagnostic()?;

    let package_name: String = input("Package name")
        .interact().into_diagnostic()?;

    let mut versions: Vec<_> = MinecraftVersion::iter()
        .map(|version| {
            (version, version.version(), "")
        })
        .collect();
    versions.reverse(); // newest first
    let game_version = select("Minecraft version")
        .items(&versions)
        .interact().into_diagnostic()?;

    let mapping_sets: Vec<_> = MappingSet::iter()
        .map(|set| {
            (set, set.name(), set.description())
        })
        .collect();
    let mapping_set = select("Mappings")
        .items(&mapping_sets)
        .interact().into_diagnostic()?;

    let mut project_types = vec![
        (ProjectType::Multiplatform, "Multiplatform", ""),
    ];
    if game_version.forge_major_version().is_some() {
        project_types.push((ProjectType::Forge, "Forge", ""));
    }
    if game_version.neoforge_major().is_some() {
        project_types.push((ProjectType::NeoForge, "NeoForge", ""));
    }
    let project_type: ProjectType = select("Project type")
        .items(&project_types)
        .interact().into_diagnostic()?;

    let mut subprojects = Subprojects::default();
    let mut dependencies = Dependencies::default();

    if project_type == ProjectType::Multiplatform {
        let mut subproject_options: Vec<_> = vec![
            (Subproject::Fabric, "Fabric", ""),
            (Subproject::Forge, "Forge", ""),
            (Subproject::NeoForge, "NeoForge", ""),
            (Subproject::Quilt, "Quilt", ""),
        ];
        subproject_options.retain(|(s, _, _)| s.is_available_on(&game_version));
        let chosen_subprojects = multiselect("Mod loaders")
            .items(&subproject_options)
            .interact().into_diagnostic()?;

        for subproject in chosen_subprojects {
            subproject.apply_to(&mut subprojects);
        }

        if subprojects.fabric && subprojects.quilt {
            subprojects.fabric_likes = confirm("Fabric-like subproject (shared code between Fabric and Quilt)?")
                .initial_value(subprojects.fabric_likes)
                .interact().into_diagnostic()?;
        }

        dependencies.architectury_api = confirm("Architectury API?")
            .initial_value(dependencies.architectury_api)
            .interact().into_diagnostic()?;
    }

    let generator = GeneratorApp {
        mod_name,
        mod_id,
        package_name,
        game_version,
        project_type,
        subprojects,
        mapping_set,
        dependencies
    };
    Ok(generator)
}

// TODO: Can this be a function ref?
struct ModIdValidate;

impl cliclack::Validate<String> for ModIdValidate {
    type Err = miette::Error;

    fn validate(&self, input: &String) -> Result<()> {
        crate::mod_ids::validate_mod_id(input)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Subproject {
    Fabric,
    Forge,
    NeoForge,
    Quilt,
}

impl Subproject {
    pub fn is_available_on(&self, game_version: &MinecraftVersion) -> bool {
        match self {
            Self::Forge => game_version.forge_major_version().is_some(),
            Self::NeoForge => game_version.neoforge_major().is_some(),
            _ => true,
        }
    }

    pub fn apply_to(&self, settings: &mut crate::Subprojects) {
        match self {
            Self::Fabric => settings.fabric = true,
            Self::Forge => settings.forge = true,
            Self::NeoForge => settings.neoforge = true,
            Self::Quilt => settings.quilt = true,
        }
    }
}
