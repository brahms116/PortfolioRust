use crate::controllers::simulation_controller::SimulationController;
use crate::game_objects::camera::Camera;
use crate::game_objects::demo_struct::*;
use crate::utils;
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
	camera: Rc<RefCell<Camera>>,
}

impl SimulationScene {
	pub fn new() -> SimulationScene {
		SimulationScene {
			static_entities: Vec::new(),
			camera: Rc::new(RefCell::new(Camera::new())),
		}
	}
	pub fn create_controller(&mut self) -> SimulationController {
		SimulationController::new(Rc::clone(&self.camera))
	}
	pub fn start(mut self) {
		let f = Rc::new(RefCell::<Option<Closure<dyn FnMut()>>>::new(None));
		let g = Rc::clone(&f);
		let demo = DemoStruct::new(-1, -1);
		let mut frame_count = 0;
		let camera = self.camera.borrow_mut();
		self.static_entities.push(Box::new(demo));
		*g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
			frame_count += 1;
			camera.update_screen_dimensions();
			camera.clear();
			for i in &self.static_entities {
				camera.draw(&i);
			}
			// self.camera.set_zoom(self.camera.get_zoom() + 1);
			request_animation_frame(f.borrow().as_ref().unwrap());
		}) as Box<dyn FnMut()>));
		request_animation_frame(g.borrow().as_ref().unwrap());
	}
}
