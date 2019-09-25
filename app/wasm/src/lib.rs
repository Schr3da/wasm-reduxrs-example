extern crate core;

use wasm_bindgen::prelude::*;
use crate::core::game::Game;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let g = Game::new();
    g.state(); 
    web_sys::console::log_1(&"Sample".into());

    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");

    body.append_child(&val)?;

    Ok(())
}
