use crate::maps::World;
use crate::state::{Game, State};

pub fn set_world(state: &State, world: &World) -> State {
    State {
        game: Game {
            world: world.clone(),
            ..state.game.clone()
        },
        ..*state
    }
}
