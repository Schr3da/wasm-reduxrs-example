extern crate core;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement, KeyboardEvent};

use std::rc::Rc;
use std::cell::RefCell;
use crate::core::game::Game;
use crate::core::reducers::state::{State, OnChangeCallback};
use crate::core::reducers::settings::{Settings};

type SharedGameRef = Rc<RefCell<Game>>; 

fn create_canvas(instance: SharedGameRef) -> Result<HtmlCanvasElement, JsValue> { 
    let w = window().unwrap();
    let d = w.document().unwrap();
    let b = d.body().unwrap();
    let mut canvas = d.create_element("canvas")?
        .dyn_into::<HtmlCanvasElement>()?;
   
    let settings = instance.as_ref().borrow().state().next.settings;
    set_size(&mut canvas, &settings);
    
    add_listeners(&mut canvas, instance);

    b.append_child(&canvas)?;
    Ok(canvas)
}

fn set_size(canvas: &mut HtmlCanvasElement, settings: &Settings) {
    canvas.set_width(settings.resolution.w as u32);
    canvas.set_height(settings.resolution.h as u32);
}

fn add_listeners(canvas: &mut HtmlCanvasElement, instance: SharedGameRef) {
    let instance_key_down = instance.clone();
    let handle_key_down = Closure::wrap(Box::new(move | _e: KeyboardEvent | {
        instance_key_down.borrow_mut().key_down('a');
    }) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("keydown", handle_key_down.as_ref().unchecked_ref()).unwrap();
    handle_key_down.forget();

    let instance_key_up = instance.clone();
    let handle_key_up = Closure::wrap(Box::new(move | _e: KeyboardEvent | {
        instance_key_up.borrow_mut().key_up('b');
    }) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("keydown", handle_key_up.as_ref().unchecked_ref()).unwrap();
    handle_key_up.forget();
}

fn render_changes(canvas: &HtmlCanvasElement) -> OnChangeCallback {
    let context = canvas.get_context("2d").unwrap()
        .unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();

    OnChangeCallback::new(Rc::new(move |_s: &State| {
        context.begin_path();
        context.arc(75.0, 75.0, 50.0, 0.0, 3.14 * 2.0).unwrap();
        context.move_to(110.0, 75.0);
        context.arc(75.0, 75.0, 35.0, 0.0, 3.14).unwrap();
        context.move_to(65.0, 65.0);
        context.arc(60.0, 65.0, 5.0, 0.0, 3.14 * 2.0).unwrap();
        context.move_to(95.0, 65.0);
        context.arc(90.0, 65.0, 5.0, 0.0, 3.14 * 2.0).unwrap();
        context.stroke();
    }))
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let instance = Rc::new(RefCell::new(Game::new()));
    
    let canvas = create_canvas(instance.clone())?; 
    let renderer = render_changes(&canvas);
    
    instance.as_ref().borrow_mut().set_callback(renderer);

    Ok(())
}
