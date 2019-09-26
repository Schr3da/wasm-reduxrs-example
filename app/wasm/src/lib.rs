extern crate core;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, window, HtmlCanvasElement, KeyboardEvent};

use std::rc::Rc;
use crate::core::game::Game;
use crate::core::reducers::state::{State, OnChangeCallback};
use crate::core::reducers::settings::{Settings};

fn create_canvas(settings: &Settings) -> Result<HtmlCanvasElement, JsValue> { 
    let w = window().unwrap();
    let d = w.document().unwrap();
    let b = d.body().unwrap();

    let mut canvas = d.create_element("canvas")?
        .dyn_into::<HtmlCanvasElement>()?;
   
    set_size(&mut canvas, settings);
    add_listeners(&mut canvas);

    b.append_child(&canvas)?;
    Ok(canvas)
}

fn set_size(canvas: &mut HtmlCanvasElement, settings: &Settings) {
    canvas.set_width(settings.resolution.w as u32);
    canvas.set_height(settings.resolution.h as u32);
}

fn add_listeners(canvas: &mut HtmlCanvasElement) {
    let handle_key_down = Closure::wrap(Box::new(move | _e: KeyboardEvent | {
        //game.key_pressed('a');
        console::log_1(&"KEyboard pressed".into());
    }) as Box<dyn FnMut(_)>);

    canvas.add_event_listener_with_callback("keydown", handle_key_down.as_ref().unchecked_ref()).unwrap();
    handle_key_down.forget();
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
    let mut game_instance = Game::new();
    let state = game_instance.state();
    
    let canvas = create_canvas(&state.next.settings)?; 
    let renderer = render_changes(&canvas);
    
    game_instance.set_callback(renderer);

    Ok(())
}
