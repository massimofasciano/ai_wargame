pub mod game;
pub mod board;
pub mod cell;
pub mod unit;
pub use unit::Unit;
pub mod unit_type;
pub mod player;
pub mod actions;
pub mod coord;

// pub use board::array::Board;
pub use board::vec::Board;

type Dim = i8;
type CoordTuple = (Dim,Dim);
// type Coord = CoordTuple;
pub use coord::Coord;
type Health = u8;
pub use game::Game;
pub use cell::Cell;
pub use unit_type::UnitType;
pub use player::Player;

const MAX_HEALTH : Health = 9;
pub const DEFAULT_BOARD_DIM : Dim = 10;
const DEFAULT_BOARD_SIZE : usize = DEFAULT_BOARD_DIM as usize * DEFAULT_BOARD_DIM as usize;

trait DisplayFirstLetter : std::fmt::Display {
    fn to_first_letter(&self) -> char {
        self.to_string().chars().next().unwrap()
    }
}
