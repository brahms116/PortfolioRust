use crate::game_objects::control_div_el::*;
use crate::templates::menu_button::*;
use crate::utils::*;
pub struct UiController {
	menu_button_control: ControlDivEl,
	full_page_control: ControlDivEl,
	tracker_card_control: ControlDivEl,
}

impl UiController {
	pub fn new() -> UiController {
		let mut menu_button_control = ControlDivEl::new("menu_button_control".into());
		let menu_button: Box<dyn Template> = Box::new(MenuButton::new());
		menu_button_control.append_child(menu_button);
		menu_button_control.show();
		let controller = UiController {
			menu_button_control,
			full_page_control: ControlDivEl::new("full_page_control".into()),
			tracker_card_control: ControlDivEl::new("tracker_card_control".into()),
		};
		controller
	}
	pub fn update(&mut self) {
		// self.menu_button_control.update();
	}
}
