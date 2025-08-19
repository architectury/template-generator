// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub mod index;
mod version_metadata;
pub use version_metadata::*;

pub const LOOM_VERSION: &'static str = "1.11-SNAPSHOT";
pub const PLUGIN_VERSION: &'static str = "3.4-SNAPSHOT";

#[cfg(target_family = "wasm")]
pub async fn load_minecraft_version_list(client: std::sync::Arc<reqwest::Client>) -> crate::Result<String> {
    crate::templates::download_relative_text(client, "minecraft_versions.json").await
}
