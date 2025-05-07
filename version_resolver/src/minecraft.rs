// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use miette::miette;
use serde::{Deserialize, Serialize};

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
