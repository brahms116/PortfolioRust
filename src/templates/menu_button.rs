use crate::templates::bind_models::*;
use crate::utils::ui::*;

pub struct MenuButton {
  template: String,
  menu_icon: String,
  close_icon: String,
  bind_model: Option<MessageModels>,
}

impl MenuButton {
  pub fn new() -> MenuButton {
    let menu_icon = String::from(
      "
      <svg
        xmlns=\"http://www.w3.org/2000/svg\"
        class=\"h-6 w-6\"
        fill=\"none\"
        viewBox=\"0 0 24 24\"
        stroke=\"#F9FAFB\"
      >
        <path
          stroke-linecap=\"round\"
          stroke-linejoin=\"round\"
          stroke-width=\"2\"
          d=\"M4 6h16M4 12h16M4 18h16\"
        />
      </svg>
    ",
    );
    let close_icon = String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" class=\"h-6 w-6\" fill=\"none\" viewBox=\"0 0 24 24\" stroke=\"currentColor\">
      <path stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"2\" d=\"M6 18L18 6M6 6l12 12\" />
    </svg>
    ");

    let template = String::from(
      "
		<div
      class=\"
        fixed
        bottom-20
        w-12
        h-12
        left-1/2
        transform
        -translate-x-1/2
        bg-blue-600
        flex
        items-center
        justify-center
        rounded-full
        shadow-2xl
        cursor-pointer
        hover:bg-blue-500
      \"
			id=\"icon_button_wrapper\"
    ></div>",
    );
    MenuButton {
      template,
      menu_icon,
      close_icon,
      bind_model: None,
    }
  }
}

impl Template for MenuButton {
  fn get_template(&self) -> &String {
    &self.template
  }
  fn set_bind_model(&mut self, model: Option<MessageModels>) {
    self.bind_model = model;
  }
  fn update(&self) {
    if let Some(message) = &self.bind_model {
      match message {
        MessageModels::MenuButton(model) => {
          let document = web_sys::window().unwrap().document().unwrap();
          let element = document.get_element_by_id("icon_button_wrapper").unwrap();
          element.set_inner_html(if model.is_menu_open {
            &self.close_icon
          } else {
            &self.menu_icon
          });
        }
      }
    }
  }
}
