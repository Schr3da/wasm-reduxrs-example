pub mod templates;

use cgmath::Vector2;
use std::vec::Vec;

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    position: Vector2<i32>,
    symbol: char,
}

impl Tile {
    fn new(x: i32, y: i32, symbol: char) -> Option<Tile> {
        match symbol {
            ' ' => None,
            _ => Some(Tile {
                position: Vector2::new(x, y),
                symbol,
            }),
        }
    }
}

type OptionTile = Option<Tile>;

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
                    .collect::<Vec<OptionTile>>()
            })
            .collect::<Vec<Vec<OptionTile>>>();

        Map { template, tiles }
    }
}

#[derive(Clone, Debug)]
pub struct World {
    pub map: Map,
    pub tiles: Vec<Vec<OptionTile>>,
}

impl World {
    pub fn new(template: &'static str, scale: i32) -> Self { 
        let map = Map::new(template);
        let tiles = map.tiles.iter().map(|t| t.iter()
            .fold(Vec::new(), |mut result: Vec<OptionTile>, t: &OptionTile| -> Vec<OptionTile> {
            match t {
                Some(v) => {
                    let pos = v.position;
                    (0..scale).for_each(|i| result.push(Tile::new(pos.x * i, pos.y * i, v.symbol)));
                },
                None => println!("invalid tile"),
            };
            result
        })).collect::<Vec<Vec<OptionTile>>>();
        
        World {
            map,
            tiles,
        }
    }

    pub fn tiles_for_rect(&self, v1: Vector2<i32>, v2: Vector2<i32>) {
        let sum = v1 + v2;
        println!("{:?}", sum);
    }
}
