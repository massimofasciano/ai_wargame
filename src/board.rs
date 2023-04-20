use crate::{Coord, BoardCell, Dim, DEFAULT_BOARD_DIM, BoardCellRefMut, CoordPair, Player};

use duplicate::duplicate_item;

pub mod array;
pub mod vec;

#[duplicate_item(I T D; [const SIZE: usize] [array::BoardArray<SIZE>] [array::BoardData<SIZE>]; [] [vec::Board] [vec::BoardData])]
impl<I> T {
    pub const fn len(&self) -> usize {
        let dim = self.dim as usize;
        dim * dim
    }
    pub const fn dim(&self) -> Dim {
        self.dim
    }
    pub const fn inner(&self) -> &D {
        &self.data
    }
    pub fn inner_mut(&mut self) -> &mut D {
        &mut self.data
    }
}

#[duplicate_item(I T; [const SIZE: usize] [array::BoardArray<SIZE>]; [] [vec::Board])]
impl<I> Default for T {
    fn default() -> Self {
        Self::new(DEFAULT_BOARD_DIM)
    }
}

#[duplicate_item(I T; [const SIZE: usize] [array::BoardArray<SIZE>]; [] [vec::Board])]
impl<I> T {
    const fn to_index(&self, coord: Coord) -> usize {
        let (row,col) = coord.to_tuple();
        let dim = self.dim() as usize;
        let row = row as usize;
        let col = col as usize;
        row * dim + col
    }
    pub fn remove(&mut self, coord: Coord) -> Option<BoardCell> {
        let index = self.to_index(coord);
        let data = self.inner_mut();
        if let Some(data_mut) = data.get_mut(index) {
            Some(std::mem::take(data_mut))
        } else {
            None
        }
    }
    pub fn get(&self, coord: Coord) -> Option<&BoardCell> {
        let index = self.to_index(coord);
        let data = self.inner();
        if let Some(data) = data.get(index) {
            Some(data)
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, coord: Coord) -> Option<BoardCellRefMut> {
        let index = self.to_index(coord);
        let data = self.inner_mut();
        if let Some(data_mut) = data.get_mut(index) {
            Some(data_mut.to_ref_mut())
        } else {
            None
        }
    }
    pub fn set(&mut self, coord: Coord, value: BoardCell) {
        let index = self.to_index(coord);
        let data = self.inner_mut();
        data[index] = value;
    }
    pub fn get_two_mut(&mut self, coord0: Coord, coord1: Coord) -> Option<[BoardCellRefMut;2]> {
        let index0 = self.to_index(coord0);
        let index1 = self.to_index(coord1);
        if index0 == index1 || index0 >= self.len() || index1 >= self.len() {
            return None
        }
        let ref_mut_0 : &mut BoardCell;
        let ref_mut_1 : &mut BoardCell;
        let data = self.inner_mut();
        unsafe {
            ref_mut_0 = &mut *(data.get_unchecked_mut(index0) as *mut _);
            ref_mut_1 = &mut *(data.get_unchecked_mut(index1) as *mut _);
        }
        Some([ref_mut_0.to_ref_mut(), ref_mut_1.to_ref_mut()])
    }
    pub fn iter_units(&self) -> impl Iterator<Item=&BoardCell> {
        let data = self.inner();
        data.iter().filter(|c|c.is_unit())
    }
    pub fn rect(&self) -> CoordPair {
        CoordPair::new(Coord::new(0,0),Coord::new(self.dim(), self.dim()))
    }
    pub fn rect_iter(&self) -> impl Iterator<Item = Coord> {
        self.rect().rect_iter()
    }
    pub fn empty_coords<'a>(&'a self) -> impl Iterator<Item = Coord> + 'a {
        self.rect_iter().filter(|&c|self.get(c).expect("valid coord").is_empty())
    }
    pub fn player_coords<'a>(&'a self, player: Player) -> impl Iterator<Item = Coord> + 'a {
        self.rect_iter().filter(move|&c|!self.get(c).expect("valid coord").is_empty() && self.get(c).expect("valid coord").player().unwrap() == player)
    }
}
