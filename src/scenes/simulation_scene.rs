use crate::game_objects::car::*;
use crate::utils::direction::*;
use crate::utils::entity::*;
use crate::utils::vector_2::*;

pub struct SimulationScene {
	cars: Vec<Car>,
}

impl SimulationScene {
	pub fn new() -> SimulationScene {
		let mut scene = SimulationScene { cars: Vec::new() };
		scene.cars.push(Car::new(
			false,
			String::from("#d96666"),
			Vec2::new(5.0, 0.0),
			Direction::NE,
		));
		scene
	}
	pub fn get_entities_draw_data(&self) -> Vec<EntityDrawData> {
		let mut draw_data = Vec::<EntityDrawData>::new();
		for car in &self.cars {
			draw_data.push(car.get_draw_data());
		}
		draw_data
	}
}
