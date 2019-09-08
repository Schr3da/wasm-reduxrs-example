use quicksilver::graphics::Color;
use quicksilver::lifecycle::Event;
use quicksilver::lifecycle::{State, Window};
use quicksilver::Error;
use quicksilver::Result;

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
        println!("{:?}", _window.update_rate());
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        Ok(())
    }
}
