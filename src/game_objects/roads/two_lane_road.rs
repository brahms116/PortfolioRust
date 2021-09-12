use crate::game_objects::segment::RelativeSegment;
use crate::utils::direction::Direction;
use crate::utils::entity::Surface;
use crate::utils::road::Joint;
use crate::utils::road::Lane;
use crate::utils::road::RoadCreationDataWithProperty;
use crate::utils::road::RoadData;
use crate::utils::road::RoadJoints;
use crate::utils::road::RoadPropertyData;
use crate::utils::road::RoadWithPropertyConfig;
use crate::utils::transform::SinglePointTransform;
use crate::utils::vector_2::Vec2;

pub struct TwoLaneRoad {
	pub id: String,
	pub data: RoadData,
	pub property_data: RoadPropertyData,
}

impl TwoLaneRoad {
	pub fn get_creation_data(
		config: &RoadWithPropertyConfig,
	) -> Result<RoadCreationDataWithProperty, &str> {
		let length = config.transform.get_length();

		let mut left_lane = Lane::new(
			&SinglePointTransform {
				position: Vec2::new(-8.0, 0.0),
				direction: Direction::N,
			},
			config.transform.get_length(),
		);

		left_lane.configure_properties(config.properties_left)?;

		let mut right_lane = Lane::new(
			&SinglePointTransform {
				position: Vec2::new(8.0, -config.transform.get_length()),
				direction: Direction::S,
			},
			config.transform.get_length(),
		);

		right_lane.configure_properties(config.properties_right)?;

		let mut relative_segments = Vec::<RelativeSegment>::new();

		let left_build_result = left_lane.build_relative_segments(&mut relative_segments);
		let right_build_result = right_lane.build_relative_segments(&mut relative_segments);
		let property_data = right_build_result
			.property_data
			.unwrap()
			.append(&mut left_build_result.property_data.unwrap());

		let n_joint = Joint {
			lane_format: String::from(".|."),
			exit_segments: vec![left_build_result.exit_segment],
			entry_segments: vec![right_build_result.entry_segment],
		};

		let s_joint = Joint {
			lane_format: String::from(".|."),
			exit_segments: vec![right_build_result.exit_segment],
			entry_segments: vec![left_build_result.entry_segment],
		};

		let joints = RoadJoints {
			list: vec![n_joint, s_joint],
		};

		let mut surfaces = vec![
			Surface {
				vertices: vec![
					Vec2::new(-18.0, 0.0),
					Vec2::new(-18.0, -length),
					Vec2::new(18.0, -length),
					Vec2::new(18.0, 0.0),
				],
				color: String::from("#9f9f9f"),
			},
			Surface {
				vertices: vec![
					Vec2::new(-17.0, 0.0),
					Vec2::new(-17.0, -length),
					Vec2::new(17.0, -length),
					Vec2::new(17.0, 0.0),
				],
				color: String::from("#393939"),
			},
		];
		let num_marker = ((length - 10.0) / 20.0).floor() as i32;
		for i in 0..num_marker {}
		RoadCreationDataWithProperty {}
	}
}
