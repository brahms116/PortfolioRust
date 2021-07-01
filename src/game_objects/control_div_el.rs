use crate::utils::*;
use wasm_bindgen::JsCast;
pub struct ControlDivEl {
	id: String,
	element: web_sys::HtmlElement,
	child: Option<web_sys::HtmlElement>,
	is_displayed: bool,
	opacity_animation_state: TweenState,
}
impl ControlDivEl {
	pub fn new(id: String) -> ControlDivEl {
		let element: web_sys::HtmlElement = web_sys::window()
			.unwrap()
			.document()
			.unwrap()
			.create_element("div")
			.unwrap()
			.dyn_into::<web_sys::HtmlElement>()
			.unwrap();
		element.set_id(&id);
		element.style().set_property("display", "none").unwrap();
		element.style().set_property("opacity", "0").unwrap();
		let control = ControlDivEl {
			id,
			element,
			child: None,
			is_displayed: false,
			opacity_animation_state: TweenState::Idle,
		};
		control
	}
	pub fn append_child() {}
	pub fn show(&mut self) {
		self
			.element
			.style()
			.set_property("display", "block")
			.unwrap();
		self.element.style().set_property("opacity", "0").unwrap();

		self.opacity_animation_state = TweenState::Increasing;
		self.is_displayed = true;
	}
	pub fn hide(&mut self) {
		self.opacity_animation_state = TweenState::Decreasing;
	}
	pub fn update(&mut self) {
		let curr_opacity = self
			.element
			.style()
			.get_property_value("opcaity")
			.unwrap()
			.parse::<f64>()
			.unwrap();
		match self.opacity_animation_state {
			TweenState::Idle => {}
			TweenState::Increasing => {
				if curr_opacity < 1.0 {
					self
						.element
						.style()
						.set_property("opacity", &format!("{:.2}", curr_opacity + 1.0))
						.unwrap();
				} else {
					self.is_displayed = true;
					self.opacity_animation_state = TweenState::Idle;
				}
			}
			TweenState::Decreasing => {
				if curr_opacity > 0.0 {
					self
						.element
						.style()
						.set_property("opacity", &format!("{:.2}", curr_opacity - 1.0))
						.unwrap();
				} else {
					self.is_displayed = false;
					self.opacity_animation_state = TweenState::Idle;
				}
			}
		}
	}
}
