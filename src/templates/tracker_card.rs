use crate::templates::bind_models::*;
use crate::utils::ui::*;
pub struct TrackerCard {
	template: String,
	bind_model: Option<MessageModels>,
}

impl TrackerCard {
	pub fn new() -> Self {
		let template = String::from(
			"
		<div
      class=\"
        bg-white
        fixed
        w-60
        h-32
        left-1/2
        top-1/2
        transform
        -translate-x-1/2 -translate-y-full
        shadow-2xl
        rounded-l
        z-10
      \"
    ></div>
		",
		);
		TrackerCard {
			template,
			bind_model: None,
		}
	}
}

impl Template for TrackerCard {
	fn get_template(&self) -> &String {
		&self.template
	}
	fn set_bind_model(&mut self, model: Option<MessageModels>) {
		self.bind_model = model;
	}
	fn update(&self) {}
}
