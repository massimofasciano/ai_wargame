use crate::{Game, BoardCell};

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
            let friendly : HeuristicScore = game.player_units(current_player).map(BoardCell::score).sum();
            let opposing : HeuristicScore = game.player_units(current_player.next()).map(BoardCell::score).sum();
            friendly-opposing
        }
    }
}
