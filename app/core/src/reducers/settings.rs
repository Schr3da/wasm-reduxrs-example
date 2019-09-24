use cgmath::Vector2;
use crate::theme::ThemeMode;

use super::state::{State, AppState, default};
use super::Actions;

#[derive(Copy, Clone, Debug)]
pub struct Settings {
    pub mode: ThemeMode,
    pub scale: i32,
    pub default_tile_size: Vector2<i32>,
    pub resolution: Vector2<i32>, 
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            default_tile_size: Vector2::new(16, 16),
            resolution: Vector2::new(800, 600),
            mode: ThemeMode::LIGHT,
            scale: 1,
        }
    }
}

fn set_theme_mode(state: &State, mode: &ThemeMode) -> State {
    State {
        prev: state.next.clone(),
        next: AppState{
            game: state.next.game.clone(),
            settings: Settings {
                mode: *mode,
                ..state.next.settings
            },
        },
        ..*state
    }
}

fn set_scale(state: &State, scale: &i32) -> State {
    State {
        prev: state.next.clone(),
        next: AppState{
            game: state.next.game.clone(),
            settings: Settings {
                scale: *scale,
                ..state.next.settings
            },
        },
        ..*state
    }
}

fn set_resolution(state: &State, resolution: &Vector2<i32>) -> State {
    State {
        prev: state.next.clone(),
        next: AppState{
            game: state.next.game.clone(),
            settings: Settings {
                resolution: *resolution,
                ..state.next.settings
            },
        },
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
