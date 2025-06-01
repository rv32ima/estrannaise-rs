use wasm_bindgen::prelude::*;
use log::Level;

mod models;

#[wasm_bindgen(start)]
fn start() {
    console_log::init_with_level(Level::Debug).unwrap();
}
