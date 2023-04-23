use crate::{UnitType, Player, Unit, DisplayFirstLetter};

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(transparent)]
pub struct BoardCell {
    data: Option<BoardCellData>,
}

impl Default for BoardCell {
    fn default() -> Self {
        // empty cell
        Self { data: None }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BoardCellData {
    Unit { player:Player, unit:Unit },
}

impl Default for BoardCellData {
    fn default() -> Self {
        Self::Unit { player: Default::default(), unit: Default::default() }
    }
}

impl BoardCellData {
    pub fn unit_mut(&mut self) -> Option<(&mut Player,&mut Unit)> {
        match self {
            BoardCellData::Unit { player, unit } => Some((player,unit)),
        }
    }
}

impl BoardCell {
    pub fn new() -> Self {
        // empty cell
        Self::default()
    }
    pub fn new_unit(player: Player, unit_type: UnitType) -> Self {
        Self { data : Some(
            BoardCellData::Unit { 
                player,
                unit: Unit::new(unit_type),
            }
        )}
    }
    pub fn is_empty(&self) -> bool {
        // None means Empty cell
        self.data.is_none()
    }
    pub fn is_unit(&self) -> bool {
        match self.data {
            Some(BoardCellData::Unit { player: _, unit: _ }) => true,
            _ => false,
        }
    }
    pub fn player(&self) -> Option<Player> {
        match self.data {
            Some(BoardCellData::Unit { player, unit: _ }) => Some(player),
            _ => None,
        }
    }
    pub fn unit(&self) -> Option<(&Player,&Unit)> {
        match &self.data {
            Some(BoardCellData::Unit { player, unit }) => Some((player,unit)),
            _ => None,
        }
    }
    pub fn unit_mut(&mut self) -> Option<(&mut Player,&mut Unit)> {
        match &mut self.data {
            Some(BoardCellData::Unit { player, unit }) => Some((player,unit)),
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
        match self.data {
            None => String::from(""),
            Some(BoardCellData::Unit { player, unit }) => format!("{}{}{:1}",
                player.to_first_letter().to_ascii_lowercase(),
                unit.unit_type.to_first_letter().to_ascii_uppercase(),unit.health),
        }
    }
    pub fn data(&mut self) -> Option<&BoardCellData> {
        match &self.data {
            None => None,
            Some(data) => Some(data),
        }
    }
    pub fn data_mut(&mut self) -> Option<&mut BoardCellData> {
        match &mut self.data {
            None => None,
            Some(data) => Some(data),
        }
    }
}

impl std::fmt::Display for BoardCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_compact_string())
    }
}

