use crate::utils;
pub struct DemoStruct {
	origin: utils::Vec2,
}
impl DemoStruct {
	pub fn new(x: f64, y: f64) -> DemoStruct {
		DemoStruct {
			origin: utils::Vec2::new(x, y),
		}
	}
}

impl utils::Entity for DemoStruct {
	fn get_draw_data(&self) -> utils::EntityDrawData {
		let mut vert = Vec::<utils::Vec2>::new();
		vert.reserve(4);
		vert.push(utils::Vec2::new(self.origin.x, self.origin.y));
		vert.push(utils::Vec2::new(self.origin.x + 24.0, self.origin.y));
		vert.push(utils::Vec2::new(self.origin.x + 24.0, self.origin.y + 14.0));
		vert.push(utils::Vec2::new(self.origin.x, self.origin.y + 54.0));
		utils::EntityDrawData {
			vertices: vert,
			color: String::from("#000000"),
		}
	}
}
