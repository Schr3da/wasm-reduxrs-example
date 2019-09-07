use std::collections::{HashMap};

use crate::buildings::{
    Building,
    BuildingType,
    BuildingPosition,
};

pub fn new_city_center(id: String, x: i32, y:i32) -> Building {
    let abilities = HashMap::new();
    let template = String::from("
    xxxx
    xCCx
    xCCx
    xxxx
    ");
    
    Building {
        id, 
        template,
        abilities,
        cost: 380,
        hp: 1500,
        armor: 5,
        food: 12,
        build_time: 180,
        building_type: BuildingType::CityCenter,
        position: BuildingPosition{ x, y },
        units: HashMap::new(),
        active_ability: Option::None,
    }
}
