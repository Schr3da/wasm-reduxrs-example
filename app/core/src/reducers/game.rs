use crate::maps::World;
use crate::state::{Game, State};

pub fn set_world(state: &State, world: &World) -> State {
    State {
        game: Game {
            world: world.clone(),
            ..state.game
        },
        ..*state
    }
}

pub fn set_elapsed_time(state: &State, tick: &f64) -> State {
    State {
        game: Game {
            world: state.game.world.clone(),
            elapsed_time: state.game.elapsed_time + (*tick),
            ..state.game
        },
        ..*state
    }
}
