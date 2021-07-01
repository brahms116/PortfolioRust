#[derive(Copy, Debug)]
pub struct Vec2 {
	pub x: f64,
	pub y: f64,
}
impl Vec2 {
	pub fn new(x: f64, y: f64) -> Vec2 {
		Vec2 { x: x, y: y }
	}
}
impl Clone for Vec2 {
	fn clone(&self) -> Vec2 {
		Vec2::new(self.x, self.y)
	}
}

pub enum Direction {
	N,
	NE,
	E,
	SE,
	S,
	SW,
	W,
	NW,
}

pub struct EntityDrawData {
	pub vertices: Vec<Vec2>,
	pub color: String,
}

pub trait Entity {
	fn get_draw_data(&self) -> EntityDrawData;
}

pub trait Dynamic {
	fn update(&mut self) -> ();
}
pub trait DynamicEntity: Entity + Dynamic {}

pub enum TweenState {
	Idle,
	Increasing,
	Decreasing,
}
