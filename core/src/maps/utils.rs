use cgmath::Vector2;

use crate::models::tile::{OptionTile, OptionTileVec, Tile};
use crate::reducers::DEFAULT_TILE_SIZE;

pub fn scale_tiles_x_direction(
    scale: i32,
) -> Box<dyn FnMut(OptionTileVec, &OptionTile) -> OptionTileVec> {
    Box::new(
        move |mut result: OptionTileVec, tile: &OptionTile| -> OptionTileVec {
            match tile {
                Some(t) => {
                    (0..scale).for_each(|index| {
                        let x = (t.position.x * scale + index) * t.size.w;
                        let y = t.position.y;
                        result.push(Tile::new(Vector2 { x, y }, DEFAULT_TILE_SIZE, t.symbol));
                    });
                }
                None => println!("invalid tile"),
            };
            result
        },
    )
}

pub fn scale_tiles_y_direction(
    scale: i32,
) -> Box<dyn FnMut(Vec<OptionTileVec>, &OptionTileVec) -> Vec<OptionTileVec>> {
    Box::new(
        move |mut result: Vec<OptionTileVec>, tiles: &OptionTileVec| -> Vec<OptionTileVec> {
            (0..scale).for_each(|index| {
                let to_add = tiles.iter().fold(Vec::new(), |mut scaled_tiles, t| {
                    match t {
                        Some(v) => {
                            let x = v.position.x;
                            let y = (v.position.y * scale + index) * v.size.h;
                            scaled_tiles.push(Tile::new(
                                Vector2 { x, y },
                                DEFAULT_TILE_SIZE,
                                v.symbol,
                            ))
                        }
                        _ => println!("Invalid tile"),
                    };
                    scaled_tiles
                });
                result.push(to_add);
            });
            result
        },
    )
}
