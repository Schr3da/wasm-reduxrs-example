use std::default::{Default};
use crate::theme::{ThemeMode};

pub struct Settings {
    pub mode: ThemeMode,
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
}


