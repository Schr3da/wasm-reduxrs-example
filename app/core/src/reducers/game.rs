use crate::state::{State, Game};
use crate::maps::{Map};

pub fn set_map(state: &State, map: &Map) -> State {
    State {
        game: Game {
            map: map.clone(),
            ..state.game.clone()
        },
        ..*state
    }
}

