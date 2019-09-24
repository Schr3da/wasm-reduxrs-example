use redux_rs::Store;

use crate::reducers::state::State;
use crate::reducers::Actions;
use crate::reducers::create_store;
use cgmath::Vector2;

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
        self.store.dispatch(Actions::GameSetViewForPosition(Vector2{x: 0, y: 0}));
    }

    pub fn subscribe_to_store_changes(&mut self) {
        self.store.subscribe(|_s: &State| { });
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
