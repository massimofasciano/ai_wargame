use crate::{DisplayFirstLetter, Health, HeuristicScore};

#[derive(Debug, PartialEq, PartialOrd,Default, Clone, Copy)]
#[derive(derive_more::Display)]
pub enum UnitType {
    AI,
    Virus,
    Tech,
    Firewall,
    #[default]
    Program,
}

impl DisplayFirstLetter for UnitType {}

impl UnitType {
    pub fn score(&self) -> HeuristicScore {
        use UnitType::*;
        match self {
            AI => 50,
            Virus => 20,
            Tech => 20,
            Firewall => 10,
            Program => 10,
        }
    }
    pub fn initial_health(&self) -> Health {
        9
    }
    pub fn damage_amount(&self, target: &Self) -> Health {
        use UnitType::*;
        match self {
            AI => match target {
                AI => 1,
                Virus => 4,
                Tech => 2,
                Firewall => 1,
                Program => 2,
            },
            Virus => match target {
                AI => 8,
                Virus => 1,
                Tech => 3,
                Firewall => 1,
                Program => 4,
            },
            Tech => match target {
                AI => 3,
                Virus => 7,
                Tech => 1,
                Firewall => 1,
                Program => 1,
            },
            Firewall => match target {
                AI => 1,
                Virus => 1,
                Tech => 1,
                Firewall => 1,
                Program => 1,
            },
            Program => match target {
                AI => 2,
                Virus => 2,
                Tech => 2,
                Firewall => 1,
                Program => 3,
            },
        }
    }
    pub fn repair_amount(&self, target: &Self) -> Health {
        use UnitType::*;
        match self {
            Tech => match target {
                AI => 5,
                Virus => 1,
                Tech => 1,
                Firewall => 3,
                Program => 2,
            },
            AI  => match target {
                Virus => 1,
                Firewall => 1,
                Program => 1,
                _ => 0,
            },
            _ => 0,
        }
    }
}