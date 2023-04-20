use std::collections::HashMap;

use crate::{Coord, BoardCell, Dim, DEFAULT_BOARD_DIM, BoardCellRefMut, Player, CoordPair};

type BoardData = HashMap<Coord,BoardCell>;

#[derive(Debug, Clone)]
pub struct Board {
    data: BoardData,
    dim: Dim,
}

impl Board {
    pub fn new(dim: Dim) -> Self {
        assert!(dim > 0);
        let data = HashMap::new();
        Self { dim, data }
    }
}

impl Board {
    pub fn size(&self) -> usize {
        self.data.len()
    }
    pub const fn len(&self) -> usize {
        let dim = self.dim as usize;
        dim * dim
    }
    pub const fn dim(&self) -> Dim {
        self.dim
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new(DEFAULT_BOARD_DIM)
    }
}

static STATIC_EMPTY_BOARD_CELL : BoardCell = BoardCell::Empty;

impl Board {
    pub fn remove(&mut self, coord: Coord) -> Option<BoardCell> {
        if let Some(cell_ref) = self.get_mut(coord) {
            if cell_ref.is_empty() {
                Some(BoardCell::new())
            } else {
                Some(std::mem::take(cell_ref.into_inner()))
            }
        } else {
            None
        }
    }
    pub fn get(&self, coord: Coord) -> Option<&BoardCell> {
        if let Some(cell_ref) = self.data.get(&coord) {
            if cell_ref.is_empty() {
                Some(&STATIC_EMPTY_BOARD_CELL)
            } else {
                Some(cell_ref)
            }
        } else {
            Some(&STATIC_EMPTY_BOARD_CELL)
        }
    }
    pub fn get_mut(&mut self, coord: Coord) -> Option<BoardCellRefMut> {
        if let Some(cell_ref) = self.data.get_mut(&coord) {
            Some(cell_ref.to_ref_mut())
        } else {
            None
        }
    }
    pub fn set(&mut self, coord: Coord, value: BoardCell) {
        if !value.is_empty() {
            self.data.insert(coord, value);
        }
    }
    pub fn get_two_mut(&mut self, coord0: Coord, coord1: Coord) -> Option<[BoardCellRefMut;2]> {
        if coord0 == coord1 {
            return None
        }
        let mut ref_mut_0 = BoardCellRefMut::Empty;
        let mut ref_mut_1 = BoardCellRefMut::Empty;
        if let Some(ref_mut) = self.data.get_mut(&coord0) {
            unsafe {
                let ref_mut : &mut BoardCell = &mut *(ref_mut as *mut _);
                ref_mut_0 = ref_mut.to_ref_mut()
            }
        }
        if let Some(ref_mut) = self.data.get_mut(&coord1) {
            unsafe {
                let ref_mut : &mut BoardCell = &mut *(ref_mut as *mut _);
                ref_mut_1 = ref_mut.to_ref_mut()
            }
        }
        Some([ref_mut_0, ref_mut_1])
    }
    pub fn iter_units(&self) -> std::collections::hash_map::Values<Coord, BoardCell> {
        self.data.values()
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
        self.data.iter().filter_map(move|(&coord,cell)|
            if cell.player().expect("should be a unit") == player { Some(coord) } else { None }
        )
    }
}
