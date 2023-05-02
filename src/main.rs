use ai_wargame::{Game, heuristics::{self}, GameOptions};

fn main() {
    // #[cfg(feature="rayon")]
    // rayon::ThreadPoolBuilder::new().num_threads(4).build_global().unwrap();

    let cmd_opt = std::env::args().nth(1);

    let mut options = GameOptions::default();
    // options.dim = 7;
    options.max_depth = Some(6);
    options.max_moves = Some(150);
    options.max_seconds = Some(5.0);
    {
        use heuristics::*;
        let _h1 = units_health_weights_bias(10,10,100) * 10
                                + ai_distance(2,1)
                                - game_moves();
        // let _h2 = -game_moves();
        // options.heuristics.attacker_max = _h1.clone();
        // options.heuristics.defender_min = _h1;
        options.heuristics.set_attack_heuristics(_h1);
        // options.heuristics.set_defense_heuristics(_h2);
    }
    // options.move_while_engaged = true;
    // options.move_while_engaged_full_health = true;
    options.mutual_damage = true;
    options.move_only_forward = true;
    // options.debug = true;
    options.adjust_max_depth = true;
    if cmd_opt == Some(String::from("auto")) {
        options.debug = true;
    }
        
    let mut game = Game::new(options);

    loop {
        println!();
        game.console_pretty_print();
        println!();

        if let Some(winner) = game.end_game_result() {
            println!("{} wins in {} moves!", winner, game.total_moves());
            break;
        }

        if cmd_opt == Some(String::from("auto")) {
            // computer plays both sides...
            game.console_computer_play_turn();
        } else {
            // human plays...
            game.console_human_play_turn();
            // show intermediate board state...
            println!();
            game.console_pretty_print();
            println!();
            // computer plays...
            game.console_computer_play_turn();
        }
    }
}