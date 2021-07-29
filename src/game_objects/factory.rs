use crate::game_objects::roads::road_cap::*;
use crate::game_objects::roads::Roads;
use crate::game_objects::segment::Segment;
use crate::utils::direction::Direction;
use crate::utils::entity::EntityUtils;
use crate::utils::vector_2::Vec2;

pub struct Factory {
	count: i32,
}

impl Factory {
	pub fn create_segment(
		&mut self,
		start_pt: Vec2,
		end_pt: Vec2,
		speed_limit: f64,
		segments: &mut Vec<Segment>,
	) -> i32 {
		let segment = Segment::new(&self.count.to_string(), start_pt, end_pt, speed_limit);
		self.count += 1;
		segments.push(segment);
		segments.len() as i32 - 1
	}

	pub fn create_road_cap(
		&mut self,
		position: Vec2,
		direction: Direction,
		roads: &mut Vec<Roads>,
		segments: &mut Vec<Segment>,
	) -> i32 {
		let segments_by_relative_vecs = RoadCap::get_segment_by_relative_vecs();

		//TODO turn each set of relative_vec into a segment, storing the returning index in a vec<i32>
		let created_segment_indices = Vec::<i32>::new();
		for segment in segments_by_relative_vecs.segments {
			let pts = EntityUtils::get_vert_from_vecs(
				position,
				vec![segment.start_pt, segment.end_pt],
				&direction,
			);
			let index = self.create_segment(pts[0], pts[1], RoadCap::get_speed_limit(), segments);
			created_segment_indices.push(index);
		}

		return 0;
	}
}
