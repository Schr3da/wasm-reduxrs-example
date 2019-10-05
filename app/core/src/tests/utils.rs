use crate::maps::templates::TEMPLE_MAP;
use crate::maps::World;

#[test]
fn test_world_scaling() {
    let scale = 1;
    let w = World::new(TEMPLE_MAP, scale);
    let count = w.map.tiles.len() as i32;
    assert_eq!(scale * scale * count, count);
}
