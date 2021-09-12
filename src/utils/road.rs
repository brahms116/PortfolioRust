use crate::game_objects::roads::RoadType;
use crate::utils::entity::Dynamic;
use crate::utils::entity::Entity;
use crate::utils::road_data::RoadData;
use crate::utils::road_data::RoadPropertyData;

pub struct RoadReference {
	pub road_type: RoadType,
	pub index: i32,
}

pub trait Road: Entity {
	fn get_road_data(&self) -> &RoadData;
	fn get_id(&self) -> &String;
}

pub trait DynamicRoad: Road + Dynamic {}

pub trait WithProperty: Road {
	fn get_property_data(&self) -> Vec<RoadPropertyData>;
}
