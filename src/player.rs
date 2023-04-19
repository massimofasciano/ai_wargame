use crate::DisplayFirstLetter;

#[derive(Debug, PartialEq, PartialOrd, Default, Clone, Copy)]
#[derive(derive_more::Display)]
pub enum Player {
    #[default]
    Blue,
    Red,
}

impl DisplayFirstLetter for Player {}
