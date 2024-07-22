// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use clap::Parser;
use cliclack::{confirm, input, intro, multiselect, outro, select, spinner};
use eyre::{eyre, Context, Result};
use strum::IntoEnumIterator;
use version_resolver::minecraft::MinecraftVersion;
use std::path::PathBuf;

use crate::{Dependencies, GeneratorApp, MappingSet, ProjectType, Subprojects};
use crate::filer::{FilerProvider, ZipFilerProvider};
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
    color_eyre::install()?;
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
            let dir = get_current_dir()?;
            (FsZipWriteTarget::InDirectory(dir), None)
        };

        run(ZipFilerProvider(file), default_name, |app| {
            if let Some(output) = &args.output {
                output.to_string_lossy().into_owned()
            } else {
                crate::generator::compose_file_name(app) + ".zip"
            }
        })
        .await?
    } else {
        let dir = if let Some(directory) = args.output {
            directory
        } else {
            get_current_dir()?
        };

        if dir.exists() {
            if !dir.is_dir() {
                return Err(eyre!("File {} is not a directory", dir.to_string_lossy()));
            }

            // Check that the directory is empty.
            let mut iter = tokio::fs::read_dir(&dir)
                .await
                .wrap_err("Could not check if the output directory is empty")?;
            if iter.next_entry().await?.is_some() {
                return Err(eyre!("Output directory {} is not empty", dir.to_string_lossy()));
            }
        }

        let default_name = dir.file_name().and_then(|s| s.to_str());
        run(DirectoryFilerProvider(&dir), default_name, |_| {
            dir.to_string_lossy()
        })
        .await?
    }
    Ok(())
}

async fn run<F, N, D>(filer_provider: F, default_mod_name: Option<&str>, output_name_provider: N) -> Result<()>
where
    F: FilerProvider,
    N: FnOnce(&GeneratorApp) -> D,
    D: std::fmt::Display,
{
    let app = prompt(default_mod_name)?;
    let spinner = spinner();
    spinner.start("Generating...");
    crate::generator::generate(&app, &filer_provider).await?;
    spinner.stop("Done!");
    outro(format!("Generated into {}!", output_name_provider(&app)))?;
    Ok(())
}

fn prompt(default_name: Option<&str>) -> Result<GeneratorApp> {
    intro("Architectury Template Generator")?;

    let mut mod_name = input("Mod name");
    if let Some(name) = default_name {
        mod_name = mod_name.default_input(name);
    }
    let mod_name: String = mod_name.interact()?;

    let mod_id: String = input("Mod ID")
        .default_input(&crate::mod_ids::to_mod_id(&mod_name))
        .validate_interactively(ModIdValidate)
        .interact()?;

    let package_name: String = input("Package name")
        .interact()?;

    let mut versions: Vec<_> = MinecraftVersion::iter()
        .map(|version| {
            (version, version.version(), "")
        })
        .collect();
    versions.reverse(); // newest first
    let game_version = select("Minecraft version")
        .items(&versions)
        .interact()?;

    let mapping_sets: Vec<_> = MappingSet::iter()
        .map(|set| {
            (set, set.name(), set.description())
        })
        .collect();
    let mapping_set = select("Mappings")
        .items(&mapping_sets)
        .interact()?;

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
        .interact()?;

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
            .interact()?;

        for subproject in chosen_subprojects {
            subproject.apply_to(&mut subprojects);
        }

        if subprojects.fabric && subprojects.quilt {
            subprojects.fabric_likes = confirm("Fabric-like subproject (shared code between Fabric and Quilt)?")
                .initial_value(subprojects.fabric_likes)
                .interact()?;
        }

        dependencies.architectury_api = confirm("Architectury API?")
            .initial_value(dependencies.architectury_api)
            .interact()?;
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

fn get_current_dir() -> Result<PathBuf> {
    std::env::current_dir().wrap_err("Couldn't get current directory")
}

// TODO: Can this be a function ref?
struct ModIdValidate;

impl cliclack::Validate<String> for ModIdValidate {
    type Err = eyre::Report;

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
