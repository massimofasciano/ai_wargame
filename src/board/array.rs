use crate::{BoardCell, Dim, DEFAULT_BOARD_SIZE};

pub type Board = BoardArray<DEFAULT_BOARD_SIZE>;

pub type BoardData<const SIZE: usize> = [BoardCell;SIZE];

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BoardArray<const SIZE: usize> {
    pub (super) data: BoardData<SIZE>,
    pub (super) dim: Dim,
}

impl<const SIZE: usize> BoardArray<SIZE> {
    pub const fn size(&self) -> usize {
        SIZE
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
