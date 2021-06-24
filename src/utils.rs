pub struct Point {
	pub x: i32,
	pub y: i32,
}
impl Point {
	pub fn new(x: i32, y: i32) -> Point {
		Point { x: x, y: y }
	}
}

pub struct EntityDrawData {
	pub vertices: Vec<Point>,
	pub color: String,
}

pub trait Entity {
	fn get_draw_data(&self) -> EntityDrawData;
}
