use crate::{Cell, Dim, DEFAULT_BOARD_SIZE};

pub type Board = BoardArray<DEFAULT_BOARD_SIZE>;

pub type BoardData<const SIZE: usize> = [Cell;SIZE];

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BoardArray<const SIZE: usize> {
    data: BoardData<SIZE>,
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
    pub const fn inner(&self) -> &BoardData<SIZE> {
        &self.data
    }
    pub fn inner_mut(&mut self) -> &mut BoardData<SIZE> {
        &mut self.data
    }
    pub fn new(dim: Dim) -> Self {
        assert!(dim > 0);
        assert!(dim as usize*dim as usize <= SIZE,"{}x{} board will not fit in array of size {}",dim,dim,SIZE);
        Self {
            dim,
            data : [Default::default();SIZE],
        }
    }
}

