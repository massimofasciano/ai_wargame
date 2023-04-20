use crate::{Coord, Game};

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
                format!("{} from {} to {}","moves",Game::coord_to_string(*from),Game::coord_to_string(*to)),
            Self::Repair { from, to } => 
                format!("{} from {} to {}","repairs",Game::coord_to_string(*from),Game::coord_to_string(*to)),
            Self::Attack { from, to } => 
                format!("{} from {} to {}","attacks",Game::coord_to_string(*from),Game::coord_to_string(*to)),
        })
    }
}
