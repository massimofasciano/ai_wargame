use ai_wargame::{Game, UnitType, Unit, Cell};

fn main() {
    let mut game = Game::new();
    loop {
        println!("{}",game);
        if let Some((from,to)) = game.get_move_from_stdin() {
            if !game.perform_action(from, to) {
                println!("Invalid move!");
            }
        }
        if let Some(winner) = game.winner() {
            println!("{} wins!",winner);
            break;
        }
    }
}