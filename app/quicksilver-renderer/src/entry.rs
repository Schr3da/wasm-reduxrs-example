use quicksilver::graphics::Color;
use quicksilver::lifecycle::Event;
use quicksilver::lifecycle::{State, Window};
use quicksilver::Error;
use quicksilver::Result;

use core::game::Game;

pub struct Canvas {
    game: Game,
}

impl State for Canvas {
    fn new() -> Result<Canvas> {
        let c = Canvas{ game: Game::new() };
        Ok(c)
    }

    fn event(&mut self, _event: &Event, _window: &mut Window) -> Result<()> {
        Ok(())
    }

    fn handle_error(error: Error) {
        println!("{:?}", error);
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        self.game.update(window.update_rate());
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        Ok(())
    }
}
