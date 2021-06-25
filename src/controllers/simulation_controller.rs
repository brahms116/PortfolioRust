use crate::game_objects::camera::Camera;

pub struct SimulationController<'a> {
	camera: &'a mut Camera,
}
impl<'a> SimulationController<'a> {
	pub fn new(camera: &'a mut Camera) -> SimulationController<'a> {
		SimulationController { camera }
	}
}
