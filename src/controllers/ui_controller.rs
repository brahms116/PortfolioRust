use crate::game_objects::control_div_el::*;
pub struct UiController {
	menu_button_control: ControlDivEl,
	full_page_control: ControlDivEl,
	tracker_card_control: ControlDivEl,
}

impl UiController {
	pub fn new() -> UiController {
		let controller = UiController {
			menu_button_control: ControlDivEl::new("menu_button_control".into()),
			full_page_control: ControlDivEl::new("full_page_control".into()),
			tracker_card_control: ControlDivEl::new("tracker_card_control".into()),
		};
		controller
	}
	pub fn update(&mut self) {}
}
