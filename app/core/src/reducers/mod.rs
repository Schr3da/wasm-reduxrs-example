mod game;
mod settings;

use redux_rs::{Store, combine_reducers};
use cgmath::Vector2;

use crate::maps::World;
use crate::theme::ThemeMode;

use settings::{Settings, settings_reducer};
use game::{Game, game_reducer};

pub enum Actions {
    SettingsSetThemeMode(ThemeMode),
    SettingsSetScale(i32),
    SettingsSetResolution(Vector2<i32>),
    GameSetElapsedTime(f64),
    GameSetWorld(World),
    GameSetViewPort(Vector2<i32>),
}

#[derive(Default, Clone)]
pub struct State {
    pub settings: Settings,
    pub game: Game,
}

pub fn default(state: &State) -> State {
    State {
        game: state.game.clone(),
        ..*state
    }
}

pub fn create_store() -> Store<State, Actions> {
    let reducers = combine_reducers!(
        State,
        &Actions,
        settings_reducer,
        game_reducer
    );
    Store::new(reducers, State::default())
}
