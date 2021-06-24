use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
mod game_objects;
mod utils;

#[wasm_bindgen(start)]
pub fn run() {
    let window = web_sys::window().unwrap();
}
