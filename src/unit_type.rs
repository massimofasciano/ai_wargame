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
            Virus => 25,
            Tech => 25,
            Firewall => 10,
            Program => 10,
        }
    }
    pub fn can_move_back(&self) -> bool {
        use UnitType::*;
        match self {
            Virus | Tech => true,
            _ => false,
        }
    }
    pub fn can_move_while_engaged(&self) -> bool {
        use UnitType::*;
        match self {
            Virus | Tech => true,
            _ => false,
        }
    }
    pub fn initial_health(&self) -> Health {
        9
    }
    pub fn units_description() -> String {
        String::from("\
            repair 3 (Tech => AI,Firewall,Program)\n\
            repair 1 (AI => Virus,Tech)\n\
            damage 9 (Virus => AI)\n\
            damage 6 (Virus => Tech,Program) (Tech => Virus)\n\
            damage 3 (AI,Program => any unit except Firewall)\n\
            damage 1 (Firewall <=> any unit) (Tech => any unit)\n\
        ")
    }
    pub fn damage_amount(&self, target: &Self) -> Health {
        // update description when changing any value
        use UnitType::*;
        match self {
            AI => match target {
                Firewall => 1,
                _ => 3,
            },
            Virus => match target {
                AI => 9,
                Tech | Program => 6,
                Virus | Firewall => 1,
            },
            Tech => match target {
                Virus => 6,
                _ => 1,
            },
            Firewall => match target {
                _ => 1,
            },
            Program => match target {
                Firewall => 1,
                _ => 3,
            },
        }
    }
    pub fn repair_amount(&self, target: &Self) -> Health {
        // update description when changing any value
        use UnitType::*;
        match self {
            Tech => match target {
                AI | Firewall | Program => 3,
                _ => 0,
            },
            AI  => match target {
                Virus | Tech => 1,
                _ => 0,
            },
            _ => 0,
        }
    }
}