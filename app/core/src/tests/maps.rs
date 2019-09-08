use crate::maps::{temple};

#[test]
pub fn generate_map() {
    let map = temple::new();
    
    assert_eq!(temple::TEMPLATE, map.template);
    assert!(map.tiles.len() > 0);
    
    for tiles in map.tiles {
        assert!(tiles.len() > 0);
        for tile in tiles {
            match tile {
                Some(tile) => assert!(true),
                None => assert!(false),
            }
        }
    }
}

#[test]
pub fn generate_world() {
    assert!(true);
}
