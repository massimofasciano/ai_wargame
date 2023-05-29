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
    pub fn set_e1(&mut self) {
        self.set_attack_heuristics(score_heuristic());
        self.set_defense_heuristics(score_heuristic());
    }
    pub fn set_e2(&mut self) {
        self.set_attack_heuristics(Default::default());
        self.set_defense_heuristics(Default::default());
    }
    pub fn set_e3e4(&mut self) {
        self.set_attack_heuristics(default_attacker_heuristic());
        self.set_defense_heuristics(default_defender_heuristic());
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

pub fn score_heuristic() -> Heuristic {
    // simple score total by unit without health
    units_score_health_weights_bias(1,1,1,0,unit_score)
}

// pub fn default_attacker_heuristic() -> Heuristic {
//     units_score_health_weights_bias(1,1,50, 1, unit_score)
//         + ai_distance(2,1)
//         - game_moves()
// }
pub fn default_attacker_heuristic() -> Heuristic {
    units_score_health_weights_bias(1,1,50, 1, unit_score)
        + potential_health_delta()
        - game_moves()
}

pub fn default_defender_heuristic() -> Heuristic {
    units_score_health_weights_bias(1,1,10, 1, unit_score)
}

pub fn ai_distance(weight_friend: HeuristicScore, weight_opponent: HeuristicScore) -> Heuristic {
    Heuristic::new(move|game: &Game, player : Player| {
        game.unit_coord_pairs().map(|(pair,_,_)| {
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

pub fn potential_health_delta() -> Heuristic {
    Heuristic::new(move|game: &Game, player : Player| {
        let mut total_delta = 0;
        for (from, from_cell) in game.player_unit_coords(player) {
            let (from_player, from_unit) = from_cell.player_unit().expect("from cell should not be empty");
            for to in from.iter_neighbors() {
                if let Some(to_cell) = game.get_cell(to) {
                    if to_cell.is_empty() {
                        continue;
                    }
                    let (to_player, to_unit) = to_cell.player_unit().expect("to cell should not be empty");
                    total_delta += if from_player == to_player {
                        to_unit.unit_type.repair_amount(&from_unit.unit_type) as HeuristicScore
                        * unit_score(from_unit.unit_type)
                    } else {
                        from_unit.unit_type.damage_amount(&to_unit.unit_type) as HeuristicScore
                        * unit_score(to_unit.unit_type)
                        -
                        to_unit.unit_type.damage_amount(&from_unit.unit_type) as HeuristicScore
                        * unit_score(from_unit.unit_type)
                    };
                }
            }   
        }
        total_delta
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
        AI => 10,
        Virus => 3,
        Tech => 3,
        Firewall => 1,
        Program => 1,
    }
}

