use crate::game_objects::camera::*;
use crate::options::*;
use crate::scenes::simulation_scene::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct SimulationController {
	camera: Camera,
	scene: SimulationScene,
}
impl SimulationController {
	pub fn new() -> SimulationController {
		SimulationController {
			camera: Camera::new(),
			scene: SimulationScene::new(),
		}
	}
	pub fn update(&mut self) {
		self.camera.update();
		let entities = self.scene.get_entities();
		let dynamic_entities = self.scene.get_dynamic_entities();
		for i in entities {
			self.camera.draw(i);
		}
		for i in dynamic_entities {
			self.camera.draw(i);
		}
	}
	pub fn handle_zoom(&mut self, delta_y: i32) {
		let current_zoom = self.camera.get_zoom();
		if delta_y > 0 {
			if current_zoom > 1.0 {
				self.camera.set_zoom(current_zoom * 0.8);
			}
		} else if delta_y < 0 {
			self.camera.set_zoom(current_zoom * 1.2);
		}
	}
	pub fn handle_mouse_move(&mut self, client_x: i32, client_y: i32) {
		let (screen_width, screen_height) = self.camera.get_screen_dimensions();
		let is_n = client_y < CAMERA_TRIGGER_AREA;
		let is_s = client_y > screen_height - CAMERA_TRIGGER_AREA;
		let is_e = client_x > screen_width - CAMERA_TRIGGER_AREA;
		let is_w = client_x < CAMERA_TRIGGER_AREA;
		self.camera.set_trigger_area_state(CameraTriggerAreaState {
			is_n,
			is_s,
			is_e,
			is_w,
		});
	}
}
