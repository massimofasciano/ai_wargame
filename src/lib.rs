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
pub const DEFAULT_MAX_DEPTH : usize = 7;
pub const DEFAULT_MIN_DEPTH : usize = 4;
pub const DEFAULT_MAX_MOVES : usize = 100;
pub const DEFAULT_MAX_SECONDS : f32 = 5.0;

trait DisplayFirstLetter : std::fmt::Display {
    fn to_first_letter(&self) -> char {
        self.to_string().chars().next().unwrap()
    }
}

use number_prefix::{NumberPrefix, Amounts};

pub fn rescale_number_to_string<N: Amounts + std::fmt::Display>(number: N) -> String {
    match NumberPrefix::decimal(number) {
        NumberPrefix::Standalone(number) => {
            number.to_string()
        }
        NumberPrefix::Prefixed(unit, number) => {
            format!("{number:.1}{unit}")
        }
    }
}

pub fn number_digits_precision_to_string(float: f64, precision: usize) -> String {
    // compute absolute value
    let a = float.abs();

    // if abs value is greater than 1, then precision becomes less than "standard"
    let precision = if a >= 1. {
        // reduce by number of digits, minimum 0
        let n = (1. + a.log10().floor()) as usize;
        if n <= precision {
            precision - n
        } else {
            0
        }
    // if precision is less than 1 (but non-zero), then precision becomes greater than "standard"
    } else if a > 0. {
        // increase number of digits
        let n = -(1. + a.log10().floor()) as usize;
        precision + n
    // special case for 0
    } else {
        0
    };

    // format with the given computed precision
    format!("{0:.1$}", float, precision)
}
