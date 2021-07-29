use crate::game_objects::segment::*;
use crate::utils::direction::Direction;
use crate::utils::entity::*;
use crate::utils::vector_2::Vec2;

pub struct RoadCap {
	pub id: String,
	pub position: Vec2,
	pub bottom_entry_seg_1: i32,
	pub bottom_exit_seg_1: i32,
	pub other_segments: Vec<i32>,
	pub direction: Direction,
}

pub struct RoadCapSegmentsByRelativeVecs {
	pub bottom_entry_seg_1: i32,
	pub bottom_exit_seg_1: i32,
	pub segments: Vec<SegmentsByRelativeVecs>,
}

impl RoadCap {
	pub fn get_speed_limit() -> f64 {
		3.0
	}
	pub fn get_segment_by_relative_vecs() -> RoadCapSegmentsByRelativeVecs {
		RoadCapSegmentsByRelativeVecs {
			bottom_entry_seg_1: 0,
			bottom_exit_seg_1: 1,
			segments: vec![
				SegmentsByRelativeVecs {
					start_pt: Vec2::new(-3.0, 0.0),
					end_pt: Vec2::new(-3.0, -2.0),
					nxt_segments: vec![1],
					prev_segments: vec![],
				},
				SegmentsByRelativeVecs {
					start_pt: Vec2::new(-3.0, -2.0),
					end_pt: Vec2::new(-2.0, -3.0),
					nxt_segments: vec![2],
					prev_segments: vec![0],
				},
				SegmentsByRelativeVecs {
					start_pt: Vec2::new(-2.0, -3.0),
					end_pt: Vec2::new(2.0, -3.0),
					nxt_segments: vec![3],
					prev_segments: vec![1],
				},
				SegmentsByRelativeVecs {
					start_pt: Vec2::new(2.0, -3.0),
					end_pt: Vec2::new(3.0, -2.0),
					nxt_segments: vec![4],
					prev_segments: vec![2],
				},
				SegmentsByRelativeVecs {
					start_pt: Vec2::new(3.0, -2.0),
					end_pt: Vec2::new(3.0, 0.0),
					nxt_segments: vec![],
					prev_segments: vec![3],
				},
			],
		}
	}

	pub fn get_draw_data(&self) -> EntityDrawData {
		let mut surfaces = Vec::<Surface>::new();
		//pavement
		let relative_vecs = vec![
			Vec2::new(-7.0, 0.0),
			Vec2::new(-7.0, -2.0),
			Vec2::new(-2.0, -7.0),
			Vec2::new(2.0, -7.0),
			Vec2::new(7.0, -2.0),
			Vec2::new(7.0, 0.0),
		];

		let vertices = EntityUtils::get_vert_from_vecs(self.position, &relative_vecs, &self.direction);

		surfaces.push(Surface {
			vertices,
			color: &"#9F9F9F",
		});

		// road
		let relative_vecs = vec![
			Vec2::new(-6.0, 0.0),
			Vec2::new(-6.0, -2.0),
			Vec2::new(-2.0, -6.0),
			Vec2::new(2.0, -6.0),
			Vec2::new(6.0, -2.0),
			Vec2::new(6.0, 0.0),
		];

		let vertices = EntityUtils::get_vert_from_vecs(self.position, &relative_vecs, &self.direction);

		surfaces.push(Surface {
			vertices,
			color: &"#9F9F9F",
		});
		EntityDrawData { surfaces }
	}
}
