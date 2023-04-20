use ai_wargame::{Game, DEFAULT_BOARD_DIM, Coord};

fn main() {
    let dim = DEFAULT_BOARD_DIM;
    // let drop_prob = None;
    let drop_prob = Some(0.05);
    let mut game = Game::new(dim, drop_prob);
    let play_random = std::env::args().len() > 1;

    loop {
        println!();
        game.pretty_print();
        println!();

        if let Some(winner) = game.check_if_winner() {
            println!("{} in {} moves!", if winner.is_none() {
                "draw".to_string()
            } else {
                format!("{} wins",winner.unwrap())
            }, game.total_moves());
            break;
        }

        if play_random {

            loop {
                use rand::Rng;
                let mut rng = rand::thread_rng();
                let md = game.dim();
                let from = Coord::new(rng.gen_range(0..md), rng.gen_range(0..md));
                let to = Coord::new(rng.gen_range(0..md), rng.gen_range(0..md));
                if let Ok(action) = game.validate_action(from, to) {
                    println!("# {} {}", game.player(), action);
                    println!("# action outcome is {}",game.perform_action(action).expect("action should have been pre-validated"));
                } else {
                    // println!("Invalid move!");
                    continue;
                }
                break;
            }

        } else {

            if let Some((from,to)) = game.parse_move_stdin() {
                if let Ok(action) = game.validate_action(from, to) {
                    println!("# {} {}", game.player(), action);
                    println!("# action outcome is {}",game.perform_action(action).expect("action should have been pre-validated"));
                } else {
                    println!("Invalid move!");
                }
            } else {
                println!("Invalid input!");
            }

        }
    }
}