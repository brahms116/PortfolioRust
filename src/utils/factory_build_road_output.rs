use crate::utils::entity::EntityDrawData;
use crate::utils::road_data::RoadPropertyData;
use crate::utils::road_joints::RoadJoints;
use crate::utils::transform::SinglePointTransform;

pub struct FactoryBuildRoadOutput {
	pub speed_limit: f64,
	pub transform: SinglePointTransform,
	pub joints: RoadJoints,
	pub draw_data: EntityDrawData,
}

pub struct FactoryBuildRoadWithPropertyOutput {
	pub speed_limit: f64,
	pub transform: SinglePointTransform,
	pub joints: RoadJoints,
	pub draw_data: EntityDrawData,
	pub property_data: Vec<RoadPropertyData>,
}
