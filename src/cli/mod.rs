// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use clap::Parser;
use cliclack::{confirm, input, intro, multiselect, select};
use miette::{Context, IntoDiagnostic, Result};
use strum::IntoEnumIterator;
use version_resolver::minecraft::MinecraftVersion;
use std::path::{Path, PathBuf};

use crate::{Dependencies, GeneratorApp, MappingSet, ProjectType, Subprojects};
use crate::filer::native::DirectoryFilerProvider;

#[derive(Parser)]
#[command(version)]
struct Args {
    /// The project path (default: the current directory)
    output: Option<PathBuf>,
}

pub async fn main() -> Result<()> {
    let args = Args::parse();
    // TODO: Support zips
    let dir = if let Some(directory) = args.output {
        directory
    } else {
        std::env::current_dir()
            .into_diagnostic()
            .wrap_err("Couldn't get current directory")?
    };
    let app = prompt(&dir)?;
    let filer_provider = DirectoryFilerProvider(&dir);
    crate::generator::generate(&app, &filer_provider).await
}

fn prompt(dir: &Path) -> Result<GeneratorApp> {
    intro("Architectury Template Generator").into_diagnostic()?;
    
    let mod_name: String = input("Mod name")
        .default_input(dir.file_name().and_then(|s| s.to_str()).unwrap_or(""))
        .interact().into_diagnostic()?;

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
