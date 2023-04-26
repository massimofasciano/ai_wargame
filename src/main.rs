use ai_wargame::{Game, heuristics, GameOptions};

fn main() {
    #[cfg(feature="rayon")]
    rayon::ThreadPoolBuilder::new().num_threads(4).build_global().unwrap();
    let cmd_opt = std::env::args().nth(1);

    let mut options = GameOptions::default();
    options.dim = 4;
    options.max_depth = Some(6);
    options.max_moves = Some(100);
    options.max_seconds = Some(5.0);
    options.heuristics.attacker = heuristics::ai_distance_units_health_heuristic;
    options.heuristics.defender = heuristics::units_health_heuristic;
    // options.mutual_damage = true;
    // options.debug = true;
    if cmd_opt == Some(String::from("auto")) {
        options.debug = true;
    }
        
    let mut game = Game::new(options);

    loop {
        println!();
        game.pretty_print();
        println!();

        if let Some(winner) = game.end_game_result() {
            println!("{} wins in {} moves!", winner, game.total_moves());
            break;
        }

        if cmd_opt == Some(String::from("auto")) {
            // computer plays both sides...
            game.computer_play_turn();
        } else {
            // make a quick suggestion...
            game.console_quick_suggestion();
            // human plays...
            game.console_play_turn_stdin();
            // show intermediate board state...
            // println!();
            // game.pretty_print();
            // println!();
            // computer plays...
            game.computer_play_turn();
        }
    }
}