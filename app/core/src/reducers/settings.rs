use crate::state::{Settings, State};
use crate::theme::ThemeMode;

pub fn set_theme_mode(state: &State, mode: &ThemeMode) -> State {
    State {
        settings: Settings {
            mode: *mode,
            ..state.settings
        },
        game: state.game.clone(),
        ..*state
    }
}
