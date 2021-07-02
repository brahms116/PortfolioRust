pub struct MenuButtonModel {
	pub is_menu_open: bool,
}

pub enum MessageModels {
	MenuButton(MenuButtonModel),
}
