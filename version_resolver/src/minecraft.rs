// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use miette::miette;
use serde::{Deserialize, Serialize};

#[deprecated]
#[derive(Clone)]
pub struct MinecraftVersion {
    inner: crate::version_metadata::MinecraftVersion,
}

impl MinecraftVersion {
    pub fn new(inner: crate::version_metadata::MinecraftVersion) -> Self {
        Self {
            inner
        }
    }

    pub fn latest<'a>(map: &crate::version_metadata::MinecraftVersionMap<'a>) -> Self {
        Self::new(map.latest_version().clone())
    }

    pub fn version(&self) -> &str {
        &self.inner.version
    }

    pub fn java_version(&self) -> JavaVersion {
        self.inner.java_version.try_into().unwrap()
    }

    pub fn architectury_package(&self) -> &str {
        self.inner.architectury.package.as_str()
    }

    pub fn architectury_maven_group(&self) -> &str {
        self.inner.architectury.maven_group.as_str()
    }

    pub fn fabric_api_branch(&self) -> &str {
        &self.inner
            .fabric
            .fabric_api_branch
            .as_ref()
            .unwrap_or(&self.inner.version)
            .as_ref()
    }

    pub fn fabric_api_mod_id(&self) -> &str {
        // See https://github.com/architectury/template-generator/issues/18 and
        // https://github.com/FabricMC/fabric/commit/f60060dfe365941c3b7514d1e53cc7e09dbd671e.
        self.inner.fabric.fabric_api_mod_id.as_str()
    }

    pub fn forge_major_version(&self) -> Option<u32> {
        match &self.inner.forge {
            Some(forge) => Some(forge.major_version),
            None => None,
        }
    }

    pub fn architectury_api_version(&self) -> &str {
        self.inner.architectury.api_version.as_str()
    }

    pub fn neoforge_loader_major(&self) -> Option<&str> {
        match &self.inner.neoforge {
            Some(neoforge) => Some(neoforge.loader_major_version.as_str()),
            None => None,
        }
    }

    pub fn neoforge_major(&self) -> Option<&str> {
        match &self.inner.neoforge {
            Some(neoforge) => Some(neoforge.neoforge_major_version.as_str()),
            None => None,
        }
    }

    pub fn neoforge_yarn_patch_version(&self) -> Option<&str> {
        match &self.inner.neoforge {
            Some(neoforge) => neoforge.yarn_patch_version.as_ref().map(|x| x.as_str()),
            None => None,
        }
    }

    pub fn forge_pack_version(&self) -> Option<u32> {
        match &self.inner.forge {
            Some(forge) => Some(forge.pack_version),
            None => None,
        }
    }

    pub fn forge_server_pack_version(&self) -> Option<(&str, &str)> {
        match &self.inner.forge {
            Some(forge) => match &forge.server_pack_version {
                Some((key, value)) => Some((key.as_str(), value.as_str())),
                None => None,
            },
            None => None,
        }
    }
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Clone, Copy)]
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
    type Error = miette::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value >= 9 {
            Ok(Self::Java9OrNewer(value))
        } else if value == 8 {
            Ok(Self::Java8)
        } else {
            Err(miette!("Java version {} not supported", value))
        }
    }
}
