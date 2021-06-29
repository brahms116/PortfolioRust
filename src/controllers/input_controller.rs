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
			let simulation_controller = Rc::clone(&simulation_controller);
			let closure = Closure::wrap(Box::new(move |event: web_sys::WheelEvent| {
				simulation_controller
					.borrow_mut()
					.handle_zoom(event.delta_y() as i32);
			}) as Box<dyn FnMut(_)>);
			window
				.add_event_listener_with_callback("wheel", closure.as_ref().unchecked_ref())
				.unwrap();
			closure.forget();
		}
		{
			let simulation_controller = Rc::clone(&simulation_controller);
			let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
				let mut simulation_controller = simulation_controller.borrow_mut();
				simulation_controller.handle_mouse_move(event.client_x(), event.client_y());
			}) as Box<dyn FnMut(_)>);
			window
				.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
				.unwrap();
			closure.forget();
		}
	}
}
