use crate::templates::bind_models::*;

pub enum TweenState {
	Idle,
	Increasing,
	Decreasing,
}

pub trait Template {
	fn update(&self) -> ();
	fn get_template(&self) -> &String;
	fn set_bind_model(&mut self, model: Option<MessageModels>);
}
