use crate::{Coord, Cell, Dim, DEFAULT_BOARD_DIM};

#[derive(Debug, PartialEq, Clone)]
pub struct Board {
    data: Vec<Cell>,
    dim: Dim,
}

impl Board {
    pub fn size(&self) -> usize {
        self.data.capacity()
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub const fn dim(&self) -> Dim {
        self.dim
    }
    const fn to_index(&self, (row, col): Coord) -> usize {
        let dim = self.dim as usize;
        let row = row as usize;
        let col = col as usize;
        row * dim + col
    }
    pub fn new(dim: Dim) -> Self {
        assert!(dim > 0);
        let dimu = dim as usize;
        let cap = dimu * dimu;
        let mut data = Vec::with_capacity(cap);
        for _ in 0..cap {
            data.push(Default::default());
        }
        Self { dim, data }
    }
    pub fn remove(&mut self, coord: Coord) -> Option<Cell> {
        if let Some(cell) = self.get_mut(coord) {
            Some(std::mem::take(cell))
        } else {
            None
        }
    }
    pub fn get(&self, coord: Coord) -> Option<&Cell> {
        let index = self.to_index(coord);
        self.data.get(index)
    }
    pub fn get_mut(&mut self, coord: Coord) -> Option<&mut Cell> {
        let index = self.to_index(coord);
        self.data.get_mut(index)
    }
    pub fn swap(&mut self, coord0: Coord, coord1: Coord) {
        let index0 = self.to_index(coord0);
        let index1 = self.to_index(coord1);
        self.data.swap(index0,index1);
    }
    pub fn get_two_mut(&mut self, coord0: Coord, coord1: Coord) -> Option<[&mut Cell;2]> {
        let index0 = self.to_index(coord0);
        let index1 = self.to_index(coord1);
        if index0 == index1 || index0 >= self.len() || index1 >= self.len() {
            return None
        }
        let ref_mut_0;
        let ref_mut_1;
        unsafe {
            ref_mut_0 = &mut *(self.data.get_unchecked_mut(index0) as *mut _);
            ref_mut_1 = &mut *(self.data.get_unchecked_mut(index1) as *mut _);
        }
        Some([ref_mut_0, ref_mut_1])
    }
    pub fn iter(&self) -> std::slice::Iter<Cell> {
        self.data.iter()
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<Cell> {
        self.data.iter_mut()
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new(DEFAULT_BOARD_DIM)
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.data.iter() {
            write!(f,":{}",c)?;
        }
        Ok(())
    }
}

impl std::ops::Index<Coord> for Board {
    type Output = Cell;
    fn index(&self, coord: Coord) -> & Self::Output {
        self.data.index(self.to_index(coord))
    }
}

impl std::ops::IndexMut<Coord> for Board {
    fn index_mut(&mut self, coord: Coord) -> &mut Self::Output {
        self.data.index_mut(self.to_index(coord))
    }
}

