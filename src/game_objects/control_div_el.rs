use crate::templates::bind_models::*;
use crate::utils::entity::*;
use crate::utils::ui::*;
use crate::utils::vector_2::*;
use wasm_bindgen::JsCast;

pub struct ControlDivEl {
	id: String,
	element: web_sys::HtmlElement,
	child: Option<Box<dyn Template>>,
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
		web_sys::window()
			.unwrap()
			.document()
			.unwrap()
			.body()
			.unwrap()
			.append_child(&element)
			.unwrap();
		let control = ControlDivEl {
			id,
			element,
			child: None,
			is_displayed: false,
			opacity_animation_state: TweenState::Idle,
		};
		control
	}
	pub fn append_child(&mut self, child: Box<dyn Template>) {
		self.child = Some(child);
		self
			.element
			.set_inner_html(&self.child.as_ref().unwrap().get_template());
	}
	pub fn bind_model(&mut self, model: Option<MessageModels>) {
		if let Some(child) = self.child.as_mut() {
			child.set_bind_model(model);
		}
	}
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
		if let Some(child) = self.child.as_mut() {
			child.update();
		}
		let curr_opacity = self
			.element
			.style()
			.get_property_value("opacity")
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
						.set_property("opacity", &format!("{:.2}", curr_opacity + 0.02))
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
						.set_property("opacity", &format!("{:.2}", curr_opacity - 0.02))
						.unwrap();
				} else {
					self.is_displayed = false;
					self.opacity_animation_state = TweenState::Idle;
				}
			}
		}
	}
}
