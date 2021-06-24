pub struct Point {
	pub x: i32,
	pub y: i32,
}

pub struct EntityDrawData {
	pub vertices: Vec<Point>,
	pub color: String,
}

pub trait Entity {
	fn get_draw_data(&self) -> EntityDrawData;
}
