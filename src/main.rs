use ai_wargame::Game;

use rand::Rng;

fn main() {
    let mut game = Game::new();
    loop {
        println!("{}",game);

        if let Some(winner) = game.winner() {
            println!("{} in {} moves!", if winner.is_none() {
                "draw".to_string()
            } else {
                format!("{} wins",winner.unwrap())
            }, game.total_moves());
            break;
        }

        loop {
            let md = game.dim();
            let from = (rand::thread_rng().gen_range(0..md),rand::thread_rng().gen_range(0..md));
            let to = (rand::thread_rng().gen_range(0..md),rand::thread_rng().gen_range(0..md));
            if !game.perform_action(from, to) {
                // println!("Invalid move!");
                continue;
            }
            break;
        }

        // if let Some((from,to)) = game.get_move_from_stdin() {
        //     if !game.perform_action(from, to) {
        //         println!("Invalid move!");
        //     }
        // }

    }
}