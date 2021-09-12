use crate::utils::direction::Direction;
use std::collections::HashMap;

pub struct Joint {
	pub lane_format: String,
	pub entry_segments: Vec<i32>,
	pub exit_segments: Vec<i32>,
}
pub struct RoadJoints {
	pub list: HashMap<Direction, Vec<Joint>>,
}

impl RoadJoints {
	pub fn to_absolute(self, direction: &Direction, created_indicies: &Vec<i32>) -> RoadJoints {
		// TODO what is this abomination?
		let mut result_map: HashMap<Direction, Vec<Joint>> = HashMap::new();
		for (k, v) in self.list {
			let mut joint_vec = Vec::<Joint>::new();
			for joint in &v {
				let mut new_joint = Joint {
					lane_format: joint.lane_format.clone(),
					entry_segments: vec![],
					exit_segments: vec![],
				};

				for i in &joint.entry_segments {
					new_joint.entry_segments.push(created_indicies[*i as usize])
				}
				for i in &joint.exit_segments {
					new_joint.exit_segments.push(created_indicies[*i as usize])
				}
				joint_vec.push(new_joint);
			}
			let new_direction = Direction::from_i32(direction.to_i32() + k.to_i32());
			result_map.insert(new_direction, joint_vec);
		}
		RoadJoints { list: result_map }
	}
}
