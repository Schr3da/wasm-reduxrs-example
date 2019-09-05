use redux_rs::{Subscription};

use crate::state::{State};
use crate::theme::{ThemeMode};
use crate::reducers::{combine_reducer};
use crate::actions::settings::{Actions};

#[test]
pub fn change_theme() {
    let mut store = combine_reducer();
    let listener: Subscription<State> = |state: &State| {
        assert_eq!(ThemeMode::DARK, state.settings.mode);
    };
    store.subscribe(listener); 
    store.dispatch(Actions::ChangeThemeMode(ThemeMode::DARK));
}



