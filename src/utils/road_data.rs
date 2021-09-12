use crate::utils::direction::Direction;
use crate::utils::road_joints::Joint;
use crate::utils::road_joints::RoadJoints;
use crate::utils::transform::SinglePointTransform;

pub struct RoadData {
	pub joints: RoadJoints,
	pub speed_limit: f64,
}

pub struct RoadDynamicData {
	pub controlled_segments: Vec<i32>,
	pub cycles: i32,
}

pub struct RoadPropertyData {
	pub segment_index: i32,
	pub direction: Direction,
}

impl RoadPropertyData {
	pub fn to_absolute(
		&self,
		transform: &SinglePointTransform,
		created_segment_indices: &Vec<i32>,
	) -> RoadPropertyData {
		let new_direction = Direction::from_i32(self.direction.to_i32() + transform.direction.to_i32());
		RoadPropertyData {
			direction: new_direction,
			segment_index: created_segment_indices[self.segment_index as usize],
		}
	}
}
