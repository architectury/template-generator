// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub const LOOM_VERSION: &'static str = "1.5-SNAPSHOT";
pub const PLUGIN_VERSION: &'static str = "3.4-SNAPSHOT";

use miette::Result;
use reqwest::Client;
use version_resolver::index::Versions;
use version_resolver::minecraft::MinecraftVersion;

#[cfg(target_arch = "wasm32")]
pub async fn get_version_index(client: std::sync::Arc<Client>, game_version: &MinecraftVersion) -> Result<Versions> {
    use miette::{IntoDiagnostic, miette};
    use version_resolver::index::VersionIndex;

    let json = crate::templates::download_relative_file(client, "version_index.json").await?;
    let index: VersionIndex = serde_json::from_str(&json).into_diagnostic()?;
    index.versions
        .get(game_version)
        .ok_or_else(|| miette!("Could not find version index for version {}", game_version.version()))
        .cloned()
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn get_version_index(client: std::sync::Arc<Client>, game_version: &MinecraftVersion) -> Result<Versions> {
    Versions::resolve(&client, game_version).await
}
