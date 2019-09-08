use crate::theme::{ThemeMode};
use crate::state::{State, Settings};
use crate::actions::settings::{Actions};

fn change_theme_mode(state: &State, mode: ThemeMode) -> State {
    State {
        settings: Settings {
            mode,
            ..state.settings
        },
        game: state.game.clone(),
        ..*state
    }
}

pub fn reducer(state: &State, action: &Actions) -> State {
    match action {
        Actions::ChangeThemeMode(mode) => change_theme_mode(state, *mode),
    }
}
