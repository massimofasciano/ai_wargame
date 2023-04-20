use ai_wargame::{Game, DEFAULT_BOARD_DIM, Coord};

fn main() {
    let dim = DEFAULT_BOARD_DIM;
    // let drop_prob = None;
    let drop_prob = Some(0.005);
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
                if game.console_play_turn(from, to) {
                    break;
                }
            }

        } else {

            if let Some((from,to)) = game.parse_move_stdin() {
                game.console_play_turn(from, to);
            } else {
                println!("Invalid input!");
            }

        }
    }
}