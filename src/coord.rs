use std::ops::{Add, Sub, Neg, AddAssign, SubAssign};

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
    pub const fn from_tuple(coord: CoordTuple) -> Self {
        let (row,col) = coord;
        Self {row,col}
    }
    pub const fn from_dim_diag(dim: Dim) -> Self {
        Self::new(dim, dim)
    }
    pub const fn from_dim_horiz(dim: Dim) -> Self {
        Self::new(0, dim)
    }
    pub const fn from_dim_vert(dim: Dim) -> Self {
        Self::new(dim, 0)
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
    pub fn rect_around(&self, range: Dim) -> CoordPair {
        let diag = Coord::from_dim_diag(range);
        CoordPair::new(*self-diag,*self+diag)
    }
    pub fn is_in_range(&self, to : Coord, range: Dim) -> bool {
        let from = *self;
        range >= 0 &&
        ((to.row - from.row).abs() + (to.col - from.col).abs()) as Dim == range
    }
    pub fn iter_neighbors<'a>(&'a self) -> impl Iterator<Item = Coord> + 'a {
        let neighbor_coords = vec![(0,-1),(-1,0),(1,0),(0,1)];
        neighbor_coords.into_iter().map(|c|Coord::from_tuple(c)+*self)
    }
}

impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.try_to_string_as_letter_number().unwrap_or(self.to_string_as_tuple()))
    }
}

impl From<CoordTuple> for Coord {
    fn from(coord: CoordTuple) -> Self {
        Self::from_tuple(coord)
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

impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        self.row += rhs.row;
        self.col += rhs.col;
    }
}

impl SubAssign for Coord {
    fn sub_assign(&mut self, rhs: Self) {
        self.row -= rhs.row;
        self.col -= rhs.col;
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CoordPair {
    pub from: Coord,
    pub to: Coord,
}

impl CoordPair {
    pub fn new(from: impl Into<Coord>, to: impl Into<Coord>) -> Self {
        let from = from.into();
        let to = to.into();
        Self {from,to}
    }
    pub fn from_dim(dim: Dim) -> Self {
        let from = Coord::new(0, 0);
        let to = Coord::new(dim-1,dim-1);
        Self {from,to}
    }
    pub fn rect_iter(self) -> impl Iterator<Item = Coord> {
        self.row_iter().flat_map(move|row| self.col_iter().map(move|col| Coord::new(row,col)))
    }
    pub fn row_iter(self) -> Box<dyn Iterator<Item = Dim>> {
        if self.from.row > self.to.row {
            Box::new((self.to.row..=self.from.row).rev())
        } else {
            Box::new(self.from.row..=self.to.row)
        }
    }
    pub fn col_iter(self) -> Box<dyn Iterator<Item = Dim>> {
        if self.from.col > self.to.col {
            Box::new((self.to.col..=self.from.col).rev())
        } else {
            Box::new(self.from.col..=self.to.col)
        }
    }
    pub fn moves_distance(&self) -> Dim {
        (self.from.row - self.to.row).abs() + (self.from.col - self.to.col).abs()
    }
}

