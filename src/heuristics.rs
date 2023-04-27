use std::{ops::{Deref, Add, Mul}, sync::Arc};

use crate::{Game, BoardCell, Player, Unit, UnitType};

pub type HeuristicScore = i32;
pub type HeuristicFunction = fn(&Game, Player)->HeuristicScore;
pub type Heuristic = HeuristicFunction;

pub const MIN_HEURISTIC_SCORE : HeuristicScore = HeuristicScore::MIN;
pub const MAX_HEURISTIC_SCORE : HeuristicScore = HeuristicScore::MAX;

pub fn zero_heuristic(_game: &Game, _player : Player) -> HeuristicScore {
    0
}

pub fn units_heuristic(game: &Game, player : Player) -> HeuristicScore {
    game.units().map(|cell|cell_unit_type(cell,&player)).sum()
}

pub fn ai_distance_units_health_heuristic(game: &Game, player : Player) -> HeuristicScore {
    ai_distance_heuristic(game, player) + 5*units_health_heuristic(game, player)
}

pub fn ai_distance_heuristic(game: &Game, player : Player) -> HeuristicScore {
    game.unit_coord_pairs().map(|pair| {
        let from_cell = game.get_cell(pair.from).expect("valid coord");
        let from_player = from_cell.player().expect("not empty");
        let from_unit_type = from_cell.unit().expect("not empty").unit_type;
        let to_cell = game.get_cell(pair.to).expect("valid coord");
        let to_player = to_cell.player().expect("not empty");
        let to_unit_type = to_cell.unit().expect("not empty").unit_type;
        let dist = pair.moves_distance() as HeuristicScore;
        if from_player == player && to_player != player && 
            from_unit_type != UnitType::AI && from_unit_type != UnitType::Tech && to_unit_type == UnitType::AI {
            -dist * from_unit_type.damage_amount(&to_unit_type) as HeuristicScore
        } else if from_player != player && to_player == player && 
            from_unit_type != UnitType::AI && from_unit_type != UnitType::Tech && to_unit_type == UnitType::AI {
            dist * from_unit_type.damage_amount(&to_unit_type) as HeuristicScore / 2
        } else {
            0
        }
    }).sum()
}

pub fn units_health_heuristic(game: &Game, player : Player) -> HeuristicScore {
    game.units().map(|cell|cell_unit_type_health(cell,&player,1,2)).sum()
}

pub fn units_health_friend_heuristic(game: &Game, player : Player) -> HeuristicScore {
    game.units().map(|cell|cell_unit_type_health(cell,&player,1,0)).sum()
}

pub fn units_health_opponent_heuristic(game: &Game, player : Player) -> HeuristicScore {
    game.units().map(|cell|cell_unit_type_health(cell,&player,0,1)).sum()
}

fn cell_unit_type_health(cell: &BoardCell, current_player: &Player, weight_friend: HeuristicScore, weight_opponent: HeuristicScore) -> HeuristicScore {
    if cell.is_empty() {
        0
    } else {
        let (player, unit) = cell.player_unit().expect("must call with a cell containing a unit");
        let score = unit_health_score(unit);
        if player == current_player {
            weight_friend * score
        } else {
            weight_opponent * -score
        }
    }
}

fn cell_unit_type(cell: &BoardCell, current_player: &Player) -> HeuristicScore {
    if cell.is_empty() {
        0
    } else {
        let (player, unit) = cell.player_unit().expect("must call with a cell containing a unit");
        let score = unit.unit_type.score();
        if player == current_player {
            score
        } else {
            -score
        }
    }
}

// fn unit_health_score(unit: &Unit) -> HeuristicScore {
//     // health*value with bias (so that keeping units alive is better)
//     unit.unit_type.score()*(unit.health+3) as HeuristicScore
// }

fn unit_health_score(unit: &Unit) -> HeuristicScore {
    let score = unit.unit_type.score();
    score*(100+unit.health) as HeuristicScore
}


#[derive(Clone)]
pub struct HeuristicType {
    function : Arc<dyn HeuristicFn>,
}
impl HeuristicType {
    pub fn new(f: impl HeuristicFn + 'static) -> Self {
        Self { function: Arc::new(f) }
    }
}
impl Deref for HeuristicType {
    type Target = dyn HeuristicFn;
    fn deref(&self) -> &Self::Target {
        self.function.as_ref()
    }
}

pub trait HeuristicFn : Fn(&Game,Player) -> HeuristicScore {
    fn into_heuristic(self) -> HeuristicType where Self: Sized + 'static {
        HeuristicType::new(self)
    }
}
impl<T: Fn(&Game,Player) -> HeuristicScore> HeuristicFn for T {}

impl Add for HeuristicType {
    type Output = HeuristicType;
    fn add(self, rhs: Self) -> Self::Output {
        HeuristicType::new(
            move|g:&Game,p:Player| self(g,p)+rhs(g,p)
        )
    }
}

impl Mul<HeuristicScore> for HeuristicType {
    type Output = HeuristicType;
    fn mul(self, rhs: HeuristicScore) -> Self::Output {
        HeuristicType::new(
            move|g:&Game,p:Player| rhs*self(g,p)
        )
    }
}

impl<T : HeuristicFn + 'static> Add<T> for HeuristicType {
    type Output = HeuristicType;
    fn add(self, rhs: T) -> Self::Output {
        HeuristicType::new(
            move|g:&Game,p:Player| self(g,p)+rhs(g,p)
        )
    }
}

impl Mul for HeuristicType {
    type Output = HeuristicType;
    fn mul(self, rhs: Self) -> Self::Output {
        HeuristicType::new(
            move|g:&Game,p:Player| self(g,p)*rhs(g,p)
        )
    }
}

impl<T : HeuristicFn + 'static> Mul<T> for HeuristicType {
    type Output = HeuristicType;
    fn mul(self, rhs: T) -> Self::Output {
        HeuristicType::new(
            move|g:&Game,p:Player| self(g,p)*rhs(g,p)
        )
    }
}

pub fn test_heuristic_type(game: &Game, player : Player) -> HeuristicScore {
    let fb1 = (|g: &Game,p| ai_distance_units_health_heuristic(g,p)).into_heuristic();
    let fb2 = HeuristicType::new(ai_distance_units_health_heuristic);
    let fb4 = fb1 * 3 + fb2.clone() * 8 + fb2 * units_health_heuristic;
    fb4(game,player)
}

