use crate::utils::*;
pub struct Triangle {
	origin: Vec2,
	pt1: Vec2,
	pt2: Vec2,
	l_value: f64,
	is_l_increasing: bool,
}
impl Triangle {
	pub fn new(origin: Vec2, pt1: Vec2, pt2: Vec2) -> Triangle {
		Triangle {
			origin,
			pt1,
			pt2,
			l_value: 50.0 * rand::random::<f64>() + 25.0,
			is_l_increasing: true,
		}
	}
}
impl Entity for Triangle {
	fn get_draw_data(&self) -> EntityDrawData {
		let mut vert = Vec::<Vec2>::new();
		vert.reserve(3);
		vert.push(self.origin);
		vert.push(self.pt1);
		vert.push(self.pt2);
		EntityDrawData {
			vertices: vert,
			color: format!("HSL({:.2},34%,{:.20}%", self.l_value + 170.0, self.l_value),
			// color: "#000000".into(),
		}
	}
}
impl Dynamic for Triangle {
	fn update(&mut self) {
		if self.l_value > 75.0 {
			self.is_l_increasing = false;
		} else if self.l_value < 25.0 {
			self.is_l_increasing = true;
		}
		self.l_value += if self.is_l_increasing { 0.5 } else { -0.5 }
	}
}
impl DynamicEntity for Triangle {}
