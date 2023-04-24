use crate::{Game, BoardCell, Player, Unit, UnitType};

pub type HeuristicScore = i32;
pub type Heuristic = fn(&Game, Player)->HeuristicScore;

pub const MIN_HEURISTIC_SCORE : HeuristicScore = HeuristicScore::MIN;
pub const MAX_HEURISTIC_SCORE : HeuristicScore = HeuristicScore::MAX;

pub fn zero_heuristic(_game: &Game, _for_player : Player) -> HeuristicScore {
    0
}

pub fn units_heuristic(game: &Game, player : Player) -> HeuristicScore {
    game.units().map(|cell|cell_unit_type(cell,&player)).sum()
}

pub fn ai_distance_units_health_heuristic(game: &Game, player : Player) -> HeuristicScore {
    ai_distance_heuristic(game, player) + units_health_heuristic(game, player)
}

pub fn ai_distance_heuristic(game: &Game, player : Player) -> HeuristicScore {
    game.unit_coord_pairs().map(|pair| {
        let from_cell = game.get_cell(pair.from).expect("valid coord");
        let from_player = from_cell.player().expect("not empty");
        let to_cell = game.get_cell(pair.to).expect("valid coord");
        let to_player = to_cell.player().expect("not empty");
        let to_unit_type = to_cell.unit().expect("not empty").unit_type;
        let dist = pair.moves_distance() as HeuristicScore;
        if from_player == player && to_player != player && to_unit_type == UnitType::AI {
            if dist < 5 {
                // we want our units as close to opposing AI as possible
                dist
            } else {
                // don't count those that are too far as much
                dist/4
            }
        } else if from_player != player && to_player == player && to_unit_type == UnitType::AI {
            if dist < 5 {
                // we want opposing units as far to our AI as possible
                -dist
            } else {
                // don't count those that are too far as much
                -dist/4
            }
        } else {
            0
        }
    }).sum()
}

pub fn units_health_heuristic(game: &Game, player : Player) -> HeuristicScore {
    game.units().map(|cell|cell_unit_type_health(cell,&player,true,true)).sum()
}

pub fn units_health_friend_heuristic(game: &Game, player : Player) -> HeuristicScore {
    game.units().map(|cell|cell_unit_type_health(cell,&player,true,false)).sum()
}

pub fn units_health_opponent_heuristic(game: &Game, player : Player) -> HeuristicScore {
    game.units().map(|cell|cell_unit_type_health(cell,&player,false,true)).sum()
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

fn cell_unit_type_health(cell: &BoardCell, current_player: &Player, count_friend: bool, count_opponent: bool) -> HeuristicScore {
    if cell.is_empty() {
        0
    } else {
        let (player, unit) = cell.player_unit().expect("must call with a cell containing a unit");
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
    unit_type_score(&unit.unit_type)+unit.health as HeuristicScore
}

fn unit_score(unit: &Unit) -> HeuristicScore {
    unit_type_score(&unit.unit_type)
}

fn unit_type_score(unit_type: &UnitType) -> HeuristicScore {
    unit_type.score()
}
