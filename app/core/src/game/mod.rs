use redux_rs::Store;
use cgmath::Vector2;

use crate::reducers::state::State;
use crate::reducers::Actions;
use crate::reducers::create_store;

pub struct Game {
    store: Store<State, Actions>,
}

impl Game {

    pub fn new() -> Self {
        let mut game = Game{
            store: create_store() 
        };
        game.init();
        game
    }

    pub fn init(&mut self) {
        self.subscribe_to_store_changes();
    }

    pub fn subscribe_to_store_changes(&mut self) {
        self.store.subscribe(|_s: &State| { });
    }

    pub fn mouse_moved(&mut self, x: i32, y: i32) {
        self.store.dispatch(Actions::GameSetGameCursor(Vector2{ x, y }));
    }

    pub fn key_pressed(&mut self, key: char) {
        self.store.dispatch(Actions::GameHandleKey(key));
    }

    pub fn action(&mut self, action: Actions) {
        self.store.dispatch(action); 
    }

    pub fn update(&mut self, dt: f64) {
        self.store.dispatch(Actions::GameSetElapsedTime(dt));
    }

    pub fn state(&self) -> &State {
        self.store.state()
    }
}
