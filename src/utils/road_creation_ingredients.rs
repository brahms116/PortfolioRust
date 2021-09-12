use crate::game_objects::segment::RelativeSegment;
use crate::utils::entity::Surface;
use crate::utils::road_data::RoadPropertyData;
use crate::utils::road_joints::RoadJoints;

pub struct RoadCreationIngredients {
	pub relative_surfaces: Vec<Surface>,
	pub relative_segments: Vec<RelativeSegment>,
	pub relative_joints: RoadJoints,
}

pub struct PropertyCreationIngredients {
	pub left: Vec<RoadPropertyData>,
	pub right: Vec<RoadPropertyData>,
}

pub struct RoadWithPropertyCreationIngredients {
	pub relative_surfaces: Vec<Surface>,
	pub relative_segments: Vec<RelativeSegment>,
	pub relative_joints: RoadJoints,
	pub relative_property_data: PropertyCreationIngredients,
}
