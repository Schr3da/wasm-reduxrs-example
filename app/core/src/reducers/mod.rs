mod game;
mod settings;

use redux_rs::Store;

use crate::actions::Actions;
use crate::state::State;

fn reducer(state: &State, action: &Actions) -> State {
    match action {
        Actions::SettingsSetThemeMode(mode) => settings::set_theme_mode(state, mode),
        Actions::GameSetMap(map) => game::set_map(state, map),
    }
}

pub fn create_store() -> Store<State, Actions> {
    Store::new(reducer, State::default())
}
