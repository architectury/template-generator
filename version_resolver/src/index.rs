// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::HashMap;

use miette::Result;
use serde::{Deserialize, Serialize};

use crate::version_metadata::{MinecraftVersion, MinecraftVersionList};

#[derive(Clone, Serialize, Deserialize)]
pub struct VersionIndex {
    #[serde(flatten)]
    pub versions: HashMap<String, Versions>,
}

impl VersionIndex {
    pub async fn resolve(client: &reqwest::Client, version_list: &MinecraftVersionList) -> Result<Self> {
        let mut versions: HashMap<String, Versions> = HashMap::new();

        // TODO: Iterate in parallel?
        for game_version in version_list.versions.iter() {
            versions.insert(
                game_version.version.to_owned(),
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
            |version| version.starts_with(&format!("{}.", game_version.architectury.api_version)),
        )
        .await?;

        let forge = if let Some(forge) = &game_version.forge {
            Some(crate::maven::resolve_matching_version(
                &client,
                crate::maven::MavenLibrary::forge(),
                |version| {
                    version.starts_with(&format!(
                        "{}-{}.",
                        game_version.version,
                        forge.major_version
                    ))
                },
            )
            .await?)
        } else {
            None
        };

        let neoforge = if let Some(neoforge) = &game_version.neoforge {
            Some(
                crate::maven::resolve_matching_version(
                    &client,
                    crate::maven::MavenLibrary::neoforge(),
                    |version| version.starts_with(&format!("{}.", neoforge.neoforge_major_version)),
                )
                .await?,
            )
        } else {
            None
        };

        let neoforge_yarn_patch = match &game_version.neoforge {
            Some(neoforge) => match &neoforge.yarn_patch_version {
                Some(prefix) => Some(
                    crate::maven::resolve_matching_version(
                        &client,
                        crate::maven::MavenLibrary::neoforge_yarn_patch(),
                        |version| version.starts_with(&format!("{}+", prefix)),
                    )
                    .await?,
                ),
                None => None,
            },
            None => None,
        };

        Ok(Self {
            architectury_api,
            forge,
            neoforge,
            neoforge_yarn_patch,
        })
    }
}
