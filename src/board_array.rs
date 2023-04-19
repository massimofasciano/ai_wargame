use crate::{Coord, Cell, Dim, DEFAULT_BOARD_SIZE, DEFAULT_BOARD_DIM};

pub type Board = BoardArray<DEFAULT_BOARD_SIZE>;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BoardArray<const SIZE: usize> {
    data: [Cell;SIZE],
    dim: Dim,
}

impl<const SIZE: usize> BoardArray<SIZE> {
    pub const fn size(&self) -> usize {
        SIZE
    }
    pub const fn len(&self) -> usize {
        let dim = self.dim as usize;
        dim * dim
    }
    pub const fn dim(&self) -> Dim {
        self.dim
    }
    pub const fn inner(&self) -> &[Cell;SIZE] {
        &self.data
    }
    pub fn inner_mut(&mut self) -> &mut [Cell;SIZE] {
        &mut self.data
    }
    // const fn to_index(&self, (row, col): Coord) -> usize {
    //     let dim = self.dim as usize;
    //     let row = row as usize;
    //     let col = col as usize;
    //     row * dim + col
    // }
    pub fn new(dim: Dim) -> Self {
        assert!(dim > 0);
        assert!(dim as usize*dim as usize <= SIZE,"{}x{} board will not fit in array of size {}",dim,dim,SIZE);
        Self {
            dim,
            data : [Default::default();SIZE],
        }
    }
    // pub fn remove(&mut self, coord: Coord) -> Option<Cell> {
    //     if let Some(cell) = self.get_mut(coord) {
    //         Some(std::mem::take(cell))
    //     } else {
    //         None
    //     }
    // }
    // pub fn get(&self, coord: Coord) -> Option<&Cell> {
    //     let index = self.to_index(coord);
    //     self.data.get(index)
    // }
    // pub fn get_mut(&mut self, coord: Coord) -> Option<&mut Cell> {
    //     let index = self.to_index(coord);
    //     self.data.get_mut(index)
    // }
    // pub fn swap(&mut self, coord0: Coord, coord1: Coord) {
    //     let index0 = self.to_index(coord0);
    //     let index1 = self.to_index(coord1);
    //     self.data.swap(index0,index1);
    // }
    // pub fn get_two_mut(&mut self, coord0: Coord, coord1: Coord) -> Option<[&mut Cell;2]> {
    //     let index0 = self.to_index(coord0);
    //     let index1 = self.to_index(coord1);
    //     if index0 == index1 || index0 >= SIZE || index1 >= SIZE {
    //         return None
    //     }
    //     let ref_mut_0;
    //     let ref_mut_1;
    //     unsafe {
    //         ref_mut_0 = &mut *(self.data.get_unchecked_mut(index0) as *mut _);
    //         ref_mut_1 = &mut *(self.data.get_unchecked_mut(index1) as *mut _);
    //     }
    //     Some([ref_mut_0, ref_mut_1])
    // }
    // pub fn iter(&self) -> std::slice::Iter<Cell> {
    //     self.data.iter()
    // }
    // pub fn iter_mut(&mut self) -> std::slice::IterMut<Cell> {
    //     self.data.iter_mut()
    // }
}

impl<const SIZE: usize> Default for BoardArray<SIZE> {
    fn default() -> Self {
        Self::new(DEFAULT_BOARD_DIM)
    }
}

impl<const SIZE: usize> std::fmt::Display for BoardArray<SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = self.inner();
        for c in data.iter() {
            write!(f,":{}",c)?;
        }
        Ok(())
    }
}

impl<const SIZE: usize> std::ops::Index<Coord> for BoardArray<SIZE> {
    type Output = Cell;
    fn index(&self, coord: Coord) -> & Self::Output {
        let index = self.to_index(coord);
        let data = self.inner();
        data.index(index)
    }
}

impl<const SIZE: usize> std::ops::IndexMut<Coord> for BoardArray<SIZE> {
    fn index_mut(&mut self, coord: Coord) -> &mut Self::Output {
        let index = self.to_index(coord);
        let data = self.inner_mut();
        data.index_mut(index)
    }
}

impl<const SIZE: usize> BoardArray<SIZE> {
    const fn to_index(&self, (row, col): Coord) -> usize {
        let dim = self.dim as usize;
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
