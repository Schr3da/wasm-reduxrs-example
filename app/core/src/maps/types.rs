use std::vec::{Vec};

#[derive(Clone, Copy, Debug)]
struct Tile {
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
    template: &'static str,
    tiles: Vec<Vec<Option<Tile>>>,
}

impl Map {
    pub fn new(template: &'static str) -> Map {
        let data: Vec<&str> = template.split('\n')
        .collect();

        let tiles = data.iter()
            .enumerate()
            .map(|(y, d)| 
                d.chars()
                .enumerate()
                .map(|(x, s)| Tile::new(x, y, s))
                .collect::<Vec<Option<Tile>>>()
            ).collect::<Vec<_>>();
        
        Map{
            template,
            tiles,
        }
    }
}

