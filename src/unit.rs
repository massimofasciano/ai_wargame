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
        use UnitType::*;
        let health = match unit_type {
            AI => 5,
            Hacker => 3,
            Repair => 2,
            Tank => 9,
            Drone => 6,
            Soldier => 4,
        };
        Self { unit_type, health }
    }
    pub fn apply_repair(&mut self, target: &mut Self) -> u8 {
        let repair = self.repair(target);
        if repair + target.health < 9 {
            target.health += repair;
        } else {
            target.health = 9;
        }
        repair
    }
    pub fn repair(&self, target: &Self) -> Health {
        use UnitType::*;
        match self.unit_type {
            Repair => match target.unit_type {
                AI => 3,
                Hacker => 1,
                Repair => 2,
                Tank => 1,
                Drone => 1,
                Soldier => 1,
            },
            _ => 0,
        }
    }
    pub fn apply_damage(&mut self, target: &mut Self) -> u8 {
        let damage = self.damage(target);
        if damage < target.health {
            target.health -= damage;
        } else {
            target.health = 0;
        }
        damage
    }
    pub fn damage(&self, target: &Self) -> Health {
        use UnitType::*;
        match self.unit_type {
            AI => match target.unit_type {
                AI => 1,
                Hacker => 1,
                Repair => 1,
                Tank => 3,
                Drone => 3,
                Soldier => 3,
            },
            Hacker => match target.unit_type {
                AI => 4,
                Hacker => 1,
                Repair => 2,
                Tank => 1,
                Drone => 1,
                Soldier => 1,
            },
            Repair => match target.unit_type {
                AI => 0,
                Hacker => 1,
                Repair => 1,
                Tank => 0,
                Drone => 0,
                Soldier => 0,
            },
            Tank => match target.unit_type {
                AI => 1,
                Hacker => 1,
                Repair => 1,
                Tank => 2,
                Drone => 2,
                Soldier => 3,
            },
            Drone => match target.unit_type {
                AI => 1,
                Hacker => 1,
                Repair => 1,
                Tank => 6,
                Drone => 2,
                Soldier => 1,
            },
            Soldier => match target.unit_type {
                AI => 2,
                Hacker => 2,
                Repair => 1,
                Tank => 2,
                Drone => 5,
                Soldier => 2,
            },
        }
    }
}

