use crate::options::*;
use crate::utils::vector_2::*;

pub struct SegmentsByRelativeVecs {
	pub start_pt: Vec2,
	pub end_pt: Vec2,
	pub nxt_segments: Vec<i32>,
	pub prev_segments: Vec<i32>,
}

pub struct Segment {
	id: String,
	cars: Vec<i32>,
	nxt_segments: Vec<i32>,
	prev_segments: Vec<i32>,
	start_pt: Vec2,
	end_pt: Vec2,
	speed_limit: f64,
	pub start_inset: f64,
	pub end_inset: f64,
}

impl Segment {
	pub fn new(id: &str, start_pt: Vec2, end_pt: Vec2, speed_limit: f64) -> Segment {
		Segment {
			id: String::from(id),
			cars: vec![],
			nxt_segments: vec![],
			prev_segments: vec![],
			start_pt,
			end_pt,
			speed_limit,
			start_inset: 0.0,
			end_inset: 0.0,
		}
	}

	pub fn get_id<'a>(&'a self) -> &'a str {
		&self.id
	}

	pub fn get_length(&self) -> f64 {
		let diff_vec = self.end_pt - self.start_pt;
		diff_vec.get_magnitude()
	}

	pub fn get_prev_segments(&self) -> &Vec<i32> {
		&self.prev_segments
	}

	pub fn get_nxt_segments(&self) -> &Vec<i32> {
		&self.nxt_segments
	}

	pub fn get_cars(&self) -> &Vec<i32> {
		&self.cars
	}

	pub fn get_end_pt(&self) -> Vec2 {
		self.end_pt
	}

	pub fn get_start_pt(&self) -> Vec2 {
		self.start_pt
	}

	pub fn get_speed_limit(&self) -> f64 {
		self.speed_limit
	}

	pub fn get_position_via_len(&self, len: f64) -> Vec2 {
		let displacement = Vec2::new(SIN_45 * len, SIN_45 * len);
		self.start_pt + displacement
	}

	pub fn add_nxt_seg(&mut self, idx: i32) {
		self.nxt_segments.push(idx);
	}

	pub fn add_prev_seg(&mut self, idx: i32) {
		self.prev_segments.push(idx);
	}

	pub fn remove_car(&mut self, idx: i32) -> Result<(), &str> {
		for i in 0..self.cars.len() {
			if self.cars[i] == idx {
				self.cars.remove(i);
				return Ok(());
			}
		}
		Err(&"Could not find car index whilst trying to remove from segment")
	}

	pub fn add_car(&mut self, car_idx: i32) {
		self.cars.push(car_idx);
	}
}
