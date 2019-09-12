mod game;
mod settings;

use redux_rs::{Store, combine_reducers};
use crate::actions::Actions;
use crate::state::State;

use settings::settings_reducer;
use game::game_reducer;

pub fn default(state: &State) -> State {
    State {
        game: state.game.clone(),
        ..*state
    }
}

pub fn create_store() -> Store<State, Actions> {
    let reducers = combine_reducers!(
        State,
        &Actions,
        settings_reducer,
        game_reducer
    );
    
    Store::new(reducers, State::default())
}
