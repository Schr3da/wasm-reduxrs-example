use quicksilver::graphics::{Background::Col, Color};
use quicksilver::lifecycle::{State, Window};
use quicksilver::geom::Rectangle;
use quicksilver::lifecycle::Event;
use quicksilver::input::Key;
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

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        if window.keyboard()[Key::Escape].is_down() {
            window.close();
        }

        match event {
            Event::Typed(k) => self.game.key_pressed(*k),
            Event::MouseMoved(c) => self.game.mouse_moved(c.x as i32, c.y as i32),
            _ => { },
        }

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
       
        let state = self.game.state();
        for tiles in state.next.game.views.values() {
            for t in tiles {
                match t {
                    Some(t) => { 
                        window.draw(&Rectangle::new((t.position.x, t.position.y), (t.size.w, t.size.h)), Col(Color::BLUE));
                    },
                    None => println!("Not a valid tile"),
                }
            }
        }
        
        Ok(())
    }
}
