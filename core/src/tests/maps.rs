use crate::maps::{templates, Map, World};

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
    let template = templates::TEMPLE_MAP;
    test_map(template, Map::new(template));
}

#[test]
pub fn test_world() {
    let template = templates::TEMPLE_MAP;
    let world = World::new(template, 32);

    test_map(world.map.template, world.map.clone());

    assert!(world.tiles.len() > 0);
}
