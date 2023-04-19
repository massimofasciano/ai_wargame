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
    pub fn cardinality() -> usize {
        enum_iterator::cardinality::<Self>()
    }
}

impl DisplayFirstLetter for Player {}
