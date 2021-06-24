use crate::game_objects::camera::Camera;
use crate::utils;
pub struct SimulationScene {
	static_entities: Vec<Box<dyn utils::Entity>>,
	camera: Camera,
}

impl SimulationScene {
	pub fn new() -> SimulationScene {}
}
