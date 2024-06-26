use crate::{Coord, BoardCell, Dim, DEFAULT_BOARD_DIM, CoordPair, Player, BoardCellData};

use duplicate::duplicate_item;

const DEFAULT_BOARD_SIZE : usize = DEFAULT_BOARD_DIM as usize * DEFAULT_BOARD_DIM as usize;

pub mod array;
pub mod vec;

#[cfg(feature = "board_array")]
pub use array::Board;
#[cfg(feature = "board_vec")]
pub use vec::Board;

#[duplicate_item(I T D; [const SIZE: usize] [array::BoardArray<SIZE>] [array::BoardData<SIZE>]; [] [vec::Board] [vec::BoardData])]
impl<I> T {
    #[allow(clippy::len_without_is_empty)]
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
    const fn coord_to_index(&self, coord: Coord) -> usize {
        let (row,col) = coord.to_tuple();
        let dim = self.dim() as usize;
        let row = row as usize;
        let col = col as usize;
        row * dim + col
    }
    pub fn remove(&mut self, coord: Coord) -> Option<BoardCell> {
        let index = self.coord_to_index(coord);
        let data = self.inner_mut();
        data.get_mut(index).map(std::mem::take)
    }
    pub fn get(&self, coord: Coord) -> Option<&BoardCell> {
        let index = self.coord_to_index(coord);
        let data = self.inner();
        if let Some(data) = data.get(index) {
            Some(data)
        } else {
            None
        }
    }
    pub fn get_data_mut(&mut self, coord: Coord) -> Option<&mut BoardCellData> {
        let index = self.coord_to_index(coord);
        let data = self.inner_mut();
        if let Some(data_mut) = data.get_mut(index) {
            data_mut.data_mut()
        } else {
            None
        }
    }
    pub fn set(&mut self, coord: Coord, value: BoardCell) {
        let index = self.coord_to_index(coord);
        let data = self.inner_mut();
        data[index] = value;
    }
    pub fn get_two_data_mut(&mut self, coord0: Coord, coord1: Coord) -> Option<[&mut BoardCellData;2]> {
        let index0 = self.coord_to_index(coord0);
        let index1 = self.coord_to_index(coord1);
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
        if ref_mut_0.is_empty() || ref_mut_1.is_empty() {
            None
        } else {
            Some([ref_mut_0.data_mut().unwrap(), ref_mut_1.data_mut().unwrap()])
        }
    }
    pub fn iter_units(&self) -> impl Iterator<Item=&BoardCell> {
        let data = self.inner();
        data.iter().filter(|c|c.is_unit())
    }
    pub fn iter_player_units(&self, player: Player) -> impl Iterator<Item=&BoardCell> + '_ {
        let data = self.inner();
        data.iter().filter(move|cell|{
                cell.is_unit() && cell.player().unwrap() == player
        })
    }
    pub fn iter_unit_coords(&self) -> impl Iterator<Item=(Coord,&BoardCell)> + '_ {
        self.rect_iter().filter_map(|coord| {
            if let Some(cell) = self.get(coord) {
                if cell.is_unit() {
                    Some((coord,cell))
                } else {
                    None
                }
            } else {
                None
            }
        })
    }
    pub fn iter_player_unit_coords(&self, player: Player) -> impl Iterator<Item=(Coord,&BoardCell)> + '_ {
        self.rect_iter().filter_map(move|coord| {
            if let Some(cell) = self.get(coord) {
                if cell.is_unit() && cell.player().unwrap() == player {
                    Some((coord,cell))
                } else {
                    None
                }
            } else {
                None
            }
        })
    }
    pub fn rect(&self) -> CoordPair {
        CoordPair::from_dim(self.dim())
    }
    pub fn rect_iter(&self) -> impl Iterator<Item = Coord> {
        self.rect().rect_iter()
    }
    pub fn empty_coords(&self) -> impl Iterator<Item = Coord> + '_ {
        self.rect_iter().filter(|&c|self.get(c).expect("valid coord").is_empty())
    }
    pub fn player_coords(&self, player: Player) -> impl Iterator<Item = Coord> + '_ {
        self.rect_iter().filter(move|&c|!self.get(c).expect("valid coord").is_empty() && self.get(c).expect("valid coord").player().unwrap() == player)
    }
}
