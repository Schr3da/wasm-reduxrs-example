use cgmath::Vector2;

use super::tile::OptionTile;

#[derive(Clone, Copy, Debug)]
pub struct Cursor {
    pub selected_tile: OptionTile,
    pub position: Vector2<i32>,
}

impl Default for Cursor {
    fn default() -> Self {
        Cursor {
            selected_tile: Option::None,
            position: Vector2{ x: 0, y: 0 },
        }
    }
}

