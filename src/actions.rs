use crate::{Coord, Health, UnitType};

#[derive(Debug, Default, Clone, Copy)]
pub enum Action {
    #[default]
    Skip,
    Move{from:Coord,to:Coord},
    Repair{from:Coord,to:Coord},
    Attack{from:Coord,to:Coord},
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Skip => String::from("skips"),
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
    Skipped,
    Moved{delta:Coord},
    Repaired{amount:Health},
    Damaged{to_source:Health,to_target:Health},
}

impl std::fmt::Display for ActionOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Skipped => String::from("skipped"),
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

impl DropOutcome {
    pub fn has_dropped(&self) -> bool {
        if let Self::Drop { location: _ , unit_type: _ } = self {
            true
        } else {
            false
        }
    } 
}

impl std::fmt::Display for DropOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::NoDrop => String::from("no drop"),
            Self::Drop { location, unit_type } => format!("dropped {} at {}",unit_type,location),
        })
    }
}
