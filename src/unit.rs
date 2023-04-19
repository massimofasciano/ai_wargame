use crate::{UnitType, Health};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Unit {
    pub (crate) unit_type : UnitType,
    pub (crate) health : Health,
}

impl Default for Unit {
    fn default() -> Self {
        Self::new(UnitType::default())
    }
}

impl Unit {
    pub fn new(unit_type : UnitType) -> Self {
        Self { unit_type, health: unit_type.initial_health() }
    }
    pub fn apply_repair(&mut self, target: &mut Self) -> u8 {
        let repair = self.unit_type.repair_amount(&target.unit_type);
        if repair + target.health < 9 {
            target.health += repair;
        } else {
            target.health = 9;
        }
        repair
    }
    pub fn apply_damage(&mut self, target: &mut Self) -> u8 {
        let damage = self.unit_type.damage_amount(&target.unit_type);
        if damage < target.health {
            target.health -= damage;
        } else {
            target.health = 0;
        }
        damage
    }
}

