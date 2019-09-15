use cgmath::Vector2;
use std::collections::HashMap;

use crate::maps::{Tile, World, templates};

use super::Actions;
use super::State;
use super::default;

#[derive(Clone)]
pub struct Game {
    pub elapsed_time: f64,
    pub world: World,
    pub viewport: HashMap<&'static str, Option<Tile>>,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            elapsed_time: 0.,
            world: World::new(templates::TEMPLE_MAP, 32),
            viewport: HashMap::new(),
        }
    }
}

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

fn set_viewport(state: &State, _position: &Vector2<i32>) -> State {
    State {
        game: Game {
            world: state.game.world.clone(),
            viewport: state.game.viewport.clone(),
            ..state.game
        },
        ..*state
    }
}

pub fn game_reducer(state: &State, action: &Actions) -> State {
    match action {
        Actions::GameSetElapsedTime(dt) => set_elapsed_time(state, dt),
        Actions::GameSetWorld(w) => set_world(state, w),
        Actions::GameSetViewPort(p) => set_viewport(state, p),
        _ => default(state),
    }
}

