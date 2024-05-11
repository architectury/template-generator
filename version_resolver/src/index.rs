// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::BTreeMap;

use miette::Result;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::minecraft::MinecraftVersion;

#[derive(Clone, Serialize, Deserialize)]
pub struct VersionIndex {
    #[serde(flatten)]
    pub versions: BTreeMap<MinecraftVersion, Versions>,
}

impl VersionIndex {
    pub async fn resolve(client: &reqwest::Client) -> Result<Self> {
        let mut versions: BTreeMap<MinecraftVersion, Versions> = BTreeMap::new();

        // TODO: Iterate in parallel?
        for game_version in MinecraftVersion::iter() {
            versions.insert(
                game_version,
                Versions::resolve(client, &game_version).await?,
            );
        }

        Ok(Self { versions })
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Versions {
    pub architectury_api: String,
    pub forge: Option<String>,
    pub neoforge: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neoforge_yarn_patch: Option<String>,
}

impl Versions {
    pub async fn resolve(
        client: &reqwest::Client,
        game_version: &MinecraftVersion,
    ) -> Result<Self> {
        let architectury_api = crate::maven::resolve_matching_version(
            &client,
            crate::maven::MavenLibrary::architectury_api(game_version),
            |version| version.starts_with(&format!("{}.", game_version.architectury_api_version())),
        )
        .await?;

        let forge = if let Some(forge_major) = game_version.forge_major_version() {
            Some(crate::maven::resolve_matching_version(
                &client,
                crate::maven::MavenLibrary::forge(),
                |version| {
                    version.starts_with(&format!(
                        "{}-{}.",
                        game_version.version(),
                        forge_major
                    ))
                },
            )
            .await?)
        } else {
            None
        };

        let neoforge = if let Some(major) = game_version.neoforge_major() {
            Some(
                crate::maven::resolve_matching_version(
                    &client,
                    crate::maven::MavenLibrary::neoforge(),
                    |version| version.starts_with(&format!("{}.", major)),
                )
                .await?,
            )
        } else {
            None
        };

        let neoforge_yarn_patch = if let Some(prefix) = game_version.neoforge_yarn_patch_version() {
            Some(
                crate::maven::resolve_matching_version(
                    &client,
                    crate::maven::MavenLibrary::neoforge_yarn_patch(),
                    |version| version.starts_with(&format!("{}+", prefix)),
                )
                .await?,
            )
        } else {
            None
        };

        Ok(Self {
            architectury_api,
            forge,
            neoforge,
            neoforge_yarn_patch,
        })
    }
}
