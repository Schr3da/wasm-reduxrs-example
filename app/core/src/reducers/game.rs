use cgmath::Vector2;
use std::collections::HashMap;

use crate::maps::{Tile, World, templates};

use super::Actions;
use super::state::{State, default};

pub static STATIC_VIEWPORT_ITEMS: &'static str = "static_views_items";
pub static DYNAMIC_VIEWPORT_ITEMS: &'static str = "dynamic_views_items";

#[derive(Clone)]
pub struct Game {
    pub elapsed_time: f64,
    pub world: World,
    pub view_position: Vector2<i32>,
    pub views: HashMap<&'static str, Option<Tile>>,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            elapsed_time: 0.,
            world: World::new(templates::TEMPLE_MAP, 32),
            view_position: Vector2::new(0, 0),
            views: HashMap::new(),
        }
    }
}

fn set_world(state: &State, world: &World) -> State {
    State {
        game: Game {
            world: world.clone(),
            views: state.game.views.clone(),
            ..state.game
        },
        ..*state
    }
}

fn set_elapsed_time(state: &State, tick: &f64) -> State {
    State {
        game: Game {
            world: state.game.world.clone(),
            views: state.game.views.clone(),
            elapsed_time: state.game.elapsed_time + (*tick),
            ..state.game
        },
        ..*state
    }
}

fn set_view_for_position(state: &State, view_position: &Vector2<i32>) -> State {
    let resolution = state.settings.resolution; 
    let tile_size = state.settings.default_tile_size;
    let world_tiles = &state.game.world.tiles;
  
    let max_x = resolution.x / tile_size.x;
    let max_y = resolution.y / tile_size.y;
    for y in view_position.y .. max_y {
        for x in view_position.x .. max_x {
            match world_tiles[y as usize][x as usize] {
                Option::Some(_t) => println!("match"),
                Option::None => println!("no match"),
            };
        }
    }

    State {
        game: Game {
            world: state.game.world.clone(),
            views: state.game.views.clone(),
            view_position: *view_position,
            ..state.game
        },
        ..*state
    }
}

pub fn game_reducer(state: &State, action: &Actions) -> State {
    match action {
        Actions::GameSetElapsedTime(dt) => set_elapsed_time(state, dt),
        Actions::GameSetWorld(w) => set_world(state, w),
        Actions::GameSetViewForPosition(p) => set_view_for_position(state, p),
        _ => default(state),
    }
}

