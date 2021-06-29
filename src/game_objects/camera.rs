use crate::options::*;
use crate::utils::*;
use wasm_bindgen::JsCast;
pub struct CameraTriggerAreaState {
	pub is_n: bool,
	pub is_s: bool,
	pub is_e: bool,
	pub is_w: bool,
}
impl CameraTriggerAreaState {
	fn new() -> CameraTriggerAreaState {
		CameraTriggerAreaState {
			is_e: false,
			is_n: false,
			is_w: false,
			is_s: false,
		}
	}
}
pub struct Camera {
	zoom: i32,
	anchor: Vec2,
	screen_height: i32,
	screen_width: i32,
	window: web_sys::Window,
	ctx: web_sys::CanvasRenderingContext2d,
	canvas: web_sys::HtmlCanvasElement,
	trigger_area_state: CameraTriggerAreaState,
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
			zoom: 1,
			anchor: Vec2 { x: 0.0, y: 0.0 },
			screen_height: 0,
			screen_width: 0,
			window: window,
			ctx: ctx,
			canvas: canvas,
			trigger_area_state: CameraTriggerAreaState::new(),
		}
	}
	pub fn get_zoom(&self) -> i32 {
		self.zoom
	}
	pub fn set_zoom(&mut self, zoom: i32) {
		self.zoom = zoom;
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
		self.canvas.set_height(self.screen_height as u32);
		self.canvas.set_width(self.screen_width as u32);
	}
	pub fn update_pos(&mut self) {
		if self.trigger_area_state.is_n && self.trigger_area_state.is_e {
			self.move_camera(Direction::NE);
		} else if self.trigger_area_state.is_n && self.trigger_area_state.is_w {
			self.move_camera(Direction::NW);
		} else if self.trigger_area_state.is_s && self.trigger_area_state.is_e {
			self.move_camera(Direction::SE);
		} else if self.trigger_area_state.is_s && self.trigger_area_state.is_w {
			self.move_camera(Direction::SW);
		} else if self.trigger_area_state.is_n {
			self.move_camera(Direction::N);
		} else if self.trigger_area_state.is_e {
			self.move_camera(Direction::E);
		} else if self.trigger_area_state.is_s {
			self.move_camera(Direction::S);
		} else if self.trigger_area_state.is_w {
			self.move_camera(Direction::W);
		}
	}
	pub fn move_camera(&mut self, direction: Direction) {
		let cur_loc = self.get_anchor();
		match direction {
			Direction::N => {
				self.set_anchor(Vec2::new(
					cur_loc.x,
					cur_loc.y - CAMERA_SPD as f64 * 1.0 / self.zoom as f64,
				));
			}
			Direction::S => {
				self.set_anchor(Vec2::new(
					cur_loc.x,
					cur_loc.y + CAMERA_SPD as f64 * 1.0 / self.zoom as f64,
				));
			}
			Direction::W => {
				self.set_anchor(Vec2::new(
					cur_loc.x - CAMERA_SPD as f64 * 1.0 / self.zoom as f64,
					cur_loc.y,
				));
			}
			Direction::E => {
				self.set_anchor(Vec2::new(
					cur_loc.x + CAMERA_SPD as f64 * 1.0 / self.zoom as f64,
					cur_loc.y,
				));
			}
			Direction::NE => {
				self.set_anchor(Vec2::new(
					cur_loc.x + CAMERA_SPD as f64 * DIAG_MOV_UNIT * 1.0 / self.zoom as f64,
					cur_loc.y - CAMERA_SPD as f64 * DIAG_MOV_UNIT * 1.0 / self.zoom as f64,
				));
			}
			Direction::NW => {
				self.set_anchor(Vec2::new(
					cur_loc.x - CAMERA_SPD as f64 * DIAG_MOV_UNIT * 1.0 / self.zoom as f64,
					cur_loc.y - CAMERA_SPD as f64 * DIAG_MOV_UNIT * 1.0 / self.zoom as f64,
				));
			}
			Direction::SW => {
				self.set_anchor(Vec2::new(
					cur_loc.x - CAMERA_SPD as f64 * DIAG_MOV_UNIT * 1.0 / self.zoom as f64,
					cur_loc.y + CAMERA_SPD as f64 * DIAG_MOV_UNIT * 1.0 / self.zoom as f64,
				));
			}
			Direction::SE => {
				self.set_anchor(Vec2::new(
					cur_loc.x + CAMERA_SPD as f64 * DIAG_MOV_UNIT * 1.0 / self.zoom as f64,
					cur_loc.y + CAMERA_SPD as f64 * DIAG_MOV_UNIT * 1.0 / self.zoom as f64,
				));
			}
		}
	}
	pub fn clear(&self) {
		self.ctx.set_fill_style(&"#000000".into());

		self.ctx.fill_rect(
			0.0,
			0.0,
			self.screen_width as f64,
			self.screen_height as f64,
		)
	}
	pub fn draw<T: Entity + ?Sized>(&self, entity: &Box<T>) {
		let draw_data = entity.get_draw_data();
		let vert_vec = draw_data.vertices;
		self.ctx.begin_path();
		self.ctx.set_fill_style(&draw_data.color.into());

		if vert_vec.len() != 0 && vert_vec.len() > 2 {
			let mut screen_vert_vec = Vec::<Vec2>::new();
			screen_vert_vec.reserve(vert_vec.len());
			for vert in &vert_vec {
				let x = (self.screen_width / 2) as f64 + (vert.x - self.anchor.x) * self.zoom as f64;
				let y = (self.screen_height / 2) as f64 + (vert.y - self.anchor.y) * self.zoom as f64;
				let x = x.floor();
				let y = y.floor();
				screen_vert_vec.push(Vec2 { x: x, y: y });
			}
			self.ctx.move_to(screen_vert_vec[0].x, screen_vert_vec[0].y);
			for vert in &screen_vert_vec[1..] {
				self.ctx.line_to(vert.x, vert.y);
			}
			self.ctx.fill();
		}
	}
	pub fn get_screen_dimensions(&self) -> (i32, i32) {
		(self.screen_width, self.screen_height)
	}
	pub fn get_anchor(&self) -> Vec2 {
		self.anchor
	}
	pub fn set_anchor(&mut self, loc: Vec2) {
		self.anchor = loc;
	}
	pub fn set_trigger_area_state(&mut self, trigger_area_state: CameraTriggerAreaState) {
		self.trigger_area_state = trigger_area_state;
	}
}
