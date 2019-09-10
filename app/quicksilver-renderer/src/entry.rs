use quicksilver::graphics::Color;
use quicksilver::lifecycle::Event;
use quicksilver::lifecycle::{State, Window};
use quicksilver::Error;
use quicksilver::Result;

use core::state;
use core::redux_rs::Store;
use core::actions::Actions;
use core::reducers::create_store;




pub struct Canvas {
    store: Store<core::state::State, Actions>, 
}

impl Canvas {
    pub fn subscribe_to_store_changes(&mut self) {
        self.store.subscribe(|_s: &state::State| {
            println!("state updated"); 
        });
    }
}

impl State for Canvas {
    fn new() -> Result<Canvas> {
        let mut c = Canvas{
            store: create_store()
        };
        c.subscribe_to_store_changes();
        Ok(c)
    }

    fn event(&mut self, _event: &Event, _window: &mut Window) -> Result<()> {
        Ok(())
    }

    fn handle_error(error: Error) {
        println!("{:?}", error);
    }

    fn update(&mut self, _window: &mut Window) -> Result<()> {
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        Ok(())
    }
}
