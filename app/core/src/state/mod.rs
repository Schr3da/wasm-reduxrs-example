use std::default::{Default};

use crate::theme::{ThemeMode};
use crate::maps::{Map, temple};

pub struct Settings {
    pub mode: ThemeMode,
}

#[derive(Clone)]
pub struct Game {
    pub  map: Map, 
}

impl Default for Game {
    fn default() -> Self {
        Game{
           map: temple::new(),
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings{
            mode: ThemeMode::LIGHT,
        }
    }
}

#[derive(Default)]
pub struct State {
    pub settings: Settings,
    pub game: Game,
}


