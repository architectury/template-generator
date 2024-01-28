use serde::{Deserialize, Serialize};

mod generator;
pub mod versions;

pub const SUBHEADING_STYLE: &'static str = "subheading";

#[derive(Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProjectType {
    #[default]
    Multiplatform,
    NeoForge,
    Forge,
}

#[derive(Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum MappingSet {
    #[default]
    Mojang,
    Yarn,
}

impl MappingSet {
    fn description(&self) -> &'static str {
        match self {
            Self::Mojang => "The official obfuscation maps published by Mojang.",
            Self::Yarn => "A libre mapping set maintained by FabricMC.",
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Subprojects {
    fabric: bool,
    forge: bool,
    neoforge: bool,
    quilt: bool,
    fabric_likes: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Dependencies {
    architectury_api: bool,
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
    mod_name: String,
    mod_id: String,
    package_name: String,
    game_version: versions::MinecraftVersion,
    project_type: ProjectType,
    subprojects: Subprojects,
    mapping_set: MappingSet,
    dependencies: Dependencies,
}

impl GeneratorApp {
    pub fn new() -> Self {
        Self {
            mod_name: "Example Mod".to_owned(),
            mod_id: String::new(),
            package_name: "com.example".to_owned(),
            game_version: versions::MinecraftVersion::latest(),
            project_type: Default::default(),
            subprojects: Default::default(),
            mapping_set: Default::default(),
            dependencies: Default::default(),
        }
    }
}
