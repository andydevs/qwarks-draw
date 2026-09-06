//! Jarsdraw demo — WASM entry point for the browser demo application.
//!
//! Each module is a standalone demo exposing its own `#[wasm_bindgen]` canvas type to
//! JavaScript (e.g. `CircleCanvas`), free to draw and behave however it likes.
use wasm_bindgen::prelude::*;

mod circle;
mod line;
mod rect;
mod star;

/// Initialize console panic hook
#[wasm_bindgen(start)]
fn main() {
    console_error_panic_hook::set_once();
}
