use crate::{Cell, Dim};

pub type BoardData = Vec<Cell>;

#[derive(Debug, PartialEq, Clone)]
pub struct Board {
    pub (super) data: BoardData,
    pub (super) dim: Dim,
}

impl Board {
    pub fn size(&self) -> usize {
        self.data.capacity()
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
}
