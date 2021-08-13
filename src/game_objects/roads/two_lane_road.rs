use crate::utils::road::RoadCreationData;
use crate::utils::road::RoadData;
use crate::utils::road::RoadPropertyData;
use crate::utils::road::RoadWithPropertyConfig;

pub struct TwoLaneRoad {
	pub id: String,
	pub data: RoadData,
	pub property_data: RoadPropertyData,
}

impl TwoLaneRoad {
	pub fn get_creation_data(config: &RoadWithPropertyConfig) -> RoadCreationData {
		let length = config.transform.get_length();
		
	}
}
