use crate::theme::ThemeMode;
use crate::models::geometry::Size;

use super::state::{State, default, next};
use super::{Actions, DEFAULT_TILE_SIZE, DEFAULT_RESOLUTION};

#[derive(Copy, Clone, Debug)]
pub struct Settings {
    pub mode: ThemeMode,
    pub scale: i32,
    pub default_tile_size: Size<i32>,
    pub resolution: Size<i32>, 
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            default_tile_size: DEFAULT_TILE_SIZE,
            resolution: DEFAULT_RESOLUTION,
            mode: ThemeMode::LIGHT,
            scale: 1,
        }
    }
}

fn set_theme_mode(state: &State, mode: &ThemeMode) -> State {
    let mut next_state = next(state);
    next_state.next.settings.mode = *mode;
    next_state    
}

fn set_scale(state: &State, scale: &i32) -> State {
    let mut next_state = next(state);
    next_state.next.settings.scale = *scale;
    next_state    
}

fn set_resolution(state: &State, resolution: &Size<i32>) -> State {
    let mut next_state = next(state);
    next_state.next.settings.resolution = *resolution;
    next_state    
}

pub fn settings_reducer(state: &State, action: &Actions) -> State {
    match action {
        Actions::SettingsSetThemeMode(m) => set_theme_mode(state, m),
        Actions::SettingsSetScale(s) => set_scale(state, s),
        Actions::SettingsSetResolution(r) => set_resolution(state, r),
        _ => default(state),
    }
}
