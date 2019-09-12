use crate::actions::Actions;
use crate::state::{Settings, State};
use crate::theme::ThemeMode;

use super::default;

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

pub fn settings_reducer(state: &State, action: &Actions) -> State {
    match action {
        Actions::SettingsSetThemeMode(m) => set_theme_mode(state, m),
        Actions::SettingsSetScale(s) => set_scale(state, s),
        _ => default(state),
    }
}
