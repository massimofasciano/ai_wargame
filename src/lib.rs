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

type Dim = i8;
pub use coord::{Coord, CoordPair, CoordTuple};
type Health = u8;
pub use game::Game;
pub use board::Board;
pub use cell::{BoardCell,BoardCellData};
pub use unit_type::UnitType;
pub use player::Player;
pub use actions::{Action, ActionOutcome, DropOutcome, IsUsefulInfo};
pub use heuristics::{HeuristicScore,win_heuristic};

const MAX_HEALTH : Health = 9;
pub const DEFAULT_BOARD_DIM : Dim = 10;
const DEFAULT_BOARD_SIZE : usize = DEFAULT_BOARD_DIM as usize * DEFAULT_BOARD_DIM as usize;
pub const DEFAULT_MAX_DEPTH : usize = 3;

trait DisplayFirstLetter : std::fmt::Display {
    fn to_first_letter(&self) -> char {
        self.to_string().chars().next().unwrap()
    }
}
