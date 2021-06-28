use wasm_bindgen::prelude::*;
mod controllers;
mod game_objects;
mod options;
mod scenes;
mod utils;
use controllers::input_controller::*;
use scenes::simulation_scene::*;
#[wasm_bindgen(start)]
pub fn run() {
    let mut input_controller = InputController::new();
    let mut simulation_scene = SimulationScene::new();
    let simulation_controller = simulation_scene.create_controller();
    input_controller.setup_callbacks(simulation_controller);
    simulation_scene.start();
}
