use quicksilver::{Result};
use quicksilver::lifecycle::{State, Window};
use quicksilver::graphics::{Color};

use crate::common::theme::{Theme, LightTheme};

pub struct Canvas;

impl State for Canvas {
    fn new() -> Result<Canvas> {
        Ok(Canvas)
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        let color = Color::from_hex(LightTheme::color_background()); 
        window.clear(color)?;
        Ok(())
    }
}

