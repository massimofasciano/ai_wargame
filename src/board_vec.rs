use crate::{Cell, Dim};

pub type BoardData = Vec<Cell>;

#[derive(Debug, PartialEq, Clone)]
pub struct Board {
    data: BoardData,
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
    pub const fn inner(&self) -> &Vec<Cell> {
        &self.data
    }
    pub fn inner_mut(&mut self) -> &mut Vec<Cell> {
        &mut self.data
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
