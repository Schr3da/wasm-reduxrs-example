use crate::maps::{Map, temple};

pub fn test_map(template: &'static str, m: Map) {
    assert_eq!(template, m.template);
    assert!(m.tiles.len() > 0);
   
    for tiles in m.tiles {
        assert!(tiles.len() > 0);
        for tile in tiles {
            match tile {
                Some(_tile) => assert!(true),
                None => assert!(false),
            }
        }
    }
}

#[test]
pub fn test_maps() {
    test_map(temple::TEMPLATE, temple::new());
}

#[test]
pub fn test_world() {
    assert!(true);
}
