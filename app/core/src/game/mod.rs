use redux_rs::Store;

use crate::state::State;
use crate::actions::Actions;
use crate::reducers::create_store;

pub struct Game {
    store: Store<State, Actions>,
}

impl Game {

    pub fn new() -> Self {
        let mut game = Game{
            store: create_store() 
        };
        game.subscribe_to_store_changes();
        game
    }

    pub fn subscribe_to_store_changes(&mut self) {
        self.store.subscribe(|_s: &State| {
            println!("state updated"); 
        });
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
