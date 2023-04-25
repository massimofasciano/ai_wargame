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
    ai_distance_heuristic(game, player)/5 + units_health_heuristic(game, player)
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
            from_unit_type != UnitType::AI && from_unit_type != UnitType::Repair && to_unit_type == UnitType::AI {
            -dist * from_unit_type.damage_amount(&to_unit_type) as HeuristicScore
        } else if from_player != player && to_player == player && 
            from_unit_type != UnitType::AI && from_unit_type != UnitType::Repair && to_unit_type == UnitType::AI {
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

pub fn units_distance_from_center_row(game: &Game, player : Player) -> HeuristicScore {
    let player_coords = game.player_unit_coords(player);
    player_coords.map(|coord|{
        (game.dim()/2-coord.row-1) as HeuristicScore
    }).sum()
}

pub fn units_score_distance_center(game: &Game, player : Player) -> HeuristicScore {
    units_distance_from_center_row(game, player)+units_health_heuristic(game, player)
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
        let score = unit_score(unit);
        if player == current_player {
            score
        } else {
            -score
        }
    }
}

fn unit_health_score(unit: &Unit) -> HeuristicScore {
    // health*value with bias (so that keeping units alive is better)
    unit_type_score(&unit.unit_type)*(unit.health+3) as HeuristicScore
}

fn unit_score(unit: &Unit) -> HeuristicScore {
    unit_type_score(&unit.unit_type)
}

fn unit_type_score(unit_type: &UnitType) -> HeuristicScore {
    unit_type.score()
}

//
// a test with traits...
//
// pub trait HeuristicFn : Fn(&Game,Player) -> HeuristicScore {
//     fn clone_box<'a>(&self) -> Box<dyn HeuristicFn + 'a> where Self: 'a;
// }
// impl<F> HeuristicFn for F where F: Clone + Fn(&Game,Player) -> HeuristicScore,
// {
//     fn clone_box<'a>(&self) -> Box<dyn HeuristicFn + 'a> where Self: 'a,
//     {
//         Box::new(self.clone())
//     }
// }
// impl<'a> Clone for Box<dyn HeuristicFn + 'a> {
//     fn clone(&self) -> Self {
//         (**self).clone_box()
//     }
// }
// pub type HeuristicFnBox = Box<dyn HeuristicFn>;

// pub fn test(h : HeuristicFnBox, game: &Game, player : Player) -> HeuristicScore {
//     let f = h.clone();
//     f(game,player)
// }

// pub fn test2(game: &Game, player : Player) -> HeuristicScore {
//     let h = Box::new(zero_heuristic);
//     test(h, game, player)
// }

// pub fn test3(game: &Game, player : Player) -> HeuristicScore {
//     let h = Box::new(|game:&Game,player|game.player_score(player));
//     test(h, game, player)
// }

