pub mod templates;
mod utils;

use cgmath::Vector2;

use std::vec::Vec;

use crate::maps::templates::TEMPLE_MAP;
use crate::models::tile::{OptionTileVec, Tile};
use crate::reducers::{DEFAULT_TILE_SIZE, DEFAULT_WORLD_SCALE};
use utils::{scale_tiles_x_direction, scale_tiles_y_direction};

#[derive(Clone, Debug)]
pub struct Map {
    pub template: &'static str,
    pub tiles: Vec<Vec<Option<Tile>>>,
}

impl Map {
    pub fn new(template: &'static str) -> Map {
        let data: Vec<&str> = template.split('\n').filter(|v| *v != "").collect();

        let tiles = data
            .iter()
            .enumerate()
            .map(|(y, d)| {
                d.chars()
                    .enumerate()
                    .map(|(x, s)| {
                        Tile::new(
                            Vector2 {
                                x: x as i32,
                                y: y as i32,
                            },
                            DEFAULT_TILE_SIZE,
                            s,
                        )
                    })
                    .collect::<OptionTileVec>()
            })
            .collect::<Vec<OptionTileVec>>();

        Map { template, tiles }
    }
}

#[derive(Clone, Debug)]
pub struct World {
    pub map: Map,
    pub tiles: Vec<OptionTileVec>,
}

impl Default for World {
    fn default() -> Self {
        World::new(TEMPLE_MAP, DEFAULT_WORLD_SCALE)
    }
}

impl World {
    pub fn new(template: &'static str, scale: i32) -> Self {
        let map = Map::new(template);

        let scaled_x_tiles = map
            .tiles
            .iter()
            .map(|tiles| {
                tiles
                    .iter()
                    .fold(Vec::new(), scale_tiles_x_direction(scale))
            })
            .collect::<Vec<OptionTileVec>>();

        let scaled_y_tiles = scaled_x_tiles
            .iter()
            .fold(Vec::new(), scale_tiles_y_direction(scale));

        World {
            map,
            tiles: scaled_y_tiles,
        }
    }
}
