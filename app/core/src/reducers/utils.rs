use cgmath::Vector2;

use crate::models::tile::{OptionTile, OptionTileVec};

use super::game::STATIC_WORLD_VIEW_ITEMS;
use super::state::State;

fn is_out_of_bounds<T>(index: usize, array: &Vec<T>) -> bool {
    let length = array.len();
    index >= length
}

fn max_tiles_to_render(state: &State) -> (i32, i32) {
    let resolution = state.next.settings.resolution;
    let tile_size = state.next.settings.default_tile_size;
    let max_x = resolution.w / tile_size.w;
    let max_y = resolution.h / tile_size.h;
    (max_x, max_y)
}

pub fn consider_cursor_resolution_limits(state: &State, position: Vector2<i32>) -> Vector2<i32> {
    let (max_x, max_y) = max_tiles_to_render(state);
    if position.x < 0 || position.x >= max_x || position.y < 0 || position.y >= max_y {
        return state.next.game.cursor.position;
    }
    return position;
}

pub fn get_selected_cursor_tile(state: &State) -> OptionTile {
    let (_, max_y) = max_tiles_to_render(state);
    let view = &state.next.game.views[STATIC_WORLD_VIEW_ITEMS];
    let position = state.next.game.cursor.position;

    let index = (position.x + position.y * max_y) as usize;

    if is_out_of_bounds(index, view) {
        return Option::None;
    }

    return view[index].clone();
}

pub fn consider_scroll_limits(state: &State, next: Vector2<i32>) -> Vector2<i32> {
    let world_tiles = &state.next.game.world.tiles;
    let prev = state.next.game.view_position.clone();
    let (max_x, max_y) = max_tiles_to_render(state);

    let x = (next.x + max_x) as usize;
    let y = (next.y + max_y) as usize;

    if is_out_of_bounds(y, &world_tiles)
        || is_out_of_bounds(x, &world_tiles[y])
        || next.x < 0
        || next.y < 0
    {
        return prev;
    }

    return next;
}

pub fn calculate_translation_for_view_position(
    state: &State,
    view_position: &Vector2<i32>,
) -> Vector2<i32> {
    let tile_size = state.next.settings.default_tile_size;
    let translation = Vector2 {
        x: view_position.x * tile_size.w,
        y: view_position.y * tile_size.h,
    };
    translation
}

pub fn tiles_for_world_view(state: &State, view_position: &Vector2<i32>) -> OptionTileVec {
    let mut world_view = Vec::new();

    let world_tiles = &state.next.game.world.tiles;
    let (max_x, max_y) = max_tiles_to_render(state);

    for y in view_position.y..(view_position.y + max_y) {
        for x in view_position.x..(view_position.x + max_x) {
            let index_x = x as usize;
            let index_y = y as usize;

            if is_out_of_bounds(y as usize, &world_tiles) {
                break;
            }

            if is_out_of_bounds(x as usize, &world_tiles[index_y]) {
                break;
            }

            world_view.push(world_tiles[index_y][index_x]);
        }
    }

    world_view
}
