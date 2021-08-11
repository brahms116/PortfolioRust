use crate::game_objects::car::*;
use crate::game_objects::factory::Factory;
use crate::game_objects::factory::FactoryCreationInput;
use crate::game_objects::roads::RoadState;
use crate::game_objects::segment::Segment;
use crate::utils::car::CarConfig;
use crate::utils::direction::*;
use crate::utils::entity::*;
use crate::utils::road::BasicRoadConfig;
use crate::utils::transform::SinglePointTransform;
use crate::utils::vector_2::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct SimulationScene {
	cars: Rc<RefCell<Vec<Car>>>,
	roads: Rc<RefCell<RoadState>>,
	segments: Rc<RefCell<Vec<Segment>>>,
	factory: Factory,
}

impl SimulationScene {
	pub fn new() -> SimulationScene {
		let cars = Rc::new(RefCell::new(Vec::<Car>::new()));
		let segments = Rc::new(RefCell::new(Vec::<Segment>::new()));
		let road_state = Rc::new(RefCell::new(RoadState::new()));
		let mut scene = SimulationScene {
			cars: Rc::clone(&cars),
			roads: Rc::clone(&road_state),
			segments: Rc::clone(&segments),
			factory: Factory::new(FactoryCreationInput {
				cars,
				segments,
				roads: road_state,
			}),
		};
		scene.factory.create_road_cap(BasicRoadConfig {
			speed_limit: 6.0,
			transform: SinglePointTransform {
				direction: Direction::N,
				position: Vec2::new(0.0, 0.0),
			},
		});
		scene.factory.create_car(CarConfig {
			color: String::from("#e32222"),
			starting_segment: 1,
			length: 8,
		});
		scene
	}
	pub fn render_entities(&self, drawer: &dyn Drawer) {
		let roads = self.roads.borrow();
		let cars = self.cars.borrow();
		for data in &roads.get_draw_data() {
			drawer.draw(data);
		}
		for car in cars.iter() {
			drawer.draw(car.get_draw_data());
		}
	}
}

impl Dynamic for SimulationScene {
	fn update(&mut self) {
		{
			let mut cars = self.cars.borrow_mut();
			for car in cars.iter_mut() {
				car.update();
			}
		}
	}
}
