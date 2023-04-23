use crate::Game;

pub type HeuristicScore = i32;

pub fn win_heuristic(game: &Game) -> HeuristicScore {
    if let Some(winner) = game.check_if_winner() {
        if let Some(player) = winner {
            if player == game.player() {
                // player wins
                HeuristicScore::MAX
            } else {
                // player loses
                HeuristicScore::MIN
            }
        } else {
            // draw
            0
        }
    } else {
        // game not finished
        0
    }
}
