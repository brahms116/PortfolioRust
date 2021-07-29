use crate::utils::direction::*;
use crate::utils::vector_2::*;
pub struct EntityDrawData<'a> {
	pub surfaces: Vec<Surface<'a>>,
}
pub struct Surface<'a> {
	pub vertices: Vec<Vec2>,
	pub color: &'a str,
}

pub trait Entity {
	fn get_draw_data(&self) -> EntityDrawData;
}

pub struct EntityUtils;

impl EntityUtils {
	//Takes a north facing object with its relative vectors and returns list of vertices
	pub fn get_vert_from_vecs(position: Vec2, vecs: &Vec<Vec2>, direction: &Direction) -> Vec<Vec2> {
		let mut result = Vec::<Vec2>::new();
		result.reserve(vecs.len());
		for vec in vecs {
			let mut new_vec = *vec + position;
			match direction {
				Direction::NE => new_vec.rotate(position, 1),
				Direction::E => new_vec.rotate(position, 2),
				Direction::SE => new_vec.rotate(position, 3),
				Direction::S => new_vec.rotate(position, 4),
				Direction::SW => new_vec.rotate(position, 5),
				Direction::W => new_vec.rotate(position, 6),
				Direction::NW => new_vec.rotate(position, 7),
				Direction::N => {}
			}
			result.push(new_vec);
		}
		result
	}
}
