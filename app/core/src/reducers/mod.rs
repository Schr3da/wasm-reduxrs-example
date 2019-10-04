pub mod game;
pub mod settings;
pub mod state;

mod utils;

use cgmath::Vector2;
use redux_rs::{combine_reducers, Store};

use crate::maps::World;
use crate::models::geometry::Size;
use crate::theme::ThemeMode;

use game::game_reducer;
use settings::settings_reducer;
use state::{state_reducer, OnChangeCallback, State};

pub static DEFAULT_WORLD_SCALE: i32 = 5;
pub static DEFAULT_TILE_SIZE: Size<i32> = Size { w: 16, h: 16 };
pub static DEFAULT_RESOLUTION: Size<i32> = Size { w: 800, h: 600 };

pub enum Actions {
    AppValidateTest(bool),
    AppSetOnChangeCallback(OnChangeCallback),
    SettingsSetThemeMode(ThemeMode),
    SettingsSetScale(i32),
    SettingsSetResolution(Size<i32>),
    GameStartNew(),
    GameHandleKeyUp(String),
    GameHandleKeyDown(String),
    GameSetElapsedTime(i32),
    GameSetGameCursor(Vector2<i32>),
    GameSetWorld(World),
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
