use redux_rs::Subscription;
use cgmath::Vector2;

use crate::maps::{World, templates};
use crate::reducers::{Actions, create_store};
use crate::reducers::state::State;
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

    let world = World::new(templates::TEMPLE_MAP, 1);
    store.dispatch(Actions::GameSetWorld(world));
}

#[test]
pub fn test_game_set_view_for_position() {
    let mut store = create_store();
    
    let listener: Subscription<State> = |state: &State| {
        if (state.validate_test == false) {
            return;
        }
        
        assert!(state.game.world.tiles.len() > 0);
        assert_eq!(10, state.game.view_position.x); 
        assert_eq!(10, state.game.view_position.y); 
    };
    store.subscribe(listener);

    let world = World::new(templates::TEMPLE_MAP, 1);
    store.dispatch(Actions::GameSetWorld(world));
    store.dispatch(Actions::GameSetViewForPosition(Vector2::new(10, 10)));
}
