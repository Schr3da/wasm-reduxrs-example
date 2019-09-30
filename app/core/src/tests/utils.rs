use crate::maps::World;
use crate::maps::templates::TEMPLE_MAP;

#[test]
fn test_world_scaling() {
    let scale = 2;
    let w = World::new(TEMPLE_MAP, scale);
    let count = w.map.tiles.len() as i32;
    assert_eq!(scale * scale * count, count);
}

