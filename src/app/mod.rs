// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use strum::EnumIter;

pub mod generator;
pub mod versions;

pub const SUBHEADING_STYLE: &'static str = "subheading";

#[derive(Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProjectType {
    #[default]
    Multiplatform,
    NeoForge,
    Forge,
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, EnumIter)]
pub enum MappingSet {
    #[default]
    Mojang,
    Yarn,
}

impl MappingSet {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Mojang => "Official Mojang mappings",
            Self::Yarn => "Yarn",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Mojang => "The official obfuscation maps published by Mojang.",
            Self::Yarn => "A libre mapping set maintained by FabricMC.",
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Subprojects {
    pub fabric: bool,
    pub forge: bool,
    pub neoforge: bool,
    pub quilt: bool,
    pub fabric_likes: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Dependencies {
    pub architectury_api: bool,
}

impl Default for Dependencies {
    fn default() -> Self {
        Self {
            architectury_api: true,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GeneratorApp {
    pub mod_name: String,
    pub mod_id: String,
    pub package_name: String,
    pub game_version: version_resolver::minecraft::MinecraftVersion,
    pub project_type: ProjectType,
    pub subprojects: Subprojects,
    pub mapping_set: MappingSet,
    pub dependencies: Dependencies,
}

impl GeneratorApp {
    pub fn new() -> Self {
        Self {
            mod_name: "Example Mod".to_owned(),
            mod_id: String::new(),
            package_name: "com.example".to_owned(),
            game_version: version_resolver::minecraft::MinecraftVersion::latest(),
            project_type: Default::default(),
            subprojects: Default::default(),
            mapping_set: Default::default(),
            dependencies: Default::default(),
        }
    }
}
