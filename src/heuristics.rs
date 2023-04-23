use crate::{Game, BoardCell, Player, Unit, UnitType};

pub type HeuristicScore = i32;
pub type Heuristic = fn(&Game)->HeuristicScore;

pub fn win_heuristic(game: &Game) -> HeuristicScore {
    let current_player = game.player();
    let result = game.check_if_winner();
    match result {
        Some(Some(winner)) if winner == current_player => HeuristicScore::MAX, // win
        Some(Some(_)) => HeuristicScore::MIN, // lose
        Some(None) => 0, // draw
        None => 0, // not finished
    }
}

pub fn units_heuristic(game: &Game) -> HeuristicScore {
    let current_player = game.player();
    let result = game.check_if_winner();
    match result {
        Some(Some(winner)) if winner == current_player => HeuristicScore::MAX, // win
        Some(Some(_)) => HeuristicScore::MIN, // lose
        Some(None) => 0, // draw
        None => { // not finished
            game.units().map(|cell|cell_unit_type(cell,&current_player)).sum()
        }
    }
}

pub fn units_health_heuristic(game: &Game) -> HeuristicScore {
    let current_player = game.player();
    let result = game.check_if_winner();
    match result {
        Some(Some(winner)) if winner == current_player => HeuristicScore::MAX, // win
        Some(Some(_)) => HeuristicScore::MIN, // lose
        Some(None) => 0, // draw
        None => { // not finished
            game.units().map(|cell|cell_unit_type_health(cell,&current_player)).sum()
        }
    }
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
