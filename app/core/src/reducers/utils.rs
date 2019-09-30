use super::state::State;
use crate::maps::OptionTileVec;
use cgmath::Vector2;

fn is_out_of_bounds<T>(index: usize, array: &Vec<T>) -> bool {
    let length = array.len();
    index < length
}

pub fn calculate_translation_for_view_position(
    state: &State,
    view_position: &Vector2<i32>,
) -> Vector2<i32> {
    let tile_size = state.next.settings.default_tile_size;
    Vector2 {
        x: view_position.x * tile_size.w,
        y: view_position.y * tile_size.h,
    }
}

pub fn tiles_for_world_view(state: &State, view_position: &Vector2<i32>) -> OptionTileVec {
    let resolution = state.next.settings.resolution;
    let tile_size = state.next.settings.default_tile_size;
    let world_tiles = &state.next.game.world.tiles;

    let max_x = resolution.w / tile_size.w;
    let max_y = resolution.h / tile_size.h;

    let mut world_view = Vec::new();

    for y in view_position.y..(view_position.y + max_y) {
        for x in view_position.x..(view_position.x + max_x) {
            let index_x = x as usize;
            let index_y = y as usize;

            if is_out_of_bounds(y as usize, &world_tiles) == false {
                break;
            }

            if is_out_of_bounds(x as usize, &world_tiles[index_y]) == false {
                break;
            }

            world_view.push(world_tiles[index_y][index_x]);
        }
    }
    world_view
}
