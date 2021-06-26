use crate::controllers::simulation_controller::*;
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
				console::log_1(&format!("{}", event.delta_y()).into());
				if event.delta_y() > 0.0 {
					simulation_controller.borrow_mut().zoom_in();
				}
			}) as Box<dyn FnMut(_)>);
			window
				.add_event_listener_with_callback("wheel", closure.as_ref().unchecked_ref())
				.unwrap();
			closure.forget();
		}
	}
}
