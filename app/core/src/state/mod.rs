use std::default::Default;
use std::collections::HashMap;

use crate::maps::{Tile, World, templates};
use crate::theme::ThemeMode;

#[derive(Copy, Clone, Debug)]
pub struct Settings {
    pub mode: ThemeMode,
    pub scale: i32,
}

#[derive(Clone)]
pub struct Game {
    pub elapsed_time: f64,
    pub world: World,
    pub viewport: HashMap<&'static str, Option<Tile>>,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            elapsed_time: 0.,
            world: World::new(templates::TEMPLE_MAP, 32),
            viewport: HashMap::new(),
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            mode: ThemeMode::LIGHT,
            scale: 1,
        }
    }
}

#[derive(Default, Clone)]
pub struct State {
    pub settings: Settings,
    pub game: Game,
}
