use crate::{Dim, CoordTuple};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Coord {
    pub row: Dim,
    pub col: Dim,
}

impl Coord {
    pub const fn new(row: Dim, col: Dim) -> Self {
        Self {row,col}
    }
    pub const fn new_from_tuple(coord: CoordTuple) -> Self {
        let (row,col) = coord;
        Self {row,col}
    }
    pub fn to_string_as_letter_number(&self) -> String {
        let (row,col) = (self.row,self.col);
        let row_char = if row < 26 {
            (row as u8 +'A' as u8) as char
        } else if row < 52 {
            (row as u8-26 +'a' as u8) as char
        } else {
            '?'
        };
        format!("{}{}", row_char, col)
    }
    pub fn to_string_as_tuple(&self) -> String {
        let (row,col) = (self.row,self.col);
        format!("({},{})", row, col)
    }
    pub const fn to_tuple(&self) -> CoordTuple {
        (self.row,self.col)
    }
}

impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string_as_letter_number())
    }
}

impl From<CoordTuple> for Coord {
    fn from(coord: CoordTuple) -> Self {
        Self::new_from_tuple(coord)
    }
}
