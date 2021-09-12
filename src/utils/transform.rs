use crate::utils::direction::*;
use crate::utils::vector_2::*;

pub struct TwoPointTransform {
	pub start_pt: Vec2,
	pub end_pt: Vec2,
}

impl TwoPointTransform {
	pub fn to_single_pt_transform(&self) -> SinglePointTransform {
		let diff_vec = self.end_pt - self.start_pt;
		SinglePointTransform {
			position: self.start_pt,
			direction: diff_vec.get_direction(),
		}
	}
	pub fn get_length(&self) -> f64 {
		let diff_vec = self.end_pt - self.start_pt;
		diff_vec.get_magnitude()
	}
}

pub struct SinglePointTransform {
	pub position: Vec2,
	pub direction: Direction,
}

pub trait HasTransform {
	fn get_transform(&self) -> &SinglePointTransform;
	fn get_transform_mut(&self) -> &mut SinglePointTransform;
}
