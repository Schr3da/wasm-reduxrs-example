use redux_rs::Subscription;

use crate::maps::{World, templates};
use crate::reducers::{Actions, State, create_store};
use crate::theme::ThemeMode;

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
        assert!(state.game.world.map.tiles.len() > 0); 
        assert!(state.game.world.tiles.len() > 0);
    };
    store.subscribe(listener);

    let world = World::new(templates::TEMPLE_MAP, 32);
    store.dispatch(Actions::GameSetWorld(world));
}
