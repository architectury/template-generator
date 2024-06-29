// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord, Serialize, Deserialize, EnumIter)]
pub enum MinecraftVersion {
    #[serde(rename = "1.16.5")]
    Minecraft1_16_5,
    #[serde(rename = "1.17.1")]
    Minecraft1_17_1,
    #[serde(rename = "1.18.1")]
    Minecraft1_18_1,
    #[serde(rename = "1.18.2")]
    Minecraft1_18_2,
    #[serde(rename = "1.19")]
    Minecraft1_19,
    #[serde(rename = "1.19.1")]
    Minecraft1_19_1,
    #[serde(rename = "1.19.2")]
    Minecraft1_19_2,
    #[serde(rename = "1.19.3")]
    Minecraft1_19_3,
    #[serde(rename = "1.19.4")]
    Minecraft1_19_4,
    #[serde(rename = "1.20.1")]
    Minecraft1_20_1,
    #[serde(rename = "1.20.2")]
    Minecraft1_20_2,
    #[serde(rename = "1.20.4")]
    Minecraft1_20_4,
    #[serde(rename = "1.20.5")]
    Minecraft1_20_5,
    #[serde(rename = "1.20.6")]
    Minecraft1_20_6,
    #[serde(rename = "1.21")]
    Minecraft1_21,
}

impl MinecraftVersion {
    pub fn latest() -> Self {
        use strum::IntoEnumIterator;
        Self::iter().last().unwrap()
    }

