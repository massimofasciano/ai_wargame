use std::num::TryFromIntError;

use crate::DisplayFirstLetter;
use enum_iterator::Sequence;

#[derive(Debug, PartialEq, PartialOrd, Default, Clone, Copy, Sequence)]
#[derive(derive_more::Display)]
pub enum Player {
    #[default]
    Blue,
    Red,
}

impl Player {
    pub fn next(&self) -> Self {
        enum_iterator::next_cycle(self).unwrap()
    }
    pub fn all() -> enum_iterator::All<Self> {
        enum_iterator::all()
    }
    pub const fn cardinality() -> usize {
        enum_iterator::cardinality::<Self>()
    }
    pub const fn index(&self) -> u8 {
        match self {
            Self::Blue => 0,
            Self::Red => 1,
        }
    }
}

impl TryFrom<u8> for Player {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Blue),
            1 => Ok(Self::Red),
            _ => Err(String::from("invalid index for player")),
        }
    }
}

impl DisplayFirstLetter for Player {}

