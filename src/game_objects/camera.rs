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
	pub fn new() -> Camera {
		let window = web_sys::window().unwrap();
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
	pub fn draw(&self, entity: &Box<dyn utils::Entity>) {
		let draw_data = entity.get_draw_data();
		let vert_vec = draw_data.vertices;
		self.ctx.begin_path();
		self.ctx.set_fill_style(&draw_data.color.into());

		if vert_vec.len() != 0 && vert_vec.len() > 2 {
			let mut screen_vert_vec = Vec::<utils::Point>::new();
			screen_vert_vec.reserve(vert_vec.len());
			for vert in &vert_vec {
				let x = self.screen_width / 2 + (vert.x - self.anchor.x) * self.zoom;
				let y = self.screen_height / 2 + (vert.y - self.anchor.y) * self.zoom;
				screen_vert_vec.push(utils::Point { x: x, y: y })
			}
			self
				.ctx
				.move_to(screen_vert_vec[0].x as f64, screen_vert_vec[0].y as f64);
			for vert in &screen_vert_vec[1..] {
				self.ctx.line_to(vert.x as f64, vert.y as f64);
			}
			self.ctx.fill();
		}
	}
}
