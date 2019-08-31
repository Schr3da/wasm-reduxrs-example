use std::vec::{Vec};

use crate::theme::{Theme, DarkTheme};

#[derive(Clone, Copy, Debug)]
struct Tile {
    x: usize,
    y: usize,
    symbol: char,
    color: &'static str, 
}

impl Tile {
    fn new(x: usize, y: usize, symbol: char) -> Option<Tile> {
        match symbol {
            ' ' => None,
            _ => Some(Tile{
                x,
                y,
                symbol,
                color: DarkTheme::color_background(),
            }),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Map {
    template: &'static str,
    tiles: Vec<Option<Tile>>,
}

impl Map {
    pub fn new(template: &'static str) -> Map {
        let mut map = Map{
            template,
            tiles: Vec::new(),
        };

        let data: Vec<&str> = template.split('\n')
        .collect();

        data.iter()
        .enumerate()
        .for_each(|(y, d)| {
            let mut tiles = Vec::new();

            d.chars()
            .enumerate()
            .for_each(|(x, s)| tiles.push(Tile::new(x, y, s)));

            map.tiles.append(&mut tiles);
        });

        return map;
    }
}

