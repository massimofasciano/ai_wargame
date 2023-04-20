use crate::{UnitType, Player, Unit, DisplayFirstLetter};

use anyhow::anyhow;

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub enum BoardCell {
    #[default]
    Empty,
    Unit { player:Player, unit:Unit },
}

impl BoardCell {
    pub const fn new() -> Self {
        Self::Empty
    }
    pub fn new_unit(player: Player, unit_type: UnitType) -> Self {
        Self::Unit { 
            player,
            unit: Unit::new(unit_type),
        }
    }
    pub fn is_empty(&self) -> bool {
        *self == Self::Empty
    }
    pub fn is_unit(&self) -> bool {
        match self {
            Self::Unit { player: _, unit: _ } => true,
            _ => false,
        }
    }
    pub fn player(&self) -> Option<Player> {
        match self {
            Self::Unit { player, unit: _ } => Some(*player),
            _ => None,
        }
    }
    pub fn unit(&self) -> Option<(&Player,&Unit)> {
        match self {
            Self::Unit { player, unit } => Some((player,unit)),
            _ => None,
        }
    }
    pub fn unit_mut(&mut self) -> Option<(&mut Player,&mut Unit)> {
        match self {
            Self::Unit { player, unit } => Some((player,unit)),
            _ => None,
        }
    }
    pub fn is_dead(&self) -> bool {
        if let Some((_, unit)) = self.unit() {
            unit.health == 0
        } else {
            false
        }
    }
    pub fn to_pretty_compact_string(&self) -> String {
        if self.is_empty() {
            String::from(" . ")
        } else {
            self.to_compact_string()
        }
    }
    pub fn to_compact_string(&self) -> String {
        use BoardCell::*;
        match self {
            Empty => String::from(""),
            Unit { player, unit } => format!("{}{}{:1}",
                player.to_first_letter().to_ascii_lowercase(),
                unit.unit_type.to_first_letter().to_ascii_uppercase(),unit.health),
        }
    }
    pub fn to_ref_mut<'a>(&'a mut self) -> BoardCellRefMut<'a> {
        match self {
            Self::Empty => BoardCellRefMut::Empty,
            _ => BoardCellRefMut::Ref(self),
        }
    }
    // pub fn to_ref<'a>(&'a self) -> BoardCellRef<'a> {
    //     match self {
    //         Self::Empty => BoardCellRef::Empty,
    //         _ => BoardCellRef::Ref(self),
    //     }
    // }
}

impl std::fmt::Display for BoardCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_compact_string())
    }
}

#[derive(Debug, PartialEq, Default)]
pub enum BoardCellRefMut<'a> {
    #[default]
    Empty,
    Ref(&'a mut BoardCell),
}

impl<'a> BoardCellRefMut<'a> {
    pub fn is_empty(&self) -> bool {
        *self == Self::Empty
    }
    pub fn is_ref(&self) -> bool {
        !self.is_empty()
    }
    pub fn try_into_inner(self) -> Result<&'a mut BoardCell,anyhow::Error> {
        match self {
            Self::Ref(r) => Ok(r),
            Self::Empty => Err(anyhow!("can't get ref mut on empty cell")),
        }
    }
    pub fn into_inner(self) -> &'a mut BoardCell{
        self.try_into_inner().unwrap_or_else(|e|panic!("{}",e))
    }
    pub fn try_to_inner(&'a mut self) -> Result<&'a mut BoardCell,anyhow::Error> {
        match self {
            Self::Ref(r) => Ok(*r),
            Self::Empty => Err(anyhow!("can't get ref mut on empty cell")),
        }
    }
    pub fn to_inner(&'a mut self) -> &'a mut BoardCell{
        self.try_to_inner().unwrap_or_else(|e|panic!("{}",e))
    }
}

// #[derive(Debug, PartialEq, Default)]
// pub enum BoardCellRef<'a> {
//     #[default]
//     Empty,
//     Ref(&'a BoardCell),
// }

// impl<'a> BoardCellRef<'a> {
//     pub fn is_empty(&self) -> bool {
//         *self == Self::Empty
//     }
//     pub fn is_ref(&self) -> bool {
//         !self.is_empty()
//     }
//     pub fn to_inner(&self) -> &'a BoardCell{
//         match self {
//             Self::Ref(r) => r,
//             Self::Empty => panic!("can't get ref on empty cell"),
//         }
//     }
//     pub fn try_to_inner(&self) -> Result<&'a BoardCell,anyhow::Error> {
//         match self {
//             Self::Ref(r) => Ok(r),
//             Self::Empty => Err(anyhow!("can't get ref on empty cell")),
//         }
//     }
// }

// impl<'a> std::ops::Deref for BoardCellRef<'a> {
//     type Target = BoardCell;
//     fn deref(&self) -> &Self::Target {
//         self.to_inner()
//     }
// }
