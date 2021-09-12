pub mod road_cap;
// pub mod static_road;
// pub mod two_lane_road;
use crate::utils::entity::*;

use crate::utils::road::DynamicRoad;
use crate::utils::road::Road;
use crate::utils::road::WithProperty;

pub enum RoadType {
	Static,
	Dynamic,
	StaticWithProperty,
}

pub struct RoadState {
	pub static_roads: Vec<Box<dyn Road>>,
	pub static_with_property: Vec<Box<dyn WithProperty>>,
	pub dynamic_roads: Vec<Box<dyn DynamicRoad>>,
}

impl RoadState {
	pub fn new() -> RoadState {
		RoadState {
			static_roads: Vec::new(),
			static_with_property: Vec::new(),
			dynamic_roads: Vec::new(),
		}
	}
	pub fn get_draw_data(&self) -> Vec<&EntityDrawData> {
		let mut result_vec = Vec::<&EntityDrawData>::new();
		for road in &self.static_roads {
			result_vec.push(&road.get_enitty_data().draw_data);
		}
		for road in &self.dynamic_roads {
			result_vec.push(&road.get_enitty_data().draw_data);
		}
		for road in &self.static_with_property {
			result_vec.push(&road.get_enitty_data().draw_data);
		}
		result_vec
	}
}
