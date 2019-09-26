extern crate core;

use std::rc::Rc;

use wasm_bindgen::prelude::*;
use crate::core::game::Game;
use crate::core::reducers::state::{State, OnChangeCallback};

use wasm_bindgen::JsCast;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    let mut game_instance = Game::new();
    let cb = OnChangeCallback::new(
        Rc::new(|s: &State| {
            web_sys::console::log_1(&s.next.game.elapsed_time.into());
        })
    );
    game_instance.set_callback(cb);

    let state = game_instance.state();

    let canvas = document.create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    canvas.set_width(state.next.settings.resolution.w as u32);
    canvas.set_height(state.next.settings.resolution.h as u32);
    document.body().unwrap().append_child(&canvas)?;

    let closure = Closure::wrap(Box::new(move |_event: web_sys::KeyboardEvent| {
        game_instance.key_pressed('a');
        web_sys::console::log_1(&"KEyboard pressed".into());
    }) as Box<dyn FnMut(_)>);

    window.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
    closure.forget();

    Ok(())
}
