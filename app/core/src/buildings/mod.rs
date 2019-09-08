use std::collections::{HashMap};

pub enum BuildingAbilityIds {

}

pub struct BuildingAbility {
    pub id: BuildingAbilityIds,
    pub name: String,
}

pub struct BuildingPosition {
    pub x: i32,
    pub y: i32,
}

pub enum BuildingType {
    CityCenter,
}

pub struct Building {
    pub id: String, 
    pub cost: i32,
    pub hp: i32,
    pub armor: i32,
    pub food: i32,
    pub template: String,
    pub build_time: i32,
    pub building_type: BuildingType,
    pub position: BuildingPosition,
    pub abilities: HashMap<String, BuildingAbility>,
    pub active_ability: Option<BuildingAbility>
}

pub mod humans;
