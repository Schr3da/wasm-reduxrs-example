use cgmath::Vector2;
use std::collections::HashMap;

use crate::maps::{templates, Tile, World};
use crate::utils::collection;

use super::state::{default, next, State};
use super::{Actions, DEFAULT_WORLD_SCALE};

pub static STATIC_WORLD_VIEW_ITEMS: &'static str = "static_world_items";

#[derive(Clone, Debug)]
pub struct Game {
    pub elapsed_time: f64,
    pub world: World,
    pub cursor: Vector2<i32>,
    pub view_position: Vector2<i32>,
    pub views: HashMap<&'static str, Vec<Option<Tile>>>,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            elapsed_time: 0.,
            world: World::new(templates::TEMPLE_MAP, DEFAULT_WORLD_SCALE),
            cursor: Vector2 { x: 0, y: 0 },
            view_position: Vector2::new(0, 0),
            views: HashMap::new(),
        }
    }
}

fn set_world(state: &State, world: &World) -> State {
    let mut next_state = next(state);
    next_state.next.game.world = world.clone();
    next_state = set_view_for_position(&next_state, &Vector2 { x: 0, y: 0 });
    next_state
}

fn set_cursor(state: &State, cursor: &Vector2<i32>) -> State {
    let mut next_state = next(state);
    next_state.next.game.cursor = *cursor;
    next_state
}

fn handle_key_up(state: &State, key: &char) -> State {
    println!("key released {:?}", key);
    let next_state = set_view_for_position(state, &Vector2 { x: 0, y: 0 });
    next_state
}

fn handle_key_down(state: &State, key: &char) -> State {
    println!("key pressed {:?}", key);
    let next_state = set_view_for_position(state, &Vector2 { x: 0, y: 0 });
    next_state
}

fn set_elapsed_time(state: &State, tick: &f64) -> State {
    let mut next_state = next(state);
    next_state.next.game.elapsed_time = next_state.next.game.elapsed_time + (*tick);
    next_state
}

fn set_view_for_position(state: &State, view_position: &Vector2<i32>) -> State {
    let resolution = state.next.settings.resolution;
    let tile_size = state.next.settings.default_tile_size;
    let world_tiles = &state.next.game.world.tiles;

    let max_x = resolution.w / tile_size.w;
    let max_y = resolution.h / tile_size.h;

    let mut views: HashMap<&'static str, Vec<Option<Tile>>> = HashMap::new();

    let mut world_view = Vec::new();
    for y in view_position.y..(view_position.y + max_y) {
        for x in view_position.x..(view_position.x + max_x) {
            let index_x = x as usize;
            let index_y = y as usize;

            if collection::is_out_of_bounds(y as usize, &world_tiles) == false {
                break;
            }

            if collection::is_out_of_bounds(x as usize, &world_tiles[index_y]) == false {
                break;
            }

            println!("{:?}", world_tiles[index_y][index_x]);
            world_view.push(world_tiles[index_y][index_x]);
        }
    }

    views.insert(STATIC_WORLD_VIEW_ITEMS, world_view);

    let mut next_state = next(state);
    next_state.next.game.views = views;
    next_state.next.game.view_position = *view_position;
    next_state
}

pub fn game_reducer(state: &State, action: &Actions) -> State {
    match action {
        Actions::GameSetElapsedTime(dt) => set_elapsed_time(state, dt),
        Actions::GameSetWorld(w) => set_world(state, w),
        Actions::GameSetGameCursor(c) => set_cursor(state, c),
        Actions::GameHandleKeyUp(k) => handle_key_up(state, k),
        Actions::GameHandleKeyDown(k) => handle_key_down(state, k),
        Actions::GameSetViewForPosition(p) => set_view_for_position(state, p),
        _ => default(state),
    }
}
