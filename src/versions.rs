// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub mod index;
pub mod version_metadata;

#[cfg(target_family = "wasm")]
pub async fn load_minecraft_version_list(client: std::sync::Arc<reqwest::Client>) -> crate::Result<String> {
    crate::templates::download_relative_text(client, "minecraft_versions.json").await
}
