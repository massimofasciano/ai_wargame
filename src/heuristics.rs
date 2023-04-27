use std::{ops::{Deref, Add, Mul}, sync::Arc};

use crate::{Game, BoardCell, Player, UnitType};

pub type HeuristicScore = i32;

pub const MIN_HEURISTIC_SCORE : HeuristicScore = HeuristicScore::MIN;
pub const MAX_HEURISTIC_SCORE : HeuristicScore = HeuristicScore::MAX;

#[derive(Clone)]
pub struct Heuristic {
    function : Arc<dyn HeuristicFn>,
}
impl Heuristic {
    pub fn new(f: impl HeuristicFn + 'static) -> Self {
        Self { function: Arc::new(f) }
    }
}
impl Deref for Heuristic {
    type Target = dyn HeuristicFn;
    fn deref(&self) -> &Self::Target {
        self.function.as_ref()
    }
}

pub trait HeuristicFn : Sync + Send + Fn(&Game,Player) -> HeuristicScore {
    fn into_heuristic(self) -> Heuristic where Self: Sized + 'static {
        Heuristic::new(self)
    }
}
impl<T: Sync + Send + Fn(&Game,Player) -> HeuristicScore> HeuristicFn for T {}

impl Add for Heuristic {
    type Output = Heuristic;
    fn add(self, rhs: Self) -> Self::Output {
        Heuristic::new(
            move|g:&Game,p:Player| self(g,p)+rhs(g,p)
        )
    }
}

impl Mul<HeuristicScore> for Heuristic {
    type Output = Heuristic;
    fn mul(self, rhs: HeuristicScore) -> Self::Output {
        Heuristic::new(
            move|g:&Game,p:Player| rhs*self(g,p)
        )
    }
}

impl<T : HeuristicFn + 'static> Add<T> for Heuristic {
    type Output = Heuristic;
    fn add(self, rhs: T) -> Self::Output {
        Heuristic::new(
            move|g:&Game,p:Player| self(g,p)+rhs(g,p)
        )
    }
}

impl Mul for Heuristic {
    type Output = Heuristic;
    fn mul(self, rhs: Self) -> Self::Output {
        Heuristic::new(
            move|g:&Game,p:Player| self(g,p)*rhs(g,p)
        )
    }
}

impl<T : HeuristicFn + 'static> Mul<T> for Heuristic {
    type Output = Heuristic;
    fn mul(self, rhs: T) -> Self::Output {
        Heuristic::new(
            move|g:&Game,p:Player| self(g,p)*rhs(g,p)
        )
    }
}

#[derive(Clone)]
pub struct Heuristics {
    pub attacker: Heuristic,
    pub defender: Heuristic,
}

impl Heuristics {
    fn new(attacker: Heuristic, defender: Heuristic) -> Self {
        Self { attacker, defender }
    }
}
impl Default for Heuristics {
    fn default() -> Self {
        Self::new(units_health(),units_health())
    }
}

impl std::fmt::Debug for Heuristics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:#?}",vec!["attacker_heuristic","defender_heuristic"])
    }
}

pub fn units_health() -> Heuristic {
    units_health_weights_bias(1,2,100)
}

pub fn ai_distance_units_health() -> Heuristic {
    units_health() * 5 + ai_distance() 
}

pub fn ai_distance() -> Heuristic {
    Heuristic::new(|game: &Game, player : Player| {
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
        }).sum::<HeuristicScore>()
    })
}

pub fn const_heuristic(value: HeuristicScore) -> Heuristic {
    Heuristic::new(move|_,_| value)
}
pub fn zero_heuristic() -> Heuristic {
    const_heuristic(0)
}

pub fn units_health_weights_bias(weight_friend: HeuristicScore, weight_opponent: HeuristicScore, health_bias: HeuristicScore) -> Heuristic {
    Heuristic::new(
        move|game:&Game,player:Player| 
            game.units().map(|cell|
                units_health_cell(cell,&player,weight_friend,weight_opponent,health_bias))
            .sum()
    )
}

fn units_health_cell(cell: &BoardCell, current_player: &Player, weight_friend: HeuristicScore, weight_opponent: HeuristicScore, health_bias: HeuristicScore) -> HeuristicScore {
    if cell.is_empty() {
        0
    } else {
        let (player, unit) = cell.player_unit().expect("must call with a cell containing a unit");
        let score = unit.unit_type.score()*(health_bias+unit.health as HeuristicScore);
        if player == current_player {
            weight_friend * score
        } else {
            weight_opponent * -score
        }
    }
}

