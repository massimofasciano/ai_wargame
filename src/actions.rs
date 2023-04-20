use crate::{Coord, Health};

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
