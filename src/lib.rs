extern crate web_sys;

use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub fn get_inventory() {
    //console::log_1(&"Inv".into());
}

#[wasm_bindgen]
pub fn execute_command(command: String) {
    console::log_1(&command.into())
}

#[wasm_bindgen]
pub fn get_item_info() {
    // console::log_1(&"Item".into());
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> String {
    console::log_1(&"Inv".into());
    "Avhgj".into()
}
