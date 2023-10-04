pub mod app;
pub mod async_support;
pub mod mod_ids;
pub mod requests;
pub mod tap;
pub mod templates;
#[cfg(target_arch = "wasm32")]
pub mod web;
pub mod xml;

pub use app::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn main() {
    let options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "app",
                options,
                Box::new(|cc| Box::new(GeneratorApp::new(cc))),
            )
            .await
            .expect("Failed to start eframe")
    });
}
