use ai_wargame::{Game, heuristics};

fn main() {
    #[cfg(feature="rayon")]
    rayon::ThreadPoolBuilder::new().num_threads(4).build_global().unwrap();
    // let dim = ai_wargame::DEFAULT_BOARD_DIM;
    let dim = 7;
    let drop_prob = None;
    // let drop_prob = Some(0.005);
    let max_depth = Some(8);
    let max_moves = Some(150);
    let max_seconds = Some(5.0);
    let attacker_heuristic = heuristics::ai_distance_units_health_heuristic;
    // let attacker_heuristic = heuristics::units_health_opponent_heuristic;
    // let attacker_heuristic = heuristics::units_health_heuristic;
    // let defender_heuristic = heuristics::ai_distance_units_health_heuristic;
    let defender_heuristic = heuristics::units_health_heuristic;
    // let attacker_heuristic = heuristics::units_heuristic;
    // let defender_heuristic = heuristics::units_heuristic;
    let mut game = Game::new(dim,attacker_heuristic,defender_heuristic,drop_prob,max_depth,max_moves,max_seconds);
    let play_alone = std::env::args().len() > 1;

    loop {
        println!();
        game.pretty_print();
        println!();

        if let Some(winner) = game.end_game_result() {
            println!("{} wins in {} moves!", winner, game.total_moves());
            // println!("{:#?}",game);
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

            let (score, suggestion,elapsed_seconds,avg_depth) = game.suggest_action();
            println!("Suggestion: {}",suggestion);
            println!("Compute time: {:.1} sec",elapsed_seconds);
            println!("Average depth: {:.1}", avg_depth);
            println!("# Score: {}", score);
            if let Some((from,to)) = game.parse_move_stdin() {
                game.console_play_turn(from, to);
            } else {
                println!("Invalid input!");
            }

        }
    }
}