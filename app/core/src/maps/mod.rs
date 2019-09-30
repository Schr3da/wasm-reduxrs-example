pub mod templates;
mod utils;

use cgmath::Vector2;
use std::vec::Vec;

use crate::models::geometry::Size;
use crate::reducers::DEFAULT_TILE_SIZE;
use utils::{scale_tiles_x_direction, scale_tiles_y_direction};

pub type OptionTile = Option<Tile>;
pub type OptionTileVec = Vec<OptionTile>;

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub position: Vector2<i32>,
    pub size: Size<i32>,
    pub symbol: char,
}

impl Tile {
    fn new(x: i32, y: i32, symbol: char) -> Option<Tile> {
        match symbol {
            ' ' => None,
            _ => Some(Tile {
                position: Vector2::new(x, y),
                size: DEFAULT_TILE_SIZE,
                symbol,
            }),
        }
    }
}

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
                    .map(|(x, s)| Tile::new(x as i32, y as i32, s))
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
