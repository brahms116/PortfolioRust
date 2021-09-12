use crate::game_objects::segment::*;
use crate::utils::direction::Direction;
use crate::utils::entity::*;
use crate::utils::road::Road;
use crate::utils::road_creation_ingredients::RoadCreationIngredients;
use crate::utils::road_data::RoadData;
use crate::utils::road_joints::Joint;
use crate::utils::road_joints::RoadJoints;
use crate::utils::vector_2::Vec2;
use std::collections::HashMap;

pub struct RoadCap {
	pub id: String,
	pub road_data: RoadData,
	pub entity_data: EntityData,
}

pub struct RoadCapRelativeSegments {
	pub bottom_entry_seg_1: i32,
	pub bottom_exit_seg_1: i32,
	pub segments: Vec<RelativeSegment>,
}

impl RoadCap {
	pub fn get_creation_data() -> RoadCreationIngredients {
		let relative_segments = vec![
			RelativeSegment {
				start_pt: Vec2::new(-8.0, 0.0),
				end_pt: Vec2::new(-8.0, -4.0),
				nxt_segments: vec![1],
				prev_segments: vec![],
			},
			RelativeSegment {
				start_pt: Vec2::new(-8.0, -4.0),
				end_pt: Vec2::new(-4.0, -8.0),
				nxt_segments: vec![2],
				prev_segments: vec![0],
			},
			RelativeSegment {
				start_pt: Vec2::new(-4.0, -8.0),
				end_pt: Vec2::new(4.0, -8.0),
				nxt_segments: vec![3],
				prev_segments: vec![1],
			},
			RelativeSegment {
				start_pt: Vec2::new(8.0, -4.0),
				end_pt: Vec2::new(3.0, -2.0),
				nxt_segments: vec![4],
				prev_segments: vec![2],
			},
			RelativeSegment {
				start_pt: Vec2::new(8.0, -0.0),
				end_pt: Vec2::new(3.0, 0.0),
				nxt_segments: vec![],
				prev_segments: vec![3],
			},
		];
		let relative_surfaces = RoadCap::get_relative_surfaces();

		let mut relative_joints: HashMap<Direction, Vec<Joint>> = HashMap::new();
		relative_joints.insert(
			Direction::S,
			vec![Joint {
				lane_format: ".|.".to_string(),
				entry_segments: vec![0],
				exit_segments: vec![4],
			}],
		);
		RoadCreationIngredients {
			relative_joints: RoadJoints {
				list: relative_joints,
			},
			relative_surfaces,
			relative_segments,
		}
	}

	fn get_relative_surfaces() -> Vec<Surface> {
		let mut surfaces = Vec::<Surface>::new();
		//pavement
		let relative_vecs = vec![
			Vec2::new(-18.0, 0.0),
			Vec2::new(-18.0, -6.0),
			Vec2::new(-4.0, -19.0),
			Vec2::new(4.0, -19.0),
			Vec2::new(18.0, -6.0),
			Vec2::new(18.0, 0.0),
		];

		surfaces.push(Surface {
			vertices: relative_vecs,
			color: "#9F9F9F".to_string(),
		});

		// road
		let relative_vecs = vec![
			Vec2::new(-16.0, 0.0),
			Vec2::new(-16.0, -5.0),
			Vec2::new(-4.0, -16.0),
			Vec2::new(4.0, -16.0),
			Vec2::new(16.0, -5.0),
			Vec2::new(16.0, 0.0),
		];

		surfaces.push(Surface {
			vertices: relative_vecs,
			color: "#393939".to_string(),
		});
		surfaces
	}
}

impl Road for RoadCap {
	fn get_id(&self) -> &String {
		&self.id
	}
	fn get_road_data(&self) -> &RoadData {
		&self.road_data
	}
}

impl Entity for RoadCap {
	fn get_enitty_data(&self) -> &EntityData {
		&self.entity_data
	}
}
