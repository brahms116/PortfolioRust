use crate::controllers::simulation_controller::SimulationController;
use crate::game_objects::camera::Camera;
use crate::game_objects::triangle::*;
use crate::utils;
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
	static_entities: Vec<Box<dyn utils::Entity>>,
	dynamic_entities: Vec<Box<dyn utils::DynamicEntity>>,
	camera: Rc<RefCell<Camera>>,
}

impl SimulationScene {
	pub fn new() -> SimulationScene {
		SimulationScene {
			static_entities: Vec::new(),
			dynamic_entities: Vec::new(),
			camera: Rc::new(RefCell::new(Camera::new())),
		}
	}
	pub fn create_controller(&mut self) -> SimulationController {
		SimulationController::new(Rc::clone(&self.camera))
	}
	pub fn start(mut self) {
		let f = Rc::new(RefCell::<Option<Closure<dyn FnMut()>>>::new(None));
		let g = Rc::clone(&f);
		let mut frame_count = 0;
		self.generate_benchmark();
		// self.static_entities.push(Box::new(demo));
		*g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
			let mut camera = self.camera.borrow_mut();
			frame_count += 1;
			camera.update_screen_dimensions();
			camera.clear();
			camera.update_pos();
			for i in &mut self.dynamic_entities {
				i.update();
			}
			for i in &mut self.dynamic_entities {
				camera.draw(i);
			}
			for i in &self.static_entities {
				camera.draw(i);
			}
			// self.camera.set_zoom(self.camera.get_zoom() + 1);
			request_animation_frame(f.borrow().as_ref().unwrap());
		}) as Box<dyn FnMut()>));
		request_animation_frame(g.borrow().as_ref().unwrap());
	}
	fn generate_benchmark(&mut self) {
		for _ in 0..1500 {
			let origin_x = rand::random::<f64>() * 500.0 - 250.0;
			let origin_y = rand::random::<f64>() * 300.0 - 150.0;
			let origin = utils::Vec2::new(origin_x, origin_y);
			let generate = || {
				let number = rand::random::<f64>() * 50.0 + 10.0;
				let is_positive = rand::random::<bool>();
				if !is_positive {
					return -number;
				}
				number
			};
			let pt1 = utils::Vec2::new(origin_x + generate(), origin_y + generate());
			let pt2 = utils::Vec2::new(origin_x + generate(), origin_y + generate());
			let tri = Triangle::new(origin, pt1, pt2);
			self.dynamic_entities.push(Box::new(tri));
		}
	}
}
