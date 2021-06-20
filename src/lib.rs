use wasm_bindgen::prelude::*;
use web_sys::console;


#[wasm_bindgen(start)]
pub fn run(){
    console::log_1(&"helloworld".into())
}



