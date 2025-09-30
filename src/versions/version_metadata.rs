// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::err;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct MinecraftVersionMap<'a> {
    versions_by_index: HashMap<&'a str, &'a MinecraftVersion>,
    latest_version: &'a MinecraftVersion,
}

impl<'a> MinecraftVersionMap<'a> {
    pub fn new(version_list: &'a MinecraftVersionList) -> Self {
        let mut versions_by_index: HashMap<&'a str, &'a MinecraftVersion> = HashMap::new();

        for version in &version_list.versions {
            versions_by_index.insert(&version.version, &version);
        }

        let latest_version = versions_by_index[&version_list.latest_version.as_ref()];

        Self {
            versions_by_index,
            latest_version,
        }
    }

    pub fn latest_version(&self) -> &'a MinecraftVersion {
        self.latest_version
    }

    pub fn get(&self, key: &str) -> &'a MinecraftVersion {
        self.versions_by_index[key]
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct MinecraftVersionList {
    pub latest_version: String,
    pub versions: Vec<MinecraftVersion>,
}

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct MinecraftVersion {
    pub version: String,
    pub java_version: u32,
    pub architectury: ArchitecturyMetadata,
    #[serde(default)]
    pub fabric: FabricMetadata,
    pub forge: Option<ForgeMetadata>,
    pub neoforge: Option<NeoForgeMetadata>,
}

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct ArchitecturyMetadata {
    pub api_version: Option<String>,
    #[serde(default = "default_architectury_package")]
    pub package: String,
    #[serde(default = "default_architectury_package")]
    pub maven_group: String,
}

fn default_architectury_package() -> String {
    "dev.architectury".to_owned()
}

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct FabricMetadata {
    pub fabric_api_branch: Option<String>,
    #[serde(default = "default_fabric_api_mod_id")]
    pub fabric_api_mod_id: String,
}

impl Default for FabricMetadata {
    fn default() -> Self {
        Self {
            fabric_api_branch: None,
            fabric_api_mod_id: default_fabric_api_mod_id()
        }
    }
}

fn default_fabric_api_mod_id() -> String {
    "fabric-api".to_owned()
}

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct ForgeMetadata {
    pub major_version: u32,
    pub pack_version: u32,
    pub server_pack_version: Option<(String, String)>,
}

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct NeoForgeMetadata {
    pub loader_major_version: String,
    pub neoforge_major_version: String,
    pub yarn_patch_version: Option<String>,
}

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum JavaVersion {
    Java8,
    Java9OrNewer(u32),
}

impl JavaVersion {
    pub fn gradle_java_version(&self) -> String {
        match self {
            Self::Java8 => "1_8".to_owned(),
            Self::Java9OrNewer(version) => version.to_string(),
        }
    }

    pub fn java_major_version(&self) -> u32 {
        match self {
            Self::Java8 => 8,
            Self::Java9OrNewer(version) => *version,
        }
    }

    pub fn mixin_compat_level(&self) -> String {
        match self {
            Self::Java8 => "JAVA_8".to_owned(),
            Self::Java9OrNewer(version) => format!("JAVA_{}", version),
        }
    }
}

impl TryFrom<u32> for JavaVersion {
    type Error = crate::result::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value >= 9 {
            Ok(Self::Java9OrNewer(value))
        } else if value == 8 {
            Ok(Self::Java8)
        } else {
            Err(err!("Java version {} not supported", value))
        }
    }
}
