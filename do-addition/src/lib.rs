// #[no_mangle]
// pub extern "C" fn add(a: u32, b: u32) -> u32 {
//     a + b
// }

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}