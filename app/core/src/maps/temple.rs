use super::types::{Map};

static TEMPLATE: &'static str ="
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

pub fn new_temple_map() -> Map {
    Map::new(TEMPLATE)
}
