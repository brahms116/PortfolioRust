pub mod road_cap;

use crate::utils::entity::*;
use road_cap::RoadCap;

pub enum Roads {
	VRoadCap(RoadCap),
}

impl Roads {
	pub fn get_draw_data(&self) -> EntityDrawData {
		match self {
			Roads::VRoadCap(x) => return x.get_draw_data(),
			_ => panic!("Road not recognised"),
		}
	}
}
