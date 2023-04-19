use crate::{Coord, Cell, DEFAULT_BOARD_DIM};

use duplicate::duplicate_item;

#[duplicate_item(I T; [const SIZE: usize] [crate::board_array::BoardArray<SIZE>]; [] [crate::board_vec::Board])]
impl<I> Default for T {
    fn default() -> Self {
        Self::new(DEFAULT_BOARD_DIM)
    }
}

#[duplicate_item(I T; [const SIZE: usize] [crate::board_array::BoardArray<SIZE>]; [] [crate::board_vec::Board])]
impl<I> std::fmt::Display for T {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = self.inner();
        for c in data.iter() {
            write!(f,":{}",c)?;
        }
        Ok(())
    }
}

#[duplicate_item(I T; [const SIZE: usize] [crate::board_array::BoardArray<SIZE>]; [] [crate::board_vec::Board])]
impl<I> std::ops::Index<Coord> for T {
    type Output = Cell;
    fn index(&self, coord: Coord) -> & Self::Output {
        let index = self.to_index(coord);
        let data = self.inner();
        data.index(index)
    }
}

#[duplicate_item(I T; [const SIZE: usize] [crate::board_array::BoardArray<SIZE>]; [] [crate::board_vec::Board])]
impl<I> std::ops::IndexMut<Coord> for T {
    fn index_mut(&mut self, coord: Coord) -> &mut Self::Output {
        let index = self.to_index(coord);
        let data = self.inner_mut();
        data.index_mut(index)
    }
}

#[duplicate_item(I T; [const SIZE: usize] [crate::board_array::BoardArray<SIZE>]; [] [crate::board_vec::Board])]
impl<I> T {
    const fn to_index(&self, (row, col): Coord) -> usize {
        let dim = self.dim() as usize;
        let row = row as usize;
        let col = col as usize;
        row * dim + col
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
        let data = self.inner();
        data.get(index)
    }
    pub fn get_mut(&mut self, coord: Coord) -> Option<&mut Cell> {
        let index = self.to_index(coord);
        let data = self.inner_mut();
        data.get_mut(index)
    }
    pub fn swap(&mut self, coord0: Coord, coord1: Coord) {
        let index0 = self.to_index(coord0);
        let index1 = self.to_index(coord1);
        let data = self.inner_mut();
        data.swap(index0,index1);
    }
    pub fn get_two_mut(&mut self, coord0: Coord, coord1: Coord) -> Option<[&mut Cell;2]> {
        let index0 = self.to_index(coord0);
        let index1 = self.to_index(coord1);
        if index0 == index1 || index0 >= self.len() || index1 >= self.len() {
            return None
        }
        let ref_mut_0;
        let ref_mut_1;
        let data = self.inner_mut();
        unsafe {
            ref_mut_0 = &mut *(data.get_unchecked_mut(index0) as *mut _);
            ref_mut_1 = &mut *(data.get_unchecked_mut(index1) as *mut _);
        }
        Some([ref_mut_0, ref_mut_1])
    }
    pub fn iter(&self) -> std::slice::Iter<Cell> {
        let data = self.inner();
        data.iter()
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<Cell> {
        let data = self.inner_mut();
        data.iter_mut()
    }
}
