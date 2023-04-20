use crate::DisplayFirstLetter;
use enum_iterator::Sequence;

#[derive(Debug, PartialEq, PartialOrd, Default, Clone, Copy, Sequence)]
#[derive(derive_more::Display)]
pub enum Player {
    #[default]
    Attacker,
    Defender,
}

impl Player {
    pub fn is_attacker(&self) -> bool {
        self == &Self::Attacker
    }
    pub fn is_defender(&self) -> bool {
        self == &Self::Defender
    }
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
            Self::Attacker => 0,
            Self::Defender => 1,
        }
    }
}

impl TryFrom<u8> for Player {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Attacker),
            1 => Ok(Self::Defender),
            _ => Err(String::from("invalid index for player")),
        }
    }
}

impl DisplayFirstLetter for Player {}

