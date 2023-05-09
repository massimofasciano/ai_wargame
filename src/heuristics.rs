use crate::{Game, BoardCell, Player, UnitType};

use std::{ops::{Deref, Add, Mul, Sub, Neg}, sync::Arc};
use rand::Rng;

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

impl Sub for Heuristic {
    type Output = Heuristic;
    fn sub(self, rhs: Self) -> Self::Output {
        Heuristic::new(
            move|g:&Game,p:Player| self(g,p)-rhs(g,p)
        )
    }
}

impl Neg for Heuristic {
    type Output = Heuristic;
    fn neg(self) -> Self::Output {
        Heuristic::new(
            move|g:&Game,p:Player| -self(g,p)
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

impl Default for Heuristic {
    fn default() -> Self {
        units_score_health_weights_bias(1,1,100,1,unit_score) - game_moves() * 10
    }
}

impl std::fmt::Debug for Heuristic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Heuristic(function=...)")
    }
}

#[derive(Clone,Debug)]
pub struct Heuristics {
    pub attacker_max: Heuristic,
    pub attacker_min: Heuristic,
    pub defender_max: Heuristic,
    pub defender_min: Heuristic,
}

impl Heuristics {
    pub fn set_attack_heuristics(&mut self, h: Heuristic) {
        self.attacker_max = h.clone();
        self.defender_min = h;
    }
    pub fn set_defense_heuristics(&mut self, h: Heuristic) {
        self.defender_max = h.clone();
        self.attacker_min = h;
    }
}

impl Default for Heuristics {
    fn default() -> Self {
        Self { 
            attacker_max: default_attacker_heuristic(),
            attacker_min: default_defender_heuristic(),
            defender_max: default_defender_heuristic(), 
            defender_min: default_attacker_heuristic(),
        }
    }
}

pub fn simple_heuristic_1() -> Heuristic {
    // simple score total by unit without health
    units_score_health_weights_bias(1,1,1,0,unit_score_simple)
}

pub fn simple_heuristic_2() -> Heuristic {
    // simple health total by unit (all units same value)
    units_score_health_weights_bias(1,1,0,1,|_|1)
}

pub fn default_attacker_heuristic() -> Heuristic {
    units_score_health_weights_bias(10,10,100, 1, unit_score) * 10
        + ai_distance(2,1)
        - game_moves() * 100
}

pub fn default_defender_heuristic() -> Heuristic {
    units_score_health_weights_bias(10,10,10, 1, unit_score) * 10
}

pub fn ai_distance(weight_friend: HeuristicScore, weight_opponent: HeuristicScore) -> Heuristic {
    Heuristic::new(move|game: &Game, player : Player| {
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
                -dist * from_unit_type.damage_amount(&to_unit_type) as HeuristicScore * weight_friend
            } else if from_player != player && to_player == player && 
                from_unit_type != UnitType::AI && from_unit_type != UnitType::Tech && to_unit_type == UnitType::AI {
                dist * from_unit_type.damage_amount(&to_unit_type) as HeuristicScore * weight_opponent
            } else {
                0
            }
        }).sum::<HeuristicScore>()
    })
}

pub fn constant_value(value: HeuristicScore) -> Heuristic {
    Heuristic::new(move|_,_| value)
}

pub fn random_value(min: HeuristicScore, max: HeuristicScore) -> Heuristic {
    Heuristic::new(move|_,_| rand::thread_rng().gen_range(min..=max))
}

pub fn game_moves() -> Heuristic {
    Heuristic::new(|game: &Game,_| game.total_moves() as HeuristicScore)
}

pub fn units_score_health_weights_bias(weight_friend: HeuristicScore, 
    weight_opponent: HeuristicScore, bias_health: HeuristicScore, weight_health: HeuristicScore,
    score_fn: fn(UnitType) -> HeuristicScore) -> Heuristic 
{
    Heuristic::new(
        move|game:&Game,player:Player| 
            game.units().map(|cell|
                units_score_health_cell(cell,&player,weight_friend,weight_opponent,bias_health, weight_health, score_fn))
            .sum()
    )
}

fn units_score_health_cell(cell: &BoardCell, current_player: &Player, 
    weight_friend: HeuristicScore, weight_opponent: HeuristicScore, 
    bias_health: HeuristicScore, weight_health: HeuristicScore,
    score_fn: fn(UnitType) -> HeuristicScore) -> HeuristicScore 
{
    if cell.is_empty() {
        0
    } else {
        let (player, unit) = cell.player_unit().expect("must call with a cell containing a unit");
        let score = score_fn(unit.unit_type)*(bias_health+weight_health*unit.health as HeuristicScore);
        if player == current_player {
            weight_friend * score
        } else {
            weight_opponent * -score
        }
    }
}

pub fn unit_score(unit_type: UnitType) -> HeuristicScore {
    use UnitType::*;
    match unit_type {
        AI => 0, // already included in end of game score
        Virus => 30,
        Tech => 30,
        Firewall => 10,
        Program => 10,
    }
}

pub fn unit_score_simple(unit_type: UnitType) -> HeuristicScore {
    use UnitType::*;
    match unit_type {
        AI => 2,
        Virus => 2,
        Tech => 2,
        Firewall => 1,
        Program => 1,
    }
}
