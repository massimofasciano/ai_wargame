use crate::{Coord, Health, UnitType};

#[derive(Debug, Default, Clone, Copy)]
pub enum Action {
    #[default]
    Pass,
    Move{from:Coord,to:Coord},
    Repair{from:Coord,to:Coord},
    Attack{from:Coord,to:Coord},
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Pass => String::from("passes"),
            Self::Move { from, to } => 
                format!("moves from {} to {}",from,to),
            Self::Repair { from, to } => 
                format!("repairs from {} to {}",from,to),
            Self::Attack { from, to } => 
                format!("attacks from {} to {}",from,to),
        })
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum ActionOutcome {
    #[default]
    Passed,
    Moved{delta:Coord},
    Repaired{amount:Health},
    Damaged{to_source:Health,to_target:Health},
}

impl std::fmt::Display for ActionOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Passed => String::from("passed"),
            Self::Moved { delta } => format!("moved by {}",delta.to_string_as_tuple()),
            Self::Repaired { amount } => format!("repaired {} health points",amount),
            Self::Damaged { to_source, to_target } => 
                format!("combat damage: to source = {}, to target = {}",to_source,to_target),
        })
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum DropOutcome {
    #[default]
    NoDrop,
    Drop{location:Coord,unit_type:UnitType},
}

impl std::fmt::Display for DropOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::NoDrop => String::from("no drop"),
            Self::Drop { location, unit_type } => format!("dropped {} at {}",unit_type,location),
        })
    }
}

pub trait IsUsefulInfo {
    fn is_useful_info(&self) -> bool;
}

impl IsUsefulInfo for ActionOutcome {
    fn is_useful_info(&self) -> bool {
        match self {
            Self::Damaged { to_source: _, to_target: _ } => true,
            Self::Repaired { amount: _ } => true,
            _ => false,
        }
    }
}

impl IsUsefulInfo for DropOutcome {
    fn is_useful_info(&self) -> bool {
        match self {
            Self::Drop { location: _, unit_type: _ } => true,
            _ => false,
        }
    }
}
