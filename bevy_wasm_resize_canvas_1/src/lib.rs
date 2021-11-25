mod app;
#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::{*};
#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
pub fn run() {
    app::run();
}
