use crate::maps::Map;
use crate::state::{Game, State};

pub fn set_map(state: &State, map: &Map) -> State {
    State {
        game: Game {
            map: map.clone(),
            ..state.game.clone()
        },
        ..*state
    }
}
