use std::default::Default;

use crate::maps::{templates, Map};
use crate::theme::ThemeMode;

#[derive(Copy, Clone, Debug)]
pub struct Settings {
    pub mode: ThemeMode,
}

#[derive(Clone)]
pub struct Game {
    pub map: Map,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            map: Map::new(templates::TEMPLE_MAP),
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            mode: ThemeMode::LIGHT,
        }
    }
}

#[derive(Default)]
pub struct State {
    pub settings: Settings,
    pub game: Game,
}
