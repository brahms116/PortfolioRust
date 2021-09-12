use crate::utils::transform::SinglePointTransform;

pub struct StaticRoad {
	id: String,
	transform: SinglePointTransform,
	road_data: RoadData,
}
