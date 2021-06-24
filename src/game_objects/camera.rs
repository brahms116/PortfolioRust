use crate::utils;
use wasm_bindgen::JsCast;
pub struct Camera {
	zoom: i32,
	anchor: utils::Point,
	screen_height: i32,
	screen_width: i32,
	window: web_sys::Window,
	ctx: web_sys::CanvasRenderingContext2d,
	canvas: web_sys::HtmlCanvasElement,
}

impl Camera {
	pub fn new(window: web_sys::Window) -> Camera {
		let canvas = window
			.document()
			.unwrap()
			.get_element_by_id("canvas")
			.unwrap()
			.dyn_into::<web_sys::HtmlCanvasElement>()
			.unwrap();
		let ctx = canvas
			.get_context("2d")
			.unwrap()
			.unwrap()
			.dyn_into::<web_sys::CanvasRenderingContext2d>()
			.unwrap();
		Camera {
			zoom: 0,
			anchor: utils::Point { x: 0, y: 0 },
			screen_height: 0,
			screen_width: 0,
			window: window,
			ctx: ctx,
			canvas: canvas,
		}
	}
	pub fn update_screen_dimensions(&mut self) {
		self.screen_height = self
			.window
			.inner_height()
			.unwrap()
			.as_f64()
			.unwrap()
			.round() as i32;
		self.screen_width = self.window.inner_width().unwrap().as_f64().unwrap().round() as i32;
	}
}
