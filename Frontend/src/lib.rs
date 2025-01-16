use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn add_numbers(a: i32, b: i32) -> i32 {
    return a + b;
}

#[wasm_bindgen]
pub fn multiply_numbers(a: i32, b: i32) -> i32 {
    return a * b;
}