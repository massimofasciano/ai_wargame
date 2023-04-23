use ai_wargame::{Game, DEFAULT_BOARD_DIM, units_health_heuristic, units_score_distance_center};

fn main() {
    let dim = DEFAULT_BOARD_DIM;
    let drop_prob = None;
    // let drop_prob = Some(0.005);
    let max_depth = 4;
    let attacker_heuristic = units_score_distance_center;
    let defender_heuristic = units_health_heuristic;
    let mut game = Game::new(dim,attacker_heuristic,defender_heuristic,drop_prob,max_depth);
    let play_alone = std::env::args().len() > 1;

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

        // for coord in game.player_coords(game.player()) {
        //     println!("% Possible actions for {} :",coord);
        //     for action in game.possible_actions_from_coord(coord) {
        //         println!("% - {}",action);
        //     }
        // }

        if play_alone {

            // loop {
            //     use rand::Rng;
            //     let mut rng = rand::thread_rng();
            //     let md = game.dim();
            //     let from = Coord::new(rng.gen_range(0..md), rng.gen_range(0..md));
            //     let to = Coord::new(rng.gen_range(0..md), rng.gen_range(0..md));
            //     if game.console_play_turn(from, to) {
            //         break;
            //     }
            // }

            game.computer_play_turn();

        } else {

            let suggestion = game.suggest_action();
            println!("Suggestion: {}",suggestion);
            if let Some((from,to)) = game.parse_move_stdin() {
                game.console_play_turn(from, to);
            } else {
                println!("Invalid input!");
            }

        }
    }
}