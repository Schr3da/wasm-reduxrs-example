use super::{Map};

pub static TEMPLATE: &'static str ="
FFFFFFFFFFWWWWFFFFFFFFFF
FFM....FFFWWWWFFF....MFF
FF........WWWW........FF
FF...FF..........FF...FF
FF..FFF..........FFF..FF
FF...FF..........FF...FF
FF........WWWW........FF
FFMP...FFFWWWWFFF...PMFF
FFFFFFFFFFWWWWFFFFFFFFFF
";

pub fn new() -> Map {
    Map::new(TEMPLATE)
}
