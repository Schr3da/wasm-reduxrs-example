mod game;
mod settings;

use redux_rs::Store;

use crate::actions::Actions;
use crate::state::State;

fn reducer(state: &State, action: &Actions) -> State {
    match action {
        Actions::SettingsSetThemeMode(m) => settings::set_theme_mode(state, m),
        Actions::GameSetWorld(w) => game::set_world(state, w),
    }
}

pub fn create_store() -> Store<State, Actions> {
    Store::new(reducer, State::default())
}
