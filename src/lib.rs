pub mod game;
pub use game::Game;
pub mod board;
//pub use board::Board;
pub mod cell;
pub use cell::Cell;
pub mod unit;
pub use unit::Unit;
pub mod unit_type;
pub use unit_type::UnitType;
pub mod player;
pub use player::Player;

const BOARD_DIM: Dim = 10;
const BOARD_SIZE: usize = BOARD_DIM as usize*BOARD_DIM as usize;
const MAX_HEALTH : u8 = 9;

type Board = [Cell;BOARD_SIZE];
type Dim = i8;
type Coord = (Dim,Dim);
type Health = u8;

trait DisplayFirstLetter : std::fmt::Display {
    fn to_first_letter(&self) -> char {
        self.to_string().chars().next().unwrap()
    }
}
