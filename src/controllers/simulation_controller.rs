use crate::game_objects::camera::Camera;
use std::cell::RefCell;
use std::rc::Rc;

pub struct SimulationController {
	camera: Rc<RefCell<Camera>>,
}
impl<'a> SimulationController {
	pub fn new(camera: Rc<RefCell<Camera>>) -> SimulationController {
		SimulationController { camera }
	}
	pub fn zoom_in(&mut self) {
		let mut camera = self.camera.borrow_mut();
		camera.set_zoom(camera.get_zoom() + 1);
	}
	pub fn zoom_out(&mut self) {
		let camera = self.camera.borrow_mut();
		let current_zoom = camera.get_zoom();
		if current_zoom > 1 {
			camera.set_zoom(current_zoom - 1);
		}
	}
}
