use redux_rs::{Subscription};

use crate::reducers::{create_store};
use crate::state::{State};
use crate::theme::{ThemeMode};
use crate::actions::{Actions};
use crate::maps::{temple};

use super::maps::{test_map};

#[test]
pub fn test_settings_set_theme() {
    let mut store = create_store();
    let listener: Subscription<State> = |state: &State| {
        assert_eq!(ThemeMode::DARK, state.settings.mode);
    };
    store.subscribe(listener); 
    store.dispatch(Actions::SettingsSetThemeMode(ThemeMode::DARK));
}

#[test]
pub fn test_game_set_map() {
    let mut store = create_store();
    let listener: Subscription<State> = |state: &State| {
        let template = state.game.map.template;
        test_map(template, state.game.map.clone()); 
        assert!(true);
    };
    store.subscribe(listener);

    let map = temple::new(); 
    store.dispatch(Actions::GameSetMap(map));
}

