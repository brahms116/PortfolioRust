use crate::game_objects::segment::RelativeSegment;
use crate::utils::road_data::RoadPropertyData;
use crate::utils::road_pit::RoadPit;
use crate::utils::transform::SinglePointTransform;
use crate::utils::vector_2::Vec2;

pub struct Lane<'a> {
	pub transform: &'a SinglePointTransform,
	pub length: f64,
	num_property: i32,
}
impl Lane<'_> {
	pub fn new<'a>(transform: &'a SinglePointTransform, length: f64) -> Lane {
		Lane {
			transform,
			length,
			num_property: 0,
		}
	}

	pub fn configure_properties(&mut self, num_property: i32) -> Result<(), &str> {
		if num_property as f64 * 38.0 > self.length {
			return Err(&"Road too short");
		}
		self.num_property = num_property;
		Ok(())
	}

	pub fn build_relative_segments(&self, segments: &mut Vec<RelativeSegment>) -> LaneBuildResult {
		let end_pt = Vec2::new(self.length, 0.0).to_absolute(&self.transform);
		if self.num_property != 0 {
			let gap_distance =
				(self.length - 38.0 * self.num_property as f64) / (self.num_property + 1) as f64;
			let entry_segment = segments.len() as i32;
			let mut temp_exit_segment: i32 = 0;
			let mut property_data = Vec::<RoadPropertyData>::new();
			for i in 0..self.num_property {
				let start_pt = if i == 0 {
					self.transform.position
				} else {
					segments[temp_exit_segment as usize].end_pt
				};
				segments.push(RelativeSegment {
					start_pt,
					end_pt: start_pt + Vec2::new(1.0 + gap_distance, 0.0).to_absolute(&self.transform),
					prev_segments: vec![],
					nxt_segments: vec![],
				});
				let before_pit = segments.len() as i32 - 1;
				if i != 0 {
					segments[before_pit as usize]
						.prev_segments
						.push(temp_exit_segment);
				}
				let x_offset = (gap_distance + 38.0) * i as f64 + gap_distance + 19.0;
				let pit = RoadPit {
					transform: &SinglePointTransform {
						direction: self.transform.direction.clone(),
						position: Vec2::new(x_offset, 0.0).to_absolute(&self.transform),
					},
				};
				let pit_result = pit.build_relative_segments(segments);

				//TODO might have to clone here
				property_data.push(pit_result.property_data);

				// segment before_pit to pit
				segments[before_pit as usize]
					.nxt_segments
					.push(pit_result.entry_segment_1);
				segments[before_pit as usize]
					.nxt_segments
					.push(pit_result.entry_segment_2);
				segments[pit_result.entry_segment_1 as usize]
					.prev_segments
					.push(before_pit);
				segments[pit_result.entry_segment_2 as usize]
					.prev_segments
					.push(before_pit);

				segments.push(RelativeSegment {
					start_pt: segments[pit_result.exit_segment_1 as usize].end_pt,
					end_pt: Vec2::new(x_offset + 19.0, 0.0).to_absolute(&self.transform),
					prev_segments: vec![pit_result.exit_segment_1, pit_result.exit_segment_2],
					nxt_segments: vec![],
				});
				temp_exit_segment = segments.len() as i32 - 1;
			}
			segments.push(RelativeSegment {
				start_pt: segments[temp_exit_segment as usize].start_pt,
				end_pt,
				prev_segments: vec![temp_exit_segment],
				nxt_segments: vec![],
			});

			return LaneBuildResult {
				entry_segment,
				exit_segment: segments.len() as i32 - 1,
				property_data: Some(property_data),
			};
		} else {
			segments.push(RelativeSegment {
				start_pt: self.transform.position,
				end_pt,
				prev_segments: vec![],
				nxt_segments: vec![],
			});
			let index = segments.len() as i32 - 1;
			return LaneBuildResult {
				entry_segment: index,
				exit_segment: index,
				property_data: None,
			};
		}
	}
}
pub struct LaneBuildResult {
	pub entry_segment: i32,
	pub exit_segment: i32,
	pub property_data: Option<Vec<RoadPropertyData>>,
}
