use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet2(name: &str) -> String {
    format!("Hello, {}!", name)
}