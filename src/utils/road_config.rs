use crate::utils::transform::SinglePointTransform;
use crate::utils::transform::TwoPointTransform;

pub struct RoadPropertyConfig {
	pub left: i32,
	pub right: i32,
}
pub struct SPTTransformRoadConfig {
	pub speed_limit: f64,
	pub transform: SinglePointTransform,
}

pub struct TPTransformRoadConfig {
	pub speed_limit: f64,
	pub transform: TwoPointTransform,
}

pub struct SPTransformRoadConfigWithProperty {
	pub speed_limit: f64,
	pub transform: SinglePointTransform,
	pub properties: RoadPropertyConfig,
}
pub struct TPTransformRoadConfigWithProperty {
	pub speed_limit: f64,
	pub transform: TwoPointTransform,
	pub properties: RoadPropertyConfig,
}
