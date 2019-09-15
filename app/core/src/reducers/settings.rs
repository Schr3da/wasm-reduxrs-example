use cgmath::Vector2;
use crate::theme::ThemeMode;

use super::default;
use super::Actions;
use super::State;

#[derive(Copy, Clone, Debug)]
pub struct Settings {
    pub mode: ThemeMode,
    pub scale: i32,
    pub resolution: Vector2<i32>, 
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            resolution: Vector2::new(800, 600),
            mode: ThemeMode::LIGHT,
            scale: 1,
        }
    }
}

fn set_theme_mode(state: &State, mode: &ThemeMode) -> State {
    State {
        settings: Settings {
            mode: *mode,
            ..state.settings
        },
        game: state.game.clone(),
        ..*state
    }
}

fn set_scale(state: &State, scale: &i32) -> State {
    State {
        settings: Settings {
            scale: *scale,
            ..state.settings
        },
        game: state.game.clone(),
        ..*state
    }
}

fn set_resolution(state: &State, resolution: &Vector2<i32>) -> State {
    State {
        settings: Settings {
            resolution: *resolution,
            ..state.settings
        },
        game: state.game.clone(),
        ..*state
    }
}

pub fn settings_reducer(state: &State, action: &Actions) -> State {
    match action {
        Actions::SettingsSetThemeMode(m) => set_theme_mode(state, m),
        Actions::SettingsSetScale(s) => set_scale(state, s),
        Actions::SettingsSetResolution(r) => set_resolution(state, r),
        _ => default(state),
    }
}
