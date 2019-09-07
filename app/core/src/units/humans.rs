use std::collections::{HashMap};
use std::string::{String};

use crate::units::{
    Unit,
    UnitPosition,
    UnitType,
    UnitAttackType,
    UnitWeaponType,
    UnitArmorType,
};

pub fn new_worker(id: String, x: i32, y: i32) -> Unit {
    let abilities = HashMap::new();
    let template = String::from("W");

    Unit {
        id,
        template,
        abilities,
        level: 1,
        cost: 75,
        food: 1,
        armor: 0,
        attack: 5,
        hp: 220,
        cooldown: 2,
        can_attack_air: false,
        can_attack_ground: true,
        position: UnitPosition{ x, y }, 
        unit_type: UnitType::Worker,
        attack_type: UnitAttackType::Normal,
        weapon_type: UnitWeaponType::Weak,
        armor_type: UnitArmorType::Normal,
        active_ability: Option::None,
    }
}
