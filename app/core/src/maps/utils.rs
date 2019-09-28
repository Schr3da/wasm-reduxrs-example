use super::{OptionTile, OptionTileVec, Tile};
use cgmath::Vector2;

fn scale_tile_in_x_direction(scale: i32, t: &Tile) -> Vec<OptionTile> {
    let mut tiles: OptionTileVec = Vec::new();
    let position = t.position;
    let size = t.size;

    (0..scale).for_each(|x| {
        let x = (position.x * scale + x) * size.w;
        let y = position.y * size.h;
        let new_pos = Vector2 { x, y };
        tiles.push(Tile::new(new_pos.x, new_pos.y, t.symbol));
    });

    tiles
}

fn scale_tile_in_y_direction(scale: i32, _t: &Tile) -> OptionTileVec {
    let mut tiles: OptionTileVec = Vec::new();

    (0..scale).for_each(|x| {});

    tiles
}

pub fn scale_tiles(scale: i32) -> Box<dyn FnMut(OptionTileVec, &OptionTile) -> OptionTileVec> {
    Box::new(
        move |mut result: OptionTileVec, t: &OptionTile| -> OptionTileVec {
            match t {
                Some(v) => {
                    let mut x_tiles = scale_tile_in_x_direction(scale, &v);
                    let mut y_tiles = scale_tile_in_y_direction(scale, &v);
                    result.append(&mut x_tiles);
                    result.append(&mut y_tiles);
                }
                None => println!("invalid tile"),
            };
            result
        },
    )
}
