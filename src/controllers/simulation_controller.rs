use crate::controllers::ui_controller::*;
use crate::game_objects::camera::*;
use crate::options::*;
use crate::scenes::simulation_scene::*;

pub struct SimulationController {
	camera: Camera,
	scene: SimulationScene,
	ui_controller: UiController,
}
impl SimulationController {
	pub fn new() -> SimulationController {
		SimulationController {
			camera: Camera::new(),
			scene: SimulationScene::new(),
			ui_controller: UiController::new(),
		}
	}
	pub fn update(&mut self) {
		self.camera.update();
		let draw_data = self.scene.get_entities_draw_data();
		for data in draw_data {
			self.camera.draw(data);
		}
		self.ui_controller.update();
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
