use crate::{Game, BoardCell, Player, Unit, UnitType};

pub type HeuristicScore = i32;
pub type Heuristic = fn(&Game, Player)->HeuristicScore;

pub fn zero_heuristic(_game: &Game, _for_player : Player) -> HeuristicScore {
    0
}

pub fn units_heuristic(game: &Game, for_player : Player) -> HeuristicScore {
    let current_player = for_player;
    game.units().map(|cell|cell_unit_type(cell,&current_player)).sum()
}

pub fn units_health_heuristic(game: &Game, for_player : Player) -> HeuristicScore {
    let current_player = for_player;
    game.units().map(|cell|cell_unit_type_health(cell,&current_player,true,true)).sum()
}

pub fn units_health_friend_heuristic(game: &Game, for_player : Player) -> HeuristicScore {
    let current_player = for_player;
    game.units().map(|cell|cell_unit_type_health(cell,&current_player,true,false)).sum()
}

pub fn units_health_opponent_heuristic(game: &Game, for_player : Player) -> HeuristicScore {
    let current_player = for_player;
    game.units().map(|cell|cell_unit_type_health(cell,&current_player,false,true)).sum()
}

pub fn units_distance_from_center_row(game: &Game, for_player : Player) -> HeuristicScore {
    let current_player = for_player;
    let player_coords = game.player_unit_coords(current_player);
    player_coords.map(|coord|{
        (game.dim()/2-coord.row-1) as HeuristicScore
    }).sum()
}

pub fn units_score_distance_center(game: &Game, for_player : Player) -> HeuristicScore {
    units_distance_from_center_row(game, for_player)+units_health_heuristic(game, for_player)
}

fn cell_unit_type_health(cell: &BoardCell, current_player: &Player, count_friend: bool, count_opponent: bool) -> HeuristicScore {
    if cell.is_empty() {
        0
    } else {
        let (player, unit) = cell.unit().expect("must call with a cell containing a unit");
        let score = unit_health_score(unit);
        if player == current_player {
            if count_friend { score } else { 0 }
        } else {
            if count_opponent { -score } else { 0 }
        }
    }
}

fn cell_unit_type(cell: &BoardCell, current_player: &Player) -> HeuristicScore {
    if cell.is_empty() {
        0
    } else {
        let (player, unit) = cell.unit().expect("must call with a cell containing a unit");
        let score = unit_score(unit);
        if player == current_player {
            score
        } else {
            -score
        }
    }
}

fn unit_health_score(unit: &Unit) -> HeuristicScore {
    unit_type_score(&unit.unit_type)+unit.health as HeuristicScore
}

fn unit_score(unit: &Unit) -> HeuristicScore {
    unit_type_score(&unit.unit_type)
}

fn unit_type_score(unit_type: &UnitType) -> HeuristicScore {
    use UnitType::*;
    match unit_type {
        AI => 30,
        Hacker => 10,
        Repair => 20,
        Tank => 10,
        Drone => 10,
        Soldier => 10,
    }
}
