use crate::options::*;
use std::f64::consts::PI;
use std::ops;

#[derive(Copy, Debug, Clone, PartialEq)]
pub struct Vec2 {
	pub x: f64,
	pub y: f64,
}
impl Vec2 {
	pub fn new(x: f64, y: f64) -> Vec2 {
		Vec2 { x: x, y: y }
	}
	pub fn get_magnitude(&self) -> f64 {
		(self.x.powi(2) + self.y.powi(2)).powf(0.5)
	}
	pub fn rotate(&mut self, origin: Vec2, clockwise_steps: i32) {
		if clockwise_steps > 7 {
			panic!("cannot rotate more than 7 times");
		}
		if clockwise_steps <= 0 {
			return;
		}
		let mut displacement_vec = *self - origin;
		// web_sys::console::log_2(&"before".into(), &format!("{:?}", displacement_vec).into());
		match clockwise_steps {
			1 | 3 | 5 | 7 => {
				let temp_vec = displacement_vec;
				displacement_vec.x = temp_vec.x * (clockwise_steps as f64 * PI / 4.0).cos()
					- temp_vec.y * (clockwise_steps as f64 * PI / 4.0).sin();
				displacement_vec.y = temp_vec.x * (clockwise_steps as f64 * PI / 4.0).sin()
					+ temp_vec.y * (clockwise_steps as f64 * PI / 4.0).cos();
			}
			2 => {
				let temp_vec = displacement_vec;
				displacement_vec.x = -temp_vec.y;
				displacement_vec.y = temp_vec.x;
			}
			4 => {
				displacement_vec.x = -displacement_vec.x;
				displacement_vec.y = -displacement_vec.y;
			}
			6 => {
				let temp_vec = displacement_vec;
				displacement_vec.x = temp_vec.y;
				displacement_vec.y = -temp_vec.x;
			}
			_ => {}
		}

		// web_sys::console::log_2(&"after".into(), &format!("{:?}", displacement_vec).into());
		self.x = displacement_vec.x + origin.x;
		self.y = displacement_vec.y + origin.y;
	}
}
// impl Clone for Vec2 {
// 	fn clone(&self) -> Vec2 {
// 		Vec2::new(self.x, self.y)
// 	}
// }

impl ops::Add<Vec2> for Vec2 {
	type Output = Vec2;
	fn add(self, _rhs: Vec2) -> Vec2 {
		Vec2 {
			x: self.x + _rhs.x,
			y: self.y + _rhs.y,
		}
	}
}

impl ops::Sub<Vec2> for Vec2 {
	type Output = Self;
	fn sub(self, _rhs: Self) -> Vec2 {
		Vec2 {
			x: self.x - _rhs.x,
			y: self.y - _rhs.y,
		}
	}
}
