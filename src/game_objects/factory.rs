use crate::game_objects::car::Car;
use crate::game_objects::roads::road_cap::*;
use crate::game_objects::roads::RoadState;
use crate::game_objects::roads::RoadType;
use crate::game_objects::segment::RelativeSegment;
use crate::game_objects::segment::Segment;
use crate::utils::car::CarConfig;
use crate::utils::entity::EntityData;
use crate::utils::entity::EntityDrawData;
use crate::utils::factory_build_road_output::FactoryBuildRoadOutput;
use crate::utils::road::Road;
use crate::utils::road::RoadReference;
use crate::utils::road_config::SPTTransformRoadConfig;
use crate::utils::road_creation_ingredients::RoadCreationIngredients;
use crate::utils::road_data::RoadData;
use crate::utils::transform::TwoPointTransform;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Factory {
	count: i32,
	segments: Rc<RefCell<Vec<Segment>>>,
	roads: Rc<RefCell<RoadState>>,
	cars: Rc<RefCell<Vec<Car>>>,
}

pub struct FactoryCreationInput {
	pub segments: Rc<RefCell<Vec<Segment>>>,
	pub roads: Rc<RefCell<RoadState>>,
	pub cars: Rc<RefCell<Vec<Car>>>,
}

impl Factory {
	pub fn new(input: FactoryCreationInput) -> Factory {
		Factory {
			count: 0,
			segments: input.segments,
			roads: input.roads,
			cars: input.cars,
		}
	}
	pub fn create_segment(&mut self, transform: TwoPointTransform, speed_limit: f64) -> i32 {
		let segment = Segment::new(&self.count.to_string(), transform, speed_limit);
		self.count += 1;

		let mut segments = self.segments.borrow_mut();
		segments.push(segment);
		segments.len() as i32 - 1
	}

	fn build_segments_from_relative(
		&mut self,
		relative_segments: &Vec<RelativeSegment>,
		config: &SPTTransformRoadConfig,
	) -> Vec<i32> {
		let mut created_segment_indices = Vec::<i32>::new();
		for segment in relative_segments {
			let start_pt = segment.start_pt.to_absolute(&config.transform);
			let end_pt = segment.end_pt.to_absolute(&config.transform);
			let index = self.create_segment(TwoPointTransform { start_pt, end_pt }, config.speed_limit);
			created_segment_indices.push(index);
		}

		let mut global_segments = self.segments.borrow_mut();

		for i in 0..created_segment_indices.len() {
			for relative_segment_index in &relative_segments[i].nxt_segments {
				global_segments[created_segment_indices[i] as usize]
					.add_nxt_seg(created_segment_indices[*relative_segment_index as usize]);
			}

			for relative_segment_index in &relative_segments[i].prev_segments {
				global_segments[created_segment_indices[i] as usize]
					.add_prev_seg(created_segment_indices[*relative_segment_index as usize]);
			}
		}
		created_segment_indices
	}

	fn transform_road_data(
		&mut self,
		relative_data: RoadCreationIngredients,
		config: SPTTransformRoadConfig,
	) -> FactoryBuildRoadOutput {
		let created_segment_indices =
			self.build_segments_from_relative(&relative_data.relative_segments, &config);
		let mut surfaces = relative_data.relative_surfaces;
		for surface in surfaces.iter_mut() {
			surface.to_absolute(&config.transform);
		}
		let joints = relative_data
			.relative_joints
			.to_absolute(&config.transform.direction, &created_segment_indices);
		FactoryBuildRoadOutput {
			speed_limit: config.speed_limit,
			transform: config.transform,
			joints,
			draw_data: EntityDrawData { surfaces },
		}
	}

	fn add_static_road(&mut self, road: Box<dyn Road>) -> RoadReference {
		self.count += 1;

		let mut road_state = self.roads.borrow_mut();

		road_state.static_roads.push(road);
		RoadReference {
			road_type: RoadType::Static,
			index: road_state.static_roads.len() as i32 - 1,
		}
	}

	pub fn create_car(&mut self, config: CarConfig) -> i32 {
		let segments = self.segments.borrow_mut();
		let transform = segments[config.starting_segment as usize]
			.get_transform()
			.to_single_pt_transform();

		let car = Car::new(
			false,
			config.color,
			transform,
			self.count.to_string(),
			config.length,
		);

		let mut cars = self.cars.borrow_mut();
		cars.push(car);
		cars.len() as i32 - 1
	}

	pub fn create_road_cap(&mut self, config: SPTTransformRoadConfig) -> RoadReference {
		let creation_data = RoadCap::get_creation_data();
		let build_output = self.transform_road_data(creation_data, config);
		let road_cap = RoadCap {
			id: self.count.to_string(),
			road_data: RoadData {
				speed_limit: build_output.speed_limit,
				joints: build_output.joints,
			},
			entity_data: EntityData {
				transform: build_output.transform,
				draw_data: build_output.draw_data,
			},
		};
		self.add_static_road(Box::new(road_cap))
	}
}
