use crate::game_objects::control_div_el::*;
// use crate::templates::bind_models::*;
use crate::templates::menu_button::*;
use crate::templates::tracker_card::*;
use crate::utils::ui::*;
pub struct UiController {
	menu_button_control: ControlDivEl,
	full_page_control: ControlDivEl,
	tracker_card_control: ControlDivEl,
}

impl UiController {
	pub fn new() -> UiController {
		let mut menu_button_control = ControlDivEl::new("menu_button_control".into());
		let menu_button: Box<dyn Template> = Box::new(MenuButton::new());
		let mut tracker_card_control = ControlDivEl::new("tracker_card_control".into());
		let tracker_card: Box<dyn Template> = Box::new(TrackerCard::new());
		tracker_card_control.append_child(tracker_card);
		menu_button_control.append_child(menu_button);
		let controller = UiController {
			menu_button_control,
			full_page_control: ControlDivEl::new("full_page_control".into()),
			tracker_card_control,
		};
		controller
	}
	pub fn update(&mut self) {
		self.menu_button_control.update();
		self.tracker_card_control.update();
		self.full_page_control.update();
	}
}
