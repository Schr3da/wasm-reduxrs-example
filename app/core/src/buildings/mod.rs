use std::collections::{HashMap};
use cgmath::{Vector2};

pub enum BuildingAbilityIds {

}

pub struct BuildingAbility {
    pub id: BuildingAbilityIds,
    pub name: String,
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
    pub position: Vector2<i32>,
    pub abilities: HashMap<String, BuildingAbility>,
    pub active_ability: Option<BuildingAbility>
}

pub mod humans;
