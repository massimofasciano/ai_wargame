pub mod game;
pub use game::Game;
pub mod board_array;
// pub use board_array::Board;
pub mod board_vec;
pub use board_vec::Board;
pub mod cell;
pub use cell::Cell;
pub mod unit;
pub use unit::Unit;
pub mod unit_type;
pub use unit_type::UnitType;
pub mod player;
pub use player::Player;

type Dim = i8;
type Coord = (Dim,Dim);
type Health = u8;

const MAX_HEALTH : Health = 9;
pub const DEFAULT_BOARD_DIM : Dim = 10;
const DEFAULT_BOARD_SIZE : usize = DEFAULT_BOARD_DIM as usize * DEFAULT_BOARD_DIM as usize;

trait DisplayFirstLetter : std::fmt::Display {
    fn to_first_letter(&self) -> char {
        self.to_string().chars().next().unwrap()
    }
}
