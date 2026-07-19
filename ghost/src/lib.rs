use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn create_identity() -> String {
    "Identity Created!".to_string()
}

#[wasm_bindgen]
pub fn add_friend() -> String {
    "Friend Added!".to_string()
}