extern crate core;

mod utils;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, window, CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};

use crate::core::game::Game;
use crate::core::reducers::settings::Settings;
use crate::core::reducers::state::{OnChangeCallback, State};
use utils::{draw_cursor_tile, draw_cursor, draw_world};

type SharedGameRef = Rc<RefCell<Game>>;

fn create_canvas(instance: SharedGameRef) -> Result<HtmlCanvasElement, JsValue> {
    let w = window().unwrap();
    let d = w.document().unwrap();
    let b = d.body().unwrap();
    let mut canvas = d
        .create_element("canvas")?
        .dyn_into::<HtmlCanvasElement>()?;

    let settings = instance.as_ref().borrow().state().next.settings;
    set_size(&mut canvas, &settings);

    b.append_child(&canvas)?;
    Ok(canvas)
}

fn set_size(canvas: &mut HtmlCanvasElement, settings: &Settings) {
    canvas.set_width(settings.resolution.w as u32);
    canvas.set_height(settings.resolution.h as u32);
}

fn add_listeners(instance: SharedGameRef) {
    let window = window().unwrap();

    let instance_key_down = instance.clone();
    let handle_key_down = Closure::wrap(Box::new(move |e: KeyboardEvent| {
        
        console::log_1(&e.key().into());
        instance_key_down.borrow_mut().key_down(e.key());
    }) as Box<dyn FnMut(_)>);
    window
        .add_event_listener_with_callback("keydown", handle_key_down.as_ref().unchecked_ref())
        .unwrap();
    handle_key_down.forget();

    let instance_key_up = instance.clone();
    let handle_key_up = Closure::wrap(Box::new(move |e: KeyboardEvent| {
        instance_key_up.borrow_mut().key_up(e.key());
    }) as Box<dyn FnMut(_)>);
    window
        .add_event_listener_with_callback("keydown", handle_key_up.as_ref().unchecked_ref())
        .unwrap();
    handle_key_up.forget();
}

fn update(instance: SharedGameRef) -> Result<i32, JsValue> {
    let game = instance.clone();
    let cb = Closure::wrap(Box::new(move || game.as_ref().borrow_mut().update()) as Box<dyn Fn()>);

    let state = instance.as_ref().borrow().state().clone();
  
    let id = window()
        .unwrap()
        .set_interval_with_callback_and_timeout_and_arguments_0(
            cb.as_ref().unchecked_ref(),
            state.next.settings.update_interval,
        )?;
    cb.forget();

    Ok(id)
}

fn render(canvas: &HtmlCanvasElement) -> OnChangeCallback {
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    OnChangeCallback::new(Rc::new(move |s: &State| {
        let resolution = s.next.settings.resolution;
        context.clear_rect(0.0, 0.0, resolution.w as f64, resolution.h as f64);

        draw_world(&context, s);
        draw_cursor(&context, s);
        draw_cursor_tile(&context, s)
    }))
}

#[wasm_bindgen(start)]
#[allow(unused_variables)]
pub fn main() -> Result<(), JsValue> {
    let instance = Rc::new(RefCell::new(Game::new()));
    instance.as_ref().borrow_mut().start_new_game();

    let canvas = create_canvas(instance.clone())?;
    let interval = update(instance.clone());
    let renderer = render(&canvas);

    instance.as_ref().borrow_mut().set_callback(renderer);
    add_listeners(instance);

    Ok(())
}
