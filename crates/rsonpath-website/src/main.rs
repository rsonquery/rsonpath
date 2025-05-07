use crate::lib::WebsiteGui;
use eframe::NativeOptions;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

mod lib;

// #[cfg(target_arch = "wasm32")]
// #[wasm_bindgen(start)]
// pub fn start() -> Result<(), JsValue> {
//
//     console_error_panic_hook::set_once();
//     let web_options = eframe::WebOptions::default();
//
//
//     //TODO: Write html file with <canvas_id="rsonquery_web"
//     wasm_bindgen_futures::spawn_local(async {
//         eframe::WebRunner::new()
//             .start("rsonquery_web", web_options, Box::new(|_cc| Box::new(WebsiteGui::default())))
//             .await
//             .expect("Failed to start eframe");
//     });
//
//     Ok(())
// }
//
// #[cfg(target_arch="wasm32")]
// fn main() {
//     start().expect("TODO: panic message");
// }

//TODO: When you change to wasm main, uncomment the stuff in config.toml
fn main() -> eframe::Result<()> {
    let options = NativeOptions::default();
    eframe::run_native(
        "RsonQuery App",
        options,
        Box::new(|_cc| Ok(Box::new(WebsiteGui::default()))),
    )
}
