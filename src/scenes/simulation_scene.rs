use crate::controllers::simulation_controller::SimulationController;
use crate::game_objects::camera::Camera;
use crate::game_objects::triangle::*;
use crate::utils::*;
use rand;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
	web_sys::window()
		.unwrap()
		.request_animation_frame(f.as_ref().unchecked_ref())
		.unwrap();
}
pub struct SimulationScene {
	static_entities: Vec<Box<dyn Entity>>,
	dynamic_entities: Vec<Box<dyn DynamicEntity>>,
}

impl SimulationScene {
	pub fn new() -> SimulationScene {
		let mut scene = SimulationScene {
			static_entities: Vec::new(),
			dynamic_entities: Vec::new(),
		};
		scene.generate_benchmark();
		scene
	}
	pub fn get_entities(&self) -> &Vec<Box<dyn Entity>> {
		&self.static_entities
	}
	pub fn get_dynamic_entities(&self) -> &Vec<Box<dyn DynamicEntity>> {
		&self.dynamic_entities
	}
	fn generate_benchmark(&mut self) {
		for _ in 0..10 {
			let origin_x = rand::random::<f64>() * 500.0 - 250.0;
			let origin_y = rand::random::<f64>() * 300.0 - 150.0;
			let origin = Vec2::new(origin_x, origin_y);
			let generate = || {
				let number = rand::random::<f64>() * 50.0 + 10.0;
				let is_positive = rand::random::<bool>();
				if !is_positive {
					return -number;
				}
				number
			};
			let pt1 = Vec2::new(origin_x + generate(), origin_y + generate());
			let pt2 = Vec2::new(origin_x + generate(), origin_y + generate());
			let tri = Triangle::new(origin, pt1, pt2);
			self.dynamic_entities.push(Box::new(tri));
		}
	}
}
