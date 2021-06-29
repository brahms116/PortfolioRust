use crate::game_objects::camera::*;
use crate::options::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct SimulationController {
	pub camera: Rc<RefCell<Camera>>,
}
impl SimulationController {
	pub fn new(camera: Rc<RefCell<Camera>>) -> SimulationController {
		SimulationController { camera }
	}
	pub fn handle_zoom(&mut self, delta_y: i32) {
		let mut camera = self.camera.borrow_mut();
		let current_zoom = camera.get_zoom();
		if delta_y > 0 {
			if current_zoom > 1 {
				camera.set_zoom(current_zoom - 1);
			}
		} else if delta_y < 0 {
			camera.set_zoom(current_zoom + 1);
		}
	}
	pub fn handle_mouse_move(&mut self, client_x: i32, client_y: i32) {
		let mut camera = self.camera.borrow_mut();
		let (screen_width, screen_height) = camera.get_screen_dimensions();
		let is_n = client_y < CAMERA_TRIGGER_AREA;
		let is_s = client_y > screen_height - CAMERA_TRIGGER_AREA;
		let is_e = client_x > screen_width - CAMERA_TRIGGER_AREA;
		let is_w = client_x < CAMERA_TRIGGER_AREA;
		camera.set_trigger_area_state(CameraTriggerAreaState {
			is_n,
			is_s,
			is_e,
			is_w,
		});
	}
}
