use crate::{Coord, Health};

#[derive(Debug, Default, Clone, Copy)]
pub enum Action {
    #[default]
    Pass,
    Move{from:Coord,to:Coord},
    Repair{from:Coord,to:Coord},
    Attack{from:Coord,to:Coord},
    SelfDestruct{from:Coord},
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Pass => String::from("passes"),
            Self::Move { from, to } => 
                format!("move from {} to {}",from,to),
            Self::Repair { from, to } => 
                format!("repair from {} to {}",from,to),
            Self::Attack { from, to } => 
                format!("attack from {} to {}",from,to),
            Self::SelfDestruct { from } => 
                format!("self-destruct at {}",from),
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
    SelfDestructed{total_damage:Health},
}

impl std::fmt::Display for ActionOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Passed => String::from("passed"),
            Self::Moved { delta } => format!("moved by {}",delta.to_string_as_tuple()),
            Self::Repaired { amount } => format!("repaired {amount} health points"),
            Self::Damaged { to_source: 0, to_target } => 
                format!("combat damage: {to_target}"),
            Self::Damaged { to_source, to_target } => 
                format!("combat damage: to source = {to_source}, to target = {to_target}"),
            Self::SelfDestructed{ total_damage } => format!("self-destructed for {total_damage} total damage"),
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
            Self::SelfDestructed { total_damage: _ } => true,
            _ => false,
        }
    }
}
