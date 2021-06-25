use crate::utils;
pub struct DemoStruct {
	origin: utils::Point,
}
impl DemoStruct {
	pub fn new(x: i32, y: i32) -> DemoStruct {
		DemoStruct {
			origin: utils::Point::new(x, y),
		}
	}
}

impl utils::Entity for DemoStruct {
	fn get_draw_data(&self) -> utils::EntityDrawData {
		let mut vert = Vec::<utils::Point>::new();
		vert.reserve(4);
		vert.push(utils::Point::new(self.origin.x, self.origin.y));
		vert.push(utils::Point::new(self.origin.x + 10, self.origin.y));
		vert.push(utils::Point::new(self.origin.x + 10, self.origin.y + 10));
		vert.push(utils::Point::new(self.origin.x, self.origin.y + 10));
		utils::EntityDrawData {
			vertices: vert,
			color: String::from("#000000"),
		}
	}
}
