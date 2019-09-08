use crate::theme::{ThemeMode};
use crate::state::{State, Settings};

pub  fn set_theme_mode(state: &State, mode: &ThemeMode) -> State {
    State {
        settings: Settings {
            mode: *mode,
            ..state.settings
        },
        game: state.game.clone(),
        ..*state
    }
}

