pub mod temple;
 
use std::vec::{Vec};

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    x: usize,
    y: usize,
    symbol: char,
}

impl Tile {
    fn new(x: usize, y: usize, symbol: char) -> Option<Tile> {
        match symbol {
            ' ' => None,
            _ => Some(Tile{
                x,
                y,
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
        let data: Vec<&str> = template.split('\n')
        .filter(|v| *v != "")
        .collect();

        let tiles = data.iter()
            .enumerate()
            .map(|(y, d)| 
                d.chars()
                .enumerate()
                .map(|(x, s)| Tile::new(x, y, s))
                .collect::<Vec<Option<Tile>>>()
            ).collect::<Vec<Vec<Option<Tile>>>>();
       
        Map{
            template,
            tiles,
        }
    }
}

pub struct World {
    pub map: Map,
    pub tiles: Vec<Vec<Option<Tile>>>,
}

impl World {

    pub fn new(map: Map) -> Self {
        World {
            map,
            tiles: Vec::new(),
        }
    }
}
