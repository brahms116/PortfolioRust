use crate::game_objects::navigator::*;
use crate::utils::entity::Dynamic;
use crate::utils::entity::Entity;
use crate::utils::entity::EntityData;
use crate::utils::entity::EntityDrawData;
use crate::utils::entity::Surface;
use crate::utils::transform::SinglePointTransform;
use crate::utils::vector_2::*;
pub struct Car {
	id: String,
	is_filled: bool,
	color: String,
	length: i32,
	navigator: Navigator,
	acceleration: f64,
	velocity: f64,
	entity_data: EntityData,
}

impl Car {
	pub fn new(
		is_filled: bool,
		color: String,
		transform: SinglePointTransform,
		id: String,
		length: i32,
	) -> Self {
		Car {
			id,
			entity_data: EntityData {
				transform,
				draw_data: EntityDrawData { surfaces: vec![] },
			},
			is_filled,
			color,
			length,
			navigator: Navigator {
				segment_length_travelled: 0.0,
				offset: 0.0,
			},
			acceleration: 0.0,
			velocity: 0.0,
		}
	}

	pub fn get_id(&self) -> &String {
		&self.id
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
	fn set_draw_data(&mut self) {
		let length_offset = self.get_length_offset();
		if self.is_filled {
			let relative_vecs = vec![
				Vec2::new(-4.0, -length_offset),
				Vec2::new(4.0, -length_offset),
				Vec2::new(4.0, length_offset),
				Vec2::new(-4.0, length_offset),
			];
			let mut car_surface = Surface {
				vertices: relative_vecs,
				color: self.color.clone(),
			};
			car_surface.to_absolute(&self.entity_data.transform);
			self.entity_data.draw_data = EntityDrawData {
				surfaces: vec![car_surface],
			};
		} else {
			let mut surfaces = Vec::<Surface>::new();
			surfaces.reserve(4);
			let relative_vecs = vec![
				Vec2::new(-4.0, -length_offset),
				Vec2::new(4.0, -length_offset),
				Vec2::new(4.0, -length_offset + 1.0),
				Vec2::new(-4.0, -length_offset + 1.0),
			];

			surfaces.push(Surface {
				vertices: relative_vecs,
				color: self.color.clone(),
			});

			let relative_vecs = vec![
				Vec2::new(4.0, -length_offset),
				Vec2::new(4.0, length_offset),
				Vec2::new(3.0, length_offset),
				Vec2::new(3.0, -length_offset),
			];
			surfaces.push(Surface {
				vertices: relative_vecs,
				color: self.color.clone(),
			});

			let relative_vecs = vec![
				Vec2::new(4.0, length_offset),
				Vec2::new(-4.0, length_offset),
				Vec2::new(-3.0, length_offset - 1.0),
				Vec2::new(3.0, length_offset - 1.0),
			];

			surfaces.push(Surface {
				vertices: relative_vecs,
				color: self.color.clone(),
			});

			let relative_vecs = vec![
				Vec2::new(-4.0, length_offset),
				Vec2::new(-4.0, -length_offset),
				Vec2::new(-3.0, -length_offset),
				Vec2::new(-3.0, length_offset),
			];
			surfaces.push(Surface {
				vertices: relative_vecs,
				color: self.color.clone(),
			});

			for surface in &mut surfaces {
				surface.to_absolute(&self.entity_data.transform)
			}
			self.entity_data.draw_data = EntityDrawData { surfaces }
		}
	}
}

impl Dynamic for Car {
	fn update(&mut self) {
		self.acceleration += 1.0;
		self.set_draw_data();
	}
}

impl Entity for Car {
	fn get_enitty_data(&self) -> &EntityData {
		&self.entity_data
	}
}
