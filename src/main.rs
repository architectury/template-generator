#![windows_subsystem = "windows"] // Disable console window on Windows

use templateer::GeneratorApp;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Templateer",
        options,
        Box::new(|cc| Box::new(GeneratorApp::new(cc))),
    )
}
