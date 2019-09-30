use super::{OptionTile, OptionTileVec, Tile};
use cgmath::Vector2;

fn scale_tile_in_x_direction(scale: i32, t: &Tile) -> OptionTileVec {
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

fn scale_tile_in_y_direction(scale: i32, x_tiles: &OptionTileVec) -> OptionTileVec {
    let mut tiles: OptionTileVec = Vec::new();

    (0..scale).for_each(|y| {
        x_tiles.iter().for_each(|t| {
            match t {
                Some(v) => {
                    let x = v.position.x;
                    let y = v.position.y * y;
                    tiles.push(Tile::new(x, y, v.symbol)); 
                },
                _ => println!("Invalid tile"),
            };
        }); 
    });

    tiles
}

pub fn scale_tiles(scale: i32) -> Box<dyn FnMut(OptionTileVec, &OptionTile) -> OptionTileVec> {
    Box::new(
        move |mut result: OptionTileVec, t: &OptionTile| -> OptionTileVec {
            match t {
                Some(v) => {
                    let mut tiles = scale_tile_in_x_direction(scale, &v);
                    //tiles = scale_tile_in_y_direction(scale, &tiles);
                    result.append(&mut tiles);
                }
                None => println!("invalid tile"),
            };
            result
        },
    )
}
