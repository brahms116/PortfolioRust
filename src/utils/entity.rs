use crate::utils::transform::SinglePointTransform;
use crate::utils::vector_2::*;
pub struct EntityDrawData {
	pub surfaces: Vec<Surface>,
}
pub struct Surface {
	pub vertices: Vec<Vec2>,
	pub color: String,
}

impl Surface {
	pub fn to_absolute(&mut self, transform: &SinglePointTransform) {
		for vec in self.vertices.iter_mut() {
			*vec = vec.to_absolute(transform)
		}
	}
}

pub trait Entity {
	fn get_draw_data(&self) -> &EntityDrawData;
}

pub trait Dynamic {
	fn update(&mut self);
}

pub trait Drawer {
	fn draw(&self, draw_data: &EntityDrawData);
}

pub struct EntityUtils;

impl EntityUtils {}
