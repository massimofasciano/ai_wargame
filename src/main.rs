use ai_wargame::{Game, GameOptions};

fn main() {
    // #[cfg(feature="rayon")]
    // rayon::ThreadPoolBuilder::new().num_threads(4).build_global().unwrap();

    let cmd_opt = std::env::args().nth(1);

    let mut options = GameOptions::default();
    // options.adjust_max_depth = false;

    match cmd_opt.as_deref() {
        Some("auto") => {
            options.debug = true;
        },
        Some("defender") | Some("defend") | Some("attacker") | Some("attack") | None => {},
        Some(_) => {
            let my_name = option_env!("CARGO_PKG_NAME").unwrap_or("AI_Wargame");
            eprintln!("usage: {} auto|attack(er)|defend(er)",my_name);
            std::process::exit(1);
        }
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

        match cmd_opt.as_deref() {
            Some("auto") => {
                game.console_computer_play_turn();
            },
            Some("defender") | Some("defend") => {
                game.console_computer_play_turn();
                println!();
                game.console_pretty_print();
                println!();
                game.console_human_play_turn();
            },
            Some("attacker") | Some("attack") | None => {
                game.console_human_play_turn();
                println!();
                game.console_pretty_print();
                println!();
                game.console_computer_play_turn();
            }
            Some(_) => {}
        }
    }
}