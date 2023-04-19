use crate::DisplayFirstLetter;

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
