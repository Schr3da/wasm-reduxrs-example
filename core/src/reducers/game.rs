use cgmath::Vector2;
use math::round::floor;
use std::collections::HashMap;

use crate::maps::World;
use crate::models::cursor::Cursor;
use crate::models::tile::Tile;

use super::{Actions, MouseActions};
use super::state::{default, next, State};
use super::utils::{
    calculate_translation_for_view_position, consider_cursor_resolution_limits,
    consider_scroll_limits, get_selected_cursor_tile, tiles_for_world_view,
};

pub static STATIC_WORLD_VIEW_ITEMS: &'static str = "static_world_items";

#[derive(Clone, Debug)]
pub struct Game {
    pub elapsed_time: i32,
    pub world: World,
    pub cursor: Cursor,
    pub view_position: Vector2<i32>,
    pub views: HashMap<&'static str, Vec<Option<Tile>>>,
    pub translation: Vector2<i32>,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            elapsed_time: 0,
            world: World::default(),
            cursor: Cursor::default(),
            views: HashMap::new(),
            view_position: Vector2::new(0, 0),
            translation: Vector2::new(0, 0),
        }
    }
}

fn start_new_game(state: &State) -> State {
    let mut next_state = next(state);
    let view_position = next_state.next.game.view_position;
    next_state.next.game = Game::default();
    next_state = set_view_for_position(&next_state, view_position);
    next_state
}

fn set_elapsed_time(state: &State, tick: &i32) -> State {
    let mut next_state = next(state);
    next_state.next.game.elapsed_time = next_state.next.game.elapsed_time + (*tick);
    next_state
}

fn set_cursor_position(state: &State, position: Vector2<i32>) -> State {
    let mut next_state = next(state);
    next_state.next.game.cursor.selected_tile = Option::None;
    next_state.next.game.cursor.position = consider_cursor_resolution_limits(&state, position);
    next_state
}

fn select_tile_at_cursor_position(state: &State, position: Vector2<i32>) -> State {
    let mut next_state = next(state);
    next_state.next.game.cursor.position = consider_cursor_resolution_limits(&state, position);
    
    let selected_tile = get_selected_cursor_tile(&next_state);
    next_state.next.game.cursor.selected_tile = selected_tile;
    next_state
}

fn toggle_cursor_tile(state: &State) -> State {
    let mut next_state = next(state);
    let selected_tile = match next_state.next.game.cursor.selected_tile {
        Some(_t) => Option::None,
        _ => get_selected_cursor_tile(&next_state),
    };
    next_state.next.game.cursor.selected_tile = selected_tile;
    next_state
}

fn set_world(state: &State, world: &World) -> State {
    let mut next_state = next(state);
    next_state.next.game.world = world.clone();
    next_state = set_view_for_position(&next_state, Vector2 { x: 0, y: 0 });
    next_state
}

fn set_view_for_position(state: &State, view_position: Vector2<i32>) -> State {
    let mut next_state = next(state);
    let next_view_position = consider_scroll_limits(&state, view_position);

    let mut views: HashMap<&'static str, Vec<Option<Tile>>> = HashMap::new();
    let world_view = tiles_for_world_view(state, &next_view_position);

    views.insert(STATIC_WORLD_VIEW_ITEMS, world_view);

    next_state.next.game.cursor.selected_tile = None;
    next_state.next.game.views = views;
    next_state.next.game.view_position = next_view_position;
    next_state.next.game.translation =
        calculate_translation_for_view_position(&next_state, &next_view_position);
    next_state
}

fn handle_mouse_up(state: &State, action: &MouseActions) -> State {
    let next_state = next(state); 
    let size = next_state.next.settings.default_tile_size;
   
    match action {
        MouseActions::Primary(x, y) =>
            select_tile_at_cursor_position(&next_state, Vector2 {
                x: floor((x / size.w) as f64, 1) as i32,
                y: floor((y / size.h) as f64, 1) as i32,
            }),
        _ => next_state
    }
}


fn handle_key_up(state: &State, key: &String) -> State {
    let view_position = state.next.game.view_position;
    let cursor_position = state.next.game.cursor.position;

    match key.as_ref() {
        "w" => set_cursor_position(state, cursor_position - Vector2 { x: 0, y: 1 }),
        "d" => set_cursor_position(state, cursor_position + Vector2 { x: 1, y: 0 }),
        "s" => set_cursor_position(state, cursor_position + Vector2 { x: 0, y: 1 }),
        "a" => set_cursor_position(state, cursor_position - Vector2 { x: 1, y: 0 }),
        " " => toggle_cursor_tile(state),
        "ArrowUp" => set_view_for_position(state, view_position - Vector2 { x: 0, y: 1 }),
        "ArrowRight" => set_view_for_position(state, view_position + Vector2 { x: 1, y: 0 }),
        "ArrowDown" => set_view_for_position(state, view_position + Vector2 { x: 0, y: 1 }),
        "ArrowLeft" => set_view_for_position(state, view_position - Vector2 { x: 1, y: 0 }),
        _ => default(state),
    }
}

fn handle_key_down(state: &State, _key: &str) -> State {
    default(&state)
}

pub fn game_reducer(state: &State, action: &Actions) -> State {
    match action {
        Actions::GameStartNew() => start_new_game(state),
        Actions::GameSetElapsedTime(dt) => set_elapsed_time(state, dt),
        Actions::GameSetWorld(w) => set_world(state, w),
        Actions::GameSetViewForPosition(v) => set_view_for_position(state, *v),
        Actions::GameSetGameCursor(c) => set_cursor_position(state, *c),
        Actions::GameHandleKeyUp(k) => handle_key_up(state, k),
        Actions::GameHandleKeyDown(k) => handle_key_down(state, k),
        Actions::GameHandleMouseUp(a) => handle_mouse_up(state, a),
        _ => default(state),
    }
}
