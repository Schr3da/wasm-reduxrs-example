use cgmath::Vector2;
use std::vec::Vec;

use super::geometry::Size;

pub type OptionTile = Option<Tile>;
pub type OptionTileVec = Vec<OptionTile>;

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub position: Vector2<i32>,
    pub size: Size<i32>,
    pub symbol: char,
}

impl Tile {
    pub fn new(position: Vector2<i32>, size: Size<i32>, symbol: char) -> Option<Tile> {
        match symbol {
            ' ' => None,
            _ => Some(Tile {
                position,
                size,
                symbol,
            }),
        }
    }
}