    pub fn version(&self) -> &'static str {
        match self {
            Self::Minecraft1_16_5 => "1.16.5",
            Self::Minecraft1_17_1 => "1.17.1",
            Self::Minecraft1_18_1 => "1.18.1",
            Self::Minecraft1_18_2 => "1.18.2",
            Self::Minecraft1_19 => "1.19",
            Self::Minecraft1_19_1 => "1.19.1",
            Self::Minecraft1_19_2 => "1.19.2",
            Self::Minecraft1_19_3 => "1.19.3",
            Self::Minecraft1_19_4 => "1.19.4",
            Self::Minecraft1_20_1 => "1.20.1",
            Self::Minecraft1_20_2 => "1.20.2",
            Self::Minecraft1_20_4 => "1.20.4",
            Self::Minecraft1_20_5 => "1.20.5",
            Self::Minecraft1_20_6 => "1.20.6",
            Self::Minecraft1_21 => "1.21",
        }
    }

    pub fn java_version(&self) -> JavaVersion {
        if self == &Self::Minecraft1_16_5 {
            JavaVersion::Java8
        } else if self == &Self::Minecraft1_17_1 {
            JavaVersion::Java9OrNewer(16)
        } else if &Self::Minecraft1_18_1 <= self && self <= &Self::Minecraft1_20_4 {
            JavaVersion::Java9OrNewer(17)
        } else {
            JavaVersion::Java9OrNewer(21)
        }
    }

    pub fn architectury_package(&self) -> &'static str {
        match self {
            MinecraftVersion::Minecraft1_16_5 => "me.shedaniel.architectury",
            _ => "dev.architectury",
        }
    }

    pub fn architectury_maven_group(&self) -> &'static str {
        match self {
            MinecraftVersion::Minecraft1_16_5 => "me.shedaniel",
            _ => "dev.architectury",
        }
    }

    pub fn fabric_api_branch(&self) -> &'static str {
        match self {
            Self::Minecraft1_16_5 => "1.16",
            Self::Minecraft1_17_1 => "1.17",
            Self::Minecraft1_18_1 => "1.18",
            _ => self.version(),
        }
    }

    pub fn forge_major_version(&self) -> Option<&'static str> {
        match self {
            Self::Minecraft1_16_5 => Some("36"),
            Self::Minecraft1_17_1 => Some("37"),
            Self::Minecraft1_18_1 => Some("39"),
            Self::Minecraft1_18_2 => Some("40"),
            Self::Minecraft1_19 => Some("41"),
            Self::Minecraft1_19_1 => Some("42"),
            Self::Minecraft1_19_2 => Some("43"),
            Self::Minecraft1_19_3 => Some("44"),
            Self::Minecraft1_19_4 => Some("45"),
            Self::Minecraft1_20_1 => Some("47"),
            Self::Minecraft1_20_2 => Some("48"),
            Self::Minecraft1_20_4 => Some("49"),
            Self::Minecraft1_20_5 => None,
            Self::Minecraft1_20_6 => None,
            Self::Minecraft1_21 => None,
        }
    }

    pub fn architectury_api_version(&self) -> &'static str {
        match self {
            Self::Minecraft1_16_5 => "1",
            Self::Minecraft1_17_1 => "2",
            Self::Minecraft1_18_1 => "3",
            Self::Minecraft1_18_2 => "4",
            Self::Minecraft1_19 => "5",
            Self::Minecraft1_19_1 => "6.3",
            Self::Minecraft1_19_2 => "6",
            Self::Minecraft1_19_3 => "7",
            Self::Minecraft1_19_4 => "8",
            Self::Minecraft1_20_1 => "9",
            Self::Minecraft1_20_2 => "10",
            Self::Minecraft1_20_4 => "11",
            Self::Minecraft1_20_5 => "12",
            Self::Minecraft1_20_6 => "12",
            Self::Minecraft1_21 => "13",
        }
    }

    pub fn neoforge_loader_major(&self) -> Option<&'static str> {
        match self {
            Self::Minecraft1_20_4 => Some("2"),
            Self::Minecraft1_20_5 => Some("2"),
            Self::Minecraft1_20_6 => Some("2"),
            Self::Minecraft1_21 => Some("4"),
            _ => None,
        }
    }

    pub fn neoforge_major(&self) -> Option<&'static str> {
        match self {
            Self::Minecraft1_20_4 => Some("20.4"),
            Self::Minecraft1_20_5 => Some("20.5"),
            Self::Minecraft1_20_6 => Some("20.6"),
            Self::Minecraft1_21 => Some("21.0"),
            _ => None,
        }
    }

    pub fn neoforge_yarn_patch_version(&self) -> Option<&'static str> {
        match self {
            Self::Minecraft1_20_5 => Some("1.20.5"),
            Self::Minecraft1_20_6 => Some("1.20.6"),
            Self::Minecraft1_21 => Some("1.21"),
            _ => None,
        }
    }

    pub fn forge_pack_version(&self) -> Option<&'static str> {
        match self {
            Self::Minecraft1_16_5 => Some("6"),
            Self::Minecraft1_17_1 => Some("7"),
            Self::Minecraft1_18_1 => Some("8"),
            Self::Minecraft1_18_2 => Some("8"),
            Self::Minecraft1_19 => Some("9"),
            Self::Minecraft1_19_1 => Some("9"),
            Self::Minecraft1_19_2 => Some("9"),
            Self::Minecraft1_19_3 => Some("12"),
            Self::Minecraft1_19_4 => Some("13"),
            Self::Minecraft1_20_1 => Some("15"),
            Self::Minecraft1_20_2 => Some("18"),
            Self::Minecraft1_20_4 => Some("22"),
            Self::Minecraft1_20_5 => None,
            Self::Minecraft1_20_6 => None,
            Self::Minecraft1_21 => None,
        }
    }

    pub fn forge_server_pack_version(&self) -> Option<(&'static str, &'static str)> {
        match self {
            Self::Minecraft1_18_2 => Some(("forge:data_pack_format", "9")),
            Self::Minecraft1_19 => Some(("forge:data_pack_format", "10")),
            Self::Minecraft1_19_1 => Some(("forge:data_pack_format", "10")),
            Self::Minecraft1_19_2 => Some(("forge:data_pack_format", "10")),
            Self::Minecraft1_19_3 => Some(("forge:data_pack_format", "10")),
            Self::Minecraft1_19_4 => Some(("forge:server_data_pack_format", "11")),
            Self::Minecraft1_20_1 => Some(("forge:server_data_pack_format", "15")),
            _ => None,
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
