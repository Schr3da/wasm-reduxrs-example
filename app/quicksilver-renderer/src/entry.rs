use quicksilver::{Error};
use quicksilver::{Result};
use quicksilver::lifecycle::{State, Window};
use quicksilver::lifecycle::{Event};

pub struct Canvas; 

impl State for Canvas {
    fn new() -> Result<Canvas> {
        Ok(Canvas)
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

    fn draw(&mut self, _window: &mut Window) -> Result<()> {
        Ok(())
    }
}

