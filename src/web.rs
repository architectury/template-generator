// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use js_sys::{Array, JsString};
use miette::{miette, Result};
use version_resolver::version_metadata::MinecraftVersionList;
use wasm_bindgen::prelude::*;

use crate::filer;

pub trait ResultExt<T> {
    fn to_miette(self) -> Result<T>;
}

impl<T> ResultExt<T> for std::result::Result<T, wasm_bindgen::JsValue> {
    fn to_miette(self) -> Result<T> {
        self.map_err(|err| miette!("{:?}", err))
    }
}

fn ok_or_display_error<T, E>(result: Result<T, E>) -> Option<T>
where
    E: std::fmt::Display,
{
    match result {
        Ok(value) => Some(value),
        Err(err) => {
            if let Some(window) = web_sys::window() {
                let _ = window.alert_with_message(&err.to_string());
            }
            None
        }
    }
}

#[wasm_bindgen]
pub fn create_state(version_list: JsValue) -> Result<JsValue, JsValue> {
    let version_list: MinecraftVersionList = serde_wasm_bindgen::from_value(version_list)?;
    let app = crate::app::GeneratorApp::new(&version_list);
    Ok(serde_wasm_bindgen::to_value(&app)?)
}

#[wasm_bindgen]
pub async fn list_all_minecraft_versions() -> String {
    use std::sync::Arc;
    let client = reqwest::ClientBuilder::new().build().unwrap();
    crate::game_versions::load_minecraft_version_list(Arc::new(client)).await.unwrap()
}

#[wasm_bindgen]
pub fn to_mod_id(mod_name: &str) -> String {
    crate::mod_ids::to_mod_id(mod_name)
}

#[wasm_bindgen]
pub fn is_valid_mod_id(mod_id: &str) -> bool {
    crate::mod_ids::is_valid_mod_id(mod_id)
}

#[wasm_bindgen]
pub fn validate_mod_id(mod_id: &str) -> Array {
    let result = crate::mod_ids::validate_mod_id(mod_id);
    let array = Array::new();
    match result {
        Ok(_) => {
            array.push(&JsValue::TRUE);
        }
        Err(err) => {
            array.push(&JsValue::FALSE);
            array.push(&JsValue::from(format!("{}", err)));
        }
    }
    array
}

#[wasm_bindgen]
pub async fn generate(state: JsValue, version_list: JsValue) {
    let result = generate_inner(state, version_list).await;
    ok_or_display_error(result.map_err(|err| JsString::from(err)));
}

async fn generate_inner(state: JsValue, version_list: JsValue) -> Result<(), JsValue> {
    let app: crate::app::GeneratorApp = serde_wasm_bindgen::from_value(state)?;
    let version_list: MinecraftVersionList = serde_wasm_bindgen::from_value(version_list)?;
    crate::app::generator::generate(&app, &version_list, &filer::ZipFilerProvider(filer::web::ZipSaveDialog))
        .await
        .map_err(|err| JsValue::from(format!("{}", err)))
}

#[wasm_bindgen]
pub fn supports_neoforge(game_version: JsValue) -> Result<bool, JsValue> {
    let game_version: version_resolver::version_metadata::MinecraftVersion = serde_wasm_bindgen::from_value(game_version)?;
    Ok(game_version.neoforge.is_some())
}

#[wasm_bindgen]
pub fn supports_forge(game_version: JsValue) -> Result<bool, JsValue> {
    let game_version: version_resolver::version_metadata::MinecraftVersion = serde_wasm_bindgen::from_value(game_version)?;
    Ok(game_version.forge.is_some())
}

#[wasm_bindgen]
pub fn arch_api_supports_forge(game_version: JsValue) -> Result<bool, JsValue> {
    let game_version: version_resolver::version_metadata::MinecraftVersion = serde_wasm_bindgen::from_value(game_version)?;
    if let Some(forge) = game_version.forge {
        Ok(forge.major_version < 50)
    } else {
        Ok(false)
    }
}
