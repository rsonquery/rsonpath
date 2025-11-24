use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    color_eyre::install().expect("a");

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document");
    let canvas = document
        .get_element_by_id("rsonquery_web")
        .expect("Canvas with ID `rsonquery_web` not found")
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    wasm_bindgen_futures::spawn_local(async move {
        let runner = eframe::WebRunner::new();
        runner
            .start(
                canvas,
                eframe::WebOptions::default(),
                Box::new(|_cc| Ok(Box::new(rsonpath_website::WebsiteGui::default()))),
            )
            .await
            .expect("Failed to start eframe WebRunner");

        rsonpath_website::register_file_handler();
    });

    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn main() {
    start().expect("TODO: panic message");
}
