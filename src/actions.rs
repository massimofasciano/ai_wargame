use crate::{Coord, Health, CoordPair};

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

impl Action {
    pub fn into_coord_pair(self) -> Option<CoordPair> {
        match self {
            Self::Pass 
                => None,
            Self::Move { from, to }
                | Self::Repair { from, to }
                | Self::Attack { from, to } 
                => Some(CoordPair::new(from, to)),
            Self::SelfDestruct { from } 
                => Some(CoordPair::new(from, from)),
        }
    }
}

impl Into<Option<CoordPair>> for Action {
    fn into(self) -> Option<CoordPair> {
        self.into_coord_pair()
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
        matches!(self, 
            Self::Damaged { to_source: _, to_target: _ } | 
            Self::Repaired { amount: _ } | 
            Self::SelfDestructed { total_damage: _ }
        )
    }
}
