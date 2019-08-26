use quicksilver::{Result};
use quicksilver::lifecycle::{State, Window};
use quicksilver::graphics::{Color};

pub struct Canvas;

impl State for Canvas {
    fn new() -> Result<Canvas> {
        Ok(Canvas)
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        Ok(())
    }
}

