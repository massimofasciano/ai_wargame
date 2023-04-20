use std::ops::{Add, Sub, Neg};

use crate::Dim;

pub type CoordTuple = (Dim,Dim);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
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
    pub fn try_to_string_as_letter_number(&self) -> Result<String,()> {
        let (row,col) = (self.row,self.col);
        let row_char = if row < 0 {
            '?'
        } else if row < 26 {
            (row as u8 +'A' as u8) as char
        } else if row < 52 {
            (row as u8-26 +'a' as u8) as char
        } else {
            '?'
        };
        if col >= 0 && row_char != '?' {
            Ok(format!("{}{}", row_char, col))
        } else {
            Err(())
        }
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
        write!(f, "{}", self.try_to_string_as_letter_number().unwrap_or(self.to_string_as_tuple()))
    }
}

impl From<CoordTuple> for Coord {
    fn from(coord: CoordTuple) -> Self {
        Self::new_from_tuple(coord)
    }
}

impl Add for Coord {
    type Output = Coord;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.row+rhs.row, self.col+rhs.col)
    }
}

impl Sub for Coord {
    type Output = Coord;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.row-rhs.row, self.col-rhs.col)
    }
}

impl Neg for Coord {
    type Output = Coord;
    fn neg(self) -> Self::Output {
        Self::new(-self.row, -self.col)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CoordPair {
    pub from: Coord,
    pub to: Coord,
}

impl CoordPair {
    pub fn new(from: Coord, to: Coord) -> Self {
        Self {from,to}
    }
    pub fn rect_iter(self) -> impl Iterator<Item = Coord> {
        (self.from.row..self.to.row).flat_map(move|row| (self.from.col..self.to.col).map(move|col| Coord::new(row,col)))
    }
}

