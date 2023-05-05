pub mod game;
pub mod board;
pub mod cell;
pub mod unit;
pub use unit::Unit;
pub mod unit_type;
pub mod player;
pub mod actions;
pub mod coord;
pub mod heuristics;

pub type Dim = i8;
pub use coord::{Coord, CoordPair, CoordTuple};
type Health = u8;
pub use game::{Game,GameOptions};
pub use board::Board;
pub use cell::{BoardCell,BoardCellData};
pub use unit_type::UnitType;
pub use player::Player;
pub use actions::{Action, ActionOutcome, IsUsefulInfo};
pub use heuristics::{HeuristicScore,Heuristics};

const MAX_HEALTH : Health = 9;
pub const DEFAULT_BOARD_DIM : Dim = 5;
pub const DEFAULT_MAX_DEPTH : usize = 6;
pub const DEFAULT_MIN_DEPTH : usize = 4;

trait DisplayFirstLetter : std::fmt::Display {
    fn to_first_letter(&self) -> char {
        self.to_string().chars().next().unwrap()
    }
}
