pub mod keys;
pub mod passring;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[wasm_bindgen]
pub fn version() -> String {
    "passring-wasm v".to_owned() + &VERSION.to_string() + " (passring v" + ::passring::VERSION + ")"
}
