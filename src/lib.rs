use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
mod controllers;
mod game_objects;
mod options;
mod scenes;
mod utils;
use controllers::simulation_controller::*;
fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap();
}
#[wasm_bindgen(start)]
pub fn run() {
    let window = web_sys::window().unwrap();
    let simulation_controller = Rc::new(RefCell::new(SimulationController::new()));
    {
        let simulation_controller = Rc::clone(&simulation_controller);
        let closure = Closure::wrap(Box::new(move |event: web_sys::WheelEvent| {
            simulation_controller
                .borrow_mut()
                .handle_zoom(event.delta_y() as i32);
        }) as Box<dyn FnMut(_)>);
        window
            .add_event_listener_with_callback("wheel", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
    {
        let simulation_controller = Rc::clone(&simulation_controller);
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let mut simulation_controller = simulation_controller.borrow_mut();
            simulation_controller.handle_mouse_move(event.client_x(), event.client_y());
        }) as Box<dyn FnMut(_)>);
        window
            .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
    {
        let closure =
            Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {}) as Box<dyn FnMut(_)>);
        window
            .add_event_listener_with_callback("mouseclick", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
    let f = Rc::new(RefCell::<Option<Closure<dyn FnMut()>>>::new(None));
    let g = Rc::clone(&f);
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let mut simulation_controller = simulation_controller.borrow_mut();
        simulation_controller.update();
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));
    request_animation_frame(g.borrow().as_ref().unwrap());
}
