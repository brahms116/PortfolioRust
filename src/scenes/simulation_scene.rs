use crate::utils;
pub struct SimulationScene {
	static_entities: Vec<Box<dyn utils::Entity>>,
}
