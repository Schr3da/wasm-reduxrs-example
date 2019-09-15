pub mod templates;

use cgmath::Vector2;
use std::vec::Vec;

#[derive(Clone, Copy, Debug)]
pub struct Size<T> {
    pub w: T,
    pub h: T,
}

impl<T> Size<T> {
    pub fn new(w: T, h: T) -> Self {
        Size { w, h }
    }
}

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
                size: Size::new(16, 16),
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
        let mapped_tiles = map.tiles.iter().map(|tiles| tiles.iter()
            .fold(Vec::new(), |mut result: Vec<OptionTile>, t| -> Vec<OptionTile> {
            match t {
                Some(v) => {
                    let position = v.position;
                    let size = v.size;

                    (0..scale).for_each(|y| {
                        (0..scale).for_each(|x| {
                            let new_pos = Vector2{ 
                                x: (position.x * scale + x) * size.w,
                                y: (position.y * size.h + y) * size.h,
                            };
                            result.push(Tile::new(new_pos.x, new_pos.y, v.symbol));
                       }) 
                    });
                },
                None => println!("invalid tile"),
            };
            result
        })).collect::<Vec<Vec<OptionTile>>>();
        
        World {
            map,
            tiles: mapped_tiles,
        }
    }
}
