use crate::units::{Unit};

pub enum Actions {
    Add(Unit),
    Remove(Unit),
    Move(Unit),
    Attack(Unit),
}
