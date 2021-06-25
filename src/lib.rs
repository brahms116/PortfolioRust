use wasm_bindgen::prelude::*;
mod controllers;
mod game_objects;
mod scenes;
mod utils;
use scenes::simulation_scene::*;
#[wasm_bindgen(start)]
pub fn run() {
    let simulation_scene = SimulationScene::new();
    simulation_scene.start();
}
