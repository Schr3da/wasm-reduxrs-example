pub mod templates;
 
use std::vec::{Vec};
use cgmath::{Vector2};

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    position: Vector2<i32>,
    symbol: char,
}

impl Tile {
    fn new(x: i32, y: i32, symbol: char) -> Option<Tile> {
        match symbol {
            ' ' => None,
            _ => Some(Tile{
                position: Vector2::new(x, y),
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
                .map(|(x, s)| Tile::new(x as i32, y as i32, s))
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

    pub fn new(template: &'static str) -> Self {
        World {
            map: Map::new(template),
            tiles: Vec::new(),
        }
    }

    pub fn tiles_for_rect(&self, v1: Vector2<i32>, v2: Vector2<i32>) {
        let sum = v1 + v2;
        println!("{:?}", sum);
    }

}
