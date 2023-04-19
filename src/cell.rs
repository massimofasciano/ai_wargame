use crate::{UnitType, Player, Unit, DisplayFirstLetter};

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub enum Cell {
    #[default]
    Empty,
    Unit { player:Player, unit:Unit },
}

impl Cell {
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
    pub fn remove_dead(&mut self) {
        if let Some((_, unit)) = self.unit() {
            if unit.health == 0 {
                *self = Cell::Empty;
            }
        }
    }
    pub fn interact(&mut self, target: &mut Self) {
        let (player_source,unit_source) = self.unit_mut().unwrap();
        let (player_target,unit_target) = target.unit_mut().unwrap();
        if player_source != player_target {
            // it's an opposing unit so we try to damage it (it will damage us back)
            unit_source.apply_damage(unit_target);
            unit_target.apply_damage(unit_source);
            self.remove_dead();
            target.remove_dead();
        } else {
            // it's our unit so we try to repair it
            unit_source.apply_repair(unit_target);
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
        use Cell::*;
        match self {
            Empty => String::from(""),
            Unit { player, unit } => format!("{}{}{:1}",
                player.to_first_letter().to_ascii_lowercase(),
                unit.unit_type.to_first_letter().to_ascii_uppercase(),unit.health),
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_compact_string())
    }
}
