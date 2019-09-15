use crate::actions::Actions;
use crate::maps::World;
use crate::state::{Game, State};

use super::default;

fn set_world(state: &State, world: &World) -> State {
    State {
        game: Game {
            world: world.clone(),
            viewport: state.game.viewport.clone(),
            ..state.game
        },
        ..*state
    }
}

fn set_elapsed_time(state: &State, tick: &f64) -> State {
    State {
        game: Game {
            world: state.game.world.clone(),
            viewport: state.game.viewport.clone(),
            elapsed_time: state.game.elapsed_time + (*tick),
            ..state.game
        },
        ..*state
    }
}

pub fn game_reducer(state: &State, action: &Actions) -> State {
    match action {
        Actions::GameSetElapsedTime(dt) => set_elapsed_time(state, dt),
        Actions::GameSetWorld(w) => set_world(state, w),
        _ => default(state),
    }
}

