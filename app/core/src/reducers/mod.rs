pub mod state;
mod settings;
mod game;

use redux_rs::{Store, combine_reducers};
use cgmath::Vector2;

use crate::maps::World;
use crate::theme::ThemeMode;

use state::{State, state_reducer};
use settings::settings_reducer;
use game::game_reducer;

pub enum Actions {
    AppValidateTest(bool),
    SettingsSetThemeMode(ThemeMode),
    SettingsSetScale(i32),
    SettingsSetResolution(Vector2<i32>),
    GameSetElapsedTime(f64),
    GameSetWorld(World),
    GameSetViewForPosition(Vector2<i32>),
}

pub fn create_store() -> Store<State, Actions> {
    let reducers = combine_reducers!(
        State,
        &Actions,
        state_reducer,
        settings_reducer,
        game_reducer
    );
    Store::new(reducers, State::default())
}
