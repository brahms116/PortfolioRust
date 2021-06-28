use crate::controllers::simulation_controller::*;
use crate::game_objects::camera::*;
use crate::options::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

pub struct InputController {
	pub stage: Rc<RefCell<i32>>,
}

impl InputController {
	pub fn new() -> InputController {
		InputController {
			stage: Rc::new(RefCell::new(0)),
		}
	}
	pub fn setup_callbacks(&mut self, simulation_controller: SimulationController) {
		let window = web_sys::window().unwrap();
		let simulation_controller = Rc::new(RefCell::new(simulation_controller));
		{
			let stage = Rc::clone(&self.stage);
			let simulation_controller = Rc::clone(&simulation_controller);
			let closure = Closure::wrap(Box::new(move |event: web_sys::WheelEvent| {
				if event.delta_y() < 0.0 {
					simulation_controller.borrow_mut().zoom_in();
				} else if event.delta_y() > 0.0 {
					simulation_controller.borrow_mut().zoom_out();
				}
			}) as Box<dyn FnMut(_)>);
			window
				.add_event_listener_with_callback("wheel", closure.as_ref().unchecked_ref())
				.unwrap();
			closure.forget();
		}
		{
			let simulation_controller = Rc::clone(&simulation_controller);
			let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
				let simulation_controller = simulation_controller.borrow_mut();
				let mut camera = simulation_controller.camera.borrow_mut();
				let (screen_width, screen_height) = camera.get_screen_dimensions();
				let is_n = event.client_y() < CAMERA_TRIGGER_AREA;
				let is_s = event.client_y() > screen_height - CAMERA_TRIGGER_AREA;
				let is_e = event.client_x() > screen_width - CAMERA_TRIGGER_AREA;
				let is_w = event.client_x() < CAMERA_TRIGGER_AREA;
				camera.set_trigger_area_state(CameraTriggerAreaState {
					is_n,
					is_s,
					is_e,
					is_w,
				});
			}) as Box<dyn FnMut(_)>);
			window
				.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
				.unwrap();
			closure.forget();
		}
	}
}
