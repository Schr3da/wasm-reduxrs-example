use cgmath::Vector2;
use redux_rs::Store;

use crate::reducers::create_store;
use crate::reducers::state::{OnChangeCallback, State};
use crate::reducers::Actions;

pub struct Game {
    store: Store<State, Actions>,
}

impl Game {
    pub fn new() -> Self {
        let mut game = Game {
            store: create_store(),
        };
        game.init();
        game
    }

    pub fn init(&mut self) {
        self.subscribe_to_store_changes();
    }

    pub fn action(&mut self, action: Actions) {
        self.store.dispatch(action);
    }

    pub fn start_new_game(&mut self) {
        self.action(Actions::GameStartNew());
    }

    pub fn set_callback(&mut self, cb: OnChangeCallback) {
        self.action(Actions::AppSetOnChangeCallback(cb));
    }

    pub fn mouse_moved(&mut self, x: i32, y: i32) {
        self.action(Actions::GameSetGameCursor(Vector2 { x, y }));
    }

    pub fn key_up(&mut self, key: String) {
        self.action(Actions::GameHandleKeyUp(key))
    }

    pub fn key_down(&mut self, key: String) {
        self.action(Actions::GameHandleKeyDown(key));
    }

    pub fn update(&mut self, dt: f64) {
        self.action(Actions::GameSetElapsedTime(dt));
    }

    pub fn state(&self) -> &State {
        self.store.state()
    }

    fn subscribe_to_store_changes(&mut self) {
        self.store.subscribe(|s: &State| (s.on_change.cb)(s));
    }
}
