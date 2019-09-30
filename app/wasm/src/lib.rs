extern crate core;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement, KeyboardEvent};

use crate::core::game::Game;
use crate::core::reducers::settings::Settings;
use crate::core::reducers::state::{OnChangeCallback, State};
use std::cell::RefCell;
use std::rc::Rc;

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
    let handle_key_down = Closure::wrap(Box::new(move |_e: KeyboardEvent| {
        instance_key_down.borrow_mut().key_down('a');
    }) as Box<dyn FnMut(_)>);
    window
        .add_event_listener_with_callback("keydown", handle_key_down.as_ref().unchecked_ref())
        .unwrap();
    handle_key_down.forget();

    let instance_key_up = instance.clone();
    let handle_key_up = Closure::wrap(Box::new(move |_e: KeyboardEvent| {
        instance_key_up.borrow_mut().key_up('b');
    }) as Box<dyn FnMut(_)>);
    window
        .add_event_listener_with_callback("keydown", handle_key_up.as_ref().unchecked_ref())
        .unwrap();
    handle_key_up.forget();
}

fn render_changes(canvas: &HtmlCanvasElement) -> OnChangeCallback {
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    OnChangeCallback::new(Rc::new(move |s: &State| {
        for tiles in s.next.game.views.values() {
            for tile in tiles {
                match tile {
                    Some(t) => {
                        context.begin_path();
                        context.set_fill_style(&"transparent".into());
                        context.fill_rect(
                            t.position.x as f64,
                            t.position.y as f64,
                            t.size.w as f64,
                            t.size.h as f64,
                        );
                        context.fill();
/*
                        context.begin_path();
                        context.set_stroke_style(&"blue".into());
                        context.move_to(t.position.x as f64, t.position.y as f64);
                        context.line_to(t.position.x as f64, t.position.y as f64 + t.size.w as f64);
                        context.line_to(
                            t.position.x as f64 + t.size.w as f64,
                            t.position.y as f64 + t.size.h as f64,
                        );
                        context.line_to(t.position.x as f64, t.position.y as f64 + t.size.h as f64);
                        context.stroke();
*/
                        context.set_fill_style(&"black".into());
                        context.set_stroke_style(&"blue".into());
                        context.set_text_align(&"center");
                        context.set_font(&"10px Arial");
                        context.fill_text(&t.symbol.to_string(), t.position.x as f64 + 8.0, t.position.y as f64 + 12.0).unwrap();
                    }
                    _ => println!("not a valid tile"),
                };
            }
        }
    }))
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let instance = Rc::new(RefCell::new(Game::new()));

    let canvas = create_canvas(instance.clone())?;
    let renderer = render_changes(&canvas);

    instance.as_ref().borrow_mut().set_callback(renderer);
    add_listeners(instance);

    Ok(())
}
