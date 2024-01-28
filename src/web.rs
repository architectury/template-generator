use js_sys::Array;
use miette::{miette, Result};
use strum::IntoEnumIterator;
use wasm_bindgen::prelude::*;

pub trait ResultExt<T> {
    fn to_miette(self) -> Result<T>;
}

impl<T> ResultExt<T> for std::result::Result<T, wasm_bindgen::JsValue> {
    fn to_miette(self) -> Result<T> {
        self.map_err(|err| miette!("{:?}", err))
    }
}

#[wasm_bindgen]
pub fn create_state() -> Result<JsValue, JsValue> {
    let app = crate::app::GeneratorApp::new();
    Ok(serde_wasm_bindgen::to_value(&app)?)
}

#[wasm_bindgen]
pub fn list_all_minecraft_versions() -> Array {
    crate::app::versions::MinecraftVersion::iter()
        .map(|version| version.version())
        .map(|x| JsValue::from_str(x))
        .collect()
}
