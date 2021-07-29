use crate::game_objects::navigator::*;
use crate::utils::direction::*;
use crate::utils::entity::*;
use crate::utils::vector_2::*;
pub struct Car {
	position: Vec2, // I need this to get the draw data
	is_filled: bool,
	color: String,
	length: i32,
	navigator: Navigator,
	acceleration: f64,
	velocity: f64,
	direction: Direction,
}

impl Car {
	pub fn new(is_filled: bool, color: String, position: Vec2, direction: Direction) -> Self {
		Car {
			position,
			is_filled,
			color,
			length: 9,
			navigator: Navigator {
				segment_length_travelled: 0.0,
				offset: 0.0,
			},
			acceleration: 0.0,
			velocity: 0.0,
			direction,
		}
	}
	pub fn get_navigator(&self) -> &Navigator {
		&self.navigator
	}
	pub fn get_length_offset(&self) -> f64 {
		(self.length / 2) as f64
	}
	pub fn get_velocity(&self) -> f64 {
		self.velocity
	}
	pub fn get_acceleration(&self) -> f64 {
		self.acceleration
	}

	pub fn update(&mut self) {
		self.acceleration += 1.0;
	}
}

impl Entity for Car {
	fn get_draw_data(&self) -> EntityDrawData {
		let length_offset = self.get_length_offset();
		if self.is_filled {
			let relative_vecs = vec![
				Vec2::new(-2.0, -length_offset),
				Vec2::new(2.0, -length_offset),
				Vec2::new(2.0, length_offset),
				Vec2::new(-2.0, length_offset),
			];
			let vertices =
				EntityUtils::get_vert_from_vecs(self.position, &relative_vecs, &self.direction);
			let car_surface = Surface {
				vertices,
				color: &self.color,
			};
			return EntityDrawData {
				surfaces: vec![car_surface],
			};
		} else {
			let mut surfaces = Vec::<Surface>::new();
			surfaces.reserve(4);
			let relative_vecs = vec![
				Vec2::new(-2.0, -length_offset),
				Vec2::new(2.0, -length_offset),
				Vec2::new(2.0, -length_offset + 1.0),
				Vec2::new(-2.0, -length_offset + 1.0),
			];
			let vertices =
				EntityUtils::get_vert_from_vecs(self.position, &relative_vecs, &self.direction);

			surfaces.push(Surface {
				vertices,
				color: &self.color,
			});

			let relative_vecs = vec![
				Vec2::new(2.0, -length_offset),
				Vec2::new(2.0, length_offset),
				Vec2::new(1.0, length_offset),
				Vec2::new(1.0, -length_offset),
			];

			let vertices =
				EntityUtils::get_vert_from_vecs(self.position, &relative_vecs, &self.direction);

			surfaces.push(Surface {
				vertices,
				color: &self.color,
			});

			let relative_vecs = vec![
				Vec2::new(2.0, length_offset),
				Vec2::new(-2.0, length_offset),
				Vec2::new(-2.0, length_offset - 1.0),
				Vec2::new(2.0, length_offset - 1.0),
			];

			let vertices =
				EntityUtils::get_vert_from_vecs(self.position, &relative_vecs, &self.direction);

			surfaces.push(Surface {
				vertices,
				color: &self.color,
			});

			let relative_vecs = vec![
				Vec2::new(-2.0, length_offset),
				Vec2::new(-2.0, -length_offset),
				Vec2::new(-1.0, -length_offset),
				Vec2::new(-1.0, length_offset),
			];

			let vertices =
				EntityUtils::get_vert_from_vecs(self.position, &relative_vecs, &self.direction);

			surfaces.push(Surface {
				vertices,
				color: &self.color,
			});

			return EntityDrawData { surfaces: surfaces };
		}
	}
}
