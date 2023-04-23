use crate::{Game, BoardCell, Player, Unit, UnitType};

pub type HeuristicScore = i32;
pub type Heuristic = fn(&Game)->HeuristicScore;

pub fn win_heuristic(_game: &Game) -> HeuristicScore {
    0
}

pub fn units_heuristic(game: &Game) -> HeuristicScore {
    let current_player = game.player();
    game.units().map(|cell|cell_unit_type(cell,&current_player)).sum()
}

pub fn units_health_heuristic(game: &Game) -> HeuristicScore {
    let current_player = game.player();
    game.units().map(|cell|cell_unit_type_health(cell,&current_player)).sum()
}

pub fn units_distance_from_center_row(game: &Game) -> HeuristicScore {
    let current_player = game.player();
    let player_coords = game.player_unit_coords(current_player);
    player_coords.map(|coord|{
        (game.dim()/2-coord.row-1) as HeuristicScore
    }).sum()
}

pub fn units_score_distance_center(game: &Game) -> HeuristicScore {
    units_distance_from_center_row(game)+units_health_heuristic(game)
}

fn cell_unit_type_health(cell: &BoardCell, current_player: &Player) -> HeuristicScore {
    if cell.is_empty() {
        0
    } else {
        let (player, unit) = cell.unit().expect("must call with a cell containing a unit");
        let score = unit_health_score(unit);
        if player == current_player {
            score
        } else {
            -score
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
