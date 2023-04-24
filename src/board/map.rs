use std::collections::HashMap;

use crate::{Coord, BoardCell, Dim, DEFAULT_BOARD_DIM, Player, CoordPair, BoardCellData};

type BoardData = HashMap<Coord,BoardCell>;

#[derive(Debug, Clone)]
pub struct Board {
    data: BoardData,
    dim: Dim,
    empty_cell: BoardCell, 
}

impl Board {
    pub fn new(dim: Dim) -> Self {
        assert!(dim > 0);
        let data = HashMap::new();
        let empty_cell = BoardCell::new();
        Self { dim, data, empty_cell }
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

impl Board {
    fn is_valid_position(&self, coord : Coord) -> bool {
        let (row,col) = coord.to_tuple();
        let is_valid = row >= 0 && col >= 0 && row < self.dim && col < self.dim;
        debug_assert_eq!(is_valid,true,"({},{}) is not valid for a {}x{} board",row,col,self.dim,self.dim);
        is_valid
    }
    pub fn remove(&mut self, coord: Coord) -> Option<BoardCell> {
        if self.is_valid_position(coord) {
            if let Some(cell_ref) = self.data.get_mut(&coord) {
                let old_cell = std::mem::take(cell_ref);
                self.data.remove(&coord);
                Some(old_cell)
            } else {
                Some(BoardCell::new())
            }
        } else {
            None
        }
    }
    pub fn get(&self, coord: Coord) -> Option<&BoardCell> {
        if self.is_valid_position(coord) {
            if let Some(cell_ref) = self.data.get(&coord) {
                if cell_ref.is_empty() {
                    Some(&self.empty_cell)
                } else {
                    Some(cell_ref)
                }
            } else {
                Some(&self.empty_cell)
            }
        } else {
            None
        }
    }
    pub fn get_data_mut(&mut self, coord: Coord) -> Option<&mut BoardCellData> {
        if let Some(cell_ref) = self.data.get_mut(&coord) {
            cell_ref.data_mut()
        } else {
            None
        }
    }
    pub fn set(&mut self, coord: Coord, value: BoardCell) {
        if !value.is_empty() {
            self.data.insert(coord, value);
        }
    }
    pub fn get_two_data_mut(&mut self, coord0: Coord, coord1: Coord) -> Option<[&mut BoardCellData;2]> {
        if coord0 == coord1 {
            return None;
        }
        let ref_mut_0;
        let ref_mut_1 ;
        if let Some(ref_mut) = self.get_data_mut(coord0) {
            unsafe {
                ref_mut_0 = &mut *(ref_mut as *mut _);
            }
        } else {
            return None;
        }
        if let Some(ref_mut) = self.get_data_mut(coord1) {
            unsafe {
                ref_mut_1 = &mut *(ref_mut as *mut _);
            }
        } else {
            return None;
        }
        Some([ref_mut_0, ref_mut_1])
    }
    pub fn iter_units(&self) -> std::collections::hash_map::Values<Coord, BoardCell> {
        self.data.values()
    }
    pub fn iter_player_units<'a>(&'a self, player: Player) -> impl Iterator<Item=&BoardCell> + 'a {
        self.data.values().filter(move|cell|{
                cell.is_unit() && cell.player().unwrap() == player
        })
    }
    pub fn iter_unit_coords<'a>(&'a self) -> impl Iterator<Item=Coord> + 'a {
        self.data.keys().map(Coord::clone)
    }
    pub fn iter_player_unit_coords<'a>(&'a self, player: Player) -> impl Iterator<Item=Coord> + 'a {
        self.data.iter().filter_map(move|(coord, cell)|{
            if cell.is_unit() && cell.player().unwrap() == player {
                Some(coord)
            } else {
                None
            }
        }).map(Coord::clone)
    }
    pub fn rect(&self) -> CoordPair {
        CoordPair::from_dim(self.dim())
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
