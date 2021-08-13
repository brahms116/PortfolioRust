use crate::game_objects::roads::RoadType;
use crate::game_objects::segment::RelativeSegment;
use crate::game_objects::segment::Segment;
use crate::utils::direction::Direction;
use crate::utils::entity::Dynamic;
use crate::utils::entity::Entity;
use crate::utils::entity::EntityDrawData;
use crate::utils::entity::Surface;
use crate::utils::transform::*;
use std::collections::HashMap;

pub struct RoadCreationData {
	pub relative_surfaces: Vec<Surface>,
	pub relative_segments: Vec<RelativeSegment>,
	pub relative_joints: RoadJoints,
}

pub struct RoadCreationDataWithProperty {
	pub relative_surfaces: Vec<Surface>,
	pub relative_segments: Vec<RelativeSegment>,
	pub relative_joints: RoadJoints,
	pub relative_property_data: RoadPropertyData,
}

pub struct RoadData {
	pub speed_limit: f64,
	pub transform: SinglePointTransform,
	pub draw_data: EntityDrawData,
	pub joints: RoadJoints,
}

pub struct BasicRoadConfig {
	pub speed_limit: f64,
	pub transform: SinglePointTransform,
}

pub struct RoadWithPropertyConfig {
	pub speed_limit: f64,
	pub transform: TwoPointTransform,
	pub properties_left: i32,
	pub properties_right: i32,
}

pub struct RoadReference {
	pub road_type: RoadType,
	pub index: i32,
}
pub struct PropertyData {
	pub segment_index: i32,
	pub direction: Direction,
}

impl PropertyData {
	pub fn to_abolute(
		&self,
		transform: &SinglePointTransform,
		created_segment_indices: &Vec<i32>,
	) -> PropertyData {
		let new_direction = Direction::from_i32(self.direction.to_i32() + transform.direction.to_i32());
		PropertyData {
			direction: new_direction,
			segment_index: created_segment_indices[self.segment_index as usize],
		}
	}
}

pub struct RoadPropertyData {
	pub left: Vec<PropertyData>,
	pub right: Vec<PropertyData>,
}
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

pub struct RoadPit {
	pub transform: SinglePointTransform,
}

impl RoadPit {
	pub fn build_pit(&self, segments: &mut Vec<RelativeSegment>) {}
}

pub struct RoadPitBuildResult {
	pub segment_entry_1: i32,
	pub segment_entry_2: i32,
	pub segment_exit_1: i32,
	pub segment_exit_2: i32,
	pub relative_property_data: PropertyData,
}

pub trait Road: Entity {
	fn get_road_data(&self) -> &RoadData;
	fn get_id(&self) -> &String;
}

pub trait DynamicRoad: Road + Dynamic {}

pub trait WithProperty: Road {
	fn get_property_data(&self) -> Vec<PropertyData>;
}
