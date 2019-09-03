extern crate redux_rs;

pub mod theme;
pub mod maps;
pub mod actions;
pub mod reducers;
pub mod state;

use redux_rs::{Subscription};
use actions::{settings};
use theme::{ThemeMode};
use state::{State};

pub fn init() {
    let mut store = reducers::combine_reducer();

    let listener: Subscription<State> = |state: &State| {
        println!("Theme changed {:?}", state.settings.mode);
    };

    store.subscribe(listener); 

    store.dispatch(settings::Actions::ChangeThemeMode(ThemeMode::DARK));
}
