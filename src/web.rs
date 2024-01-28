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
pub fn generate(state: JsValue) -> Result<(), JsValue> {
    let app: crate::app::GeneratorApp = serde_wasm_bindgen::from_value(state)?;
    crate::app::generator::generate(&app).map_err(|_| JsValue::from_str("fail"))
}
