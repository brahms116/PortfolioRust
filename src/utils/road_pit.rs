use crate::game_objects::segment::RelativeSegment;
use crate::utils::direction::Direction;
use crate::utils::road_data::RoadPropertyData;
use crate::utils::transform::SinglePointTransform;
use crate::utils::vector_2::Vec2;

pub struct RoadPit<'a> {
	pub transform: &'a SinglePointTransform,
}

impl RoadPit<'_> {
	pub fn build_relative_segments(&self, segments: &mut Vec<RelativeSegment>) -> RoadPitBuildResult {
		let entry_segment_1 = segments.len();
		let entry_segment_2 = entry_segment_1 + 1;
		segments.push(RelativeSegment {
			start_pt: Vec2::new(0.0, -18.0).to_absolute(&self.transform),
			end_pt: Vec2::new(0.0, 18.0).to_absolute(&self.transform),
			prev_segments: vec![],
			nxt_segments: vec![],
		});
		segments.push(RelativeSegment {
			start_pt: Vec2::new(0.0, 18.0).to_absolute(&self.transform),
			end_pt: Vec2::new(-9.0, 9.0).to_absolute(&self.transform),
			prev_segments: vec![],
			nxt_segments: vec![entry_segment_1 as i32 + 2],
		});
		segments.push(RelativeSegment {
			start_pt: Vec2::new(-9.0, 9.0).to_absolute(&self.transform),
			end_pt: Vec2::new(-14.0, 9.0).to_absolute(&self.transform),
			prev_segments: vec![entry_segment_1 as i32 + 1],
			nxt_segments: vec![entry_segment_1 as i32 + 3],
		});
		segments.push(RelativeSegment {
			start_pt: Vec2::new(-14.0, 9.0).to_absolute(&self.transform),
			end_pt: Vec2::new(-14.0, -9.0).to_absolute(&self.transform),
			prev_segments: vec![entry_segment_1 as i32 + 2],
			nxt_segments: vec![entry_segment_1 as i32 + 4],
		});
		segments.push(RelativeSegment {
			start_pt: Vec2::new(-14.0, -9.0).to_absolute(&self.transform),
			end_pt: Vec2::new(-9.0, -9.0).to_absolute(&self.transform),
			prev_segments: vec![entry_segment_1 as i32 + 3],
			nxt_segments: vec![entry_segment_1 as i32 + 5],
		});
		segments.push(RelativeSegment {
			start_pt: Vec2::new(-9.0, -9.0).to_absolute(&self.transform),
			end_pt: Vec2::new(0.0, -18.0).to_absolute(&self.transform),
			prev_segments: vec![entry_segment_1 as i32 + 4],
			nxt_segments: vec![],
		});
		let property_data = RoadPropertyData {
			segment_index: entry_segment_1 as i32 + 3,
			direction: Direction::from_i32(6 + self.transform.direction.to_i32()),
		};
		RoadPitBuildResult {
			property_data,
			entry_segment_1: entry_segment_1 as i32,
			entry_segment_2: entry_segment_2 as i32,
			exit_segment_1: entry_segment_1 as i32,
			exit_segment_2: entry_segment_1 as i32 + 5,
		}
	}
}

pub struct RoadPitBuildResult {
	pub entry_segment_1: i32,
	pub entry_segment_2: i32,
	pub exit_segment_1: i32,
	pub exit_segment_2: i32,
	pub property_data: RoadPropertyData,
}
