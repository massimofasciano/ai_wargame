use crate::{DisplayFirstLetter, Health};

#[derive(Debug, PartialEq, PartialOrd,Default, Clone, Copy)]
#[derive(derive_more::Display)]
pub enum UnitType {
    AI,
    Hacker,
    Repair,
    Tank,
    Drone,
    #[default]
    Soldier,
}

impl DisplayFirstLetter for UnitType {}

impl UnitType {
    pub fn initial_health(&self) -> Health {
        use UnitType::*;
        match self {
            AI => 5,
            Hacker => 3,
            Repair => 2,
            Tank => 9,
            Drone => 6,
            Soldier => 4,
        }
    }
    pub fn damage_amount(&self, target: &Self) -> Health {
        use UnitType::*;
        match self {
            AI => match target {
                AI => 1,
                Hacker => 1,
                Repair => 1,
                Tank => 3,
                Drone => 3,
                Soldier => 3,
            },
            Hacker => match target {
                AI => 4,
                Hacker => 1,
                Repair => 2,
                Tank => 1,
                Drone => 1,
                Soldier => 1,
            },
            Repair => match target {
                AI => 0,
                Hacker => 1,
                Repair => 1,
                Tank => 0,
                Drone => 0,
                Soldier => 0,
            },
            Tank => match target {
                AI => 1,
                Hacker => 1,
                Repair => 1,
                Tank => 2,
                Drone => 2,
                Soldier => 3,
            },
            Drone => match target {
                AI => 1,
                Hacker => 1,
                Repair => 1,
                Tank => 6,
                Drone => 2,
                Soldier => 1,
            },
            Soldier => match target {
                AI => 2,
                Hacker => 2,
                Repair => 1,
                Tank => 2,
                Drone => 5,
                Soldier => 2,
            },
        }
    }
    pub fn repair_amount(&self, target: &Self) -> Health {
        use UnitType::*;
        match self {
            Repair => match target {
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
}