use std::process::exit;

use ai_wargame::{Game, GameOptions};

fn print_usage(program: &str, opts: getopts::Options) {
    let my_name = option_env!("CARGO_PKG_NAME").unwrap_or(program);
    let brief = format!("Usage: {} [options]", my_name);
    print!("{}", opts.usage(&brief));
}

#[derive(Debug,Default)]
enum PlayType {
    Auto,
    Defend,
    #[default]
    Attack,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();

    let mut opts = getopts::Options::new();
    opts.optopt("d", "depth", "maximum search depth", "NUM");
    opts.optopt("s", "seconds", "maximum search time in seconds", "NUM");
    opts.optopt("m", "moves", "maximum moves in a game", "NUM");
    opts.optopt("p", "play", "type of gameplay", "auto|defender|attacker");
    #[cfg(feature = "rayon")]
    opts.optopt("t", "threads", "mumber of computing threads to use (defaults to total cores)", "NUM");

    opts.optflag("h", "help", "print this help menu");
    opts.optflag("a", "auto-depth", "try to auto adjust the search depth dynamically");
    #[cfg(feature="rayon")]
    opts.optflag("r", "rayon", "enable rayon multithreading");
    opts.optflag("n", "no-debug", "disable debug information");
    opts.optflag("b", "benchmark", "determine starting max-depth via benchmark");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => {
            print_usage(&program, opts);
            println!("\n{}",f.to_string());
            exit(1)
        }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        exit(0);
    }

    let mut options = GameOptions::default();

    let play_type = match matches.opt_str("play").as_deref() {
        None => {
            PlayType::default()
        },
        Some("auto") => {
            PlayType::Auto
        },
        Some("attacker") | Some("attack") => {
            PlayType::Attack
        },
        Some("defender") | Some("defend") => {
            PlayType::Defend
        },
        Some(_) => {
            print_usage(&program, opts);
            exit(1)
        }
    };

    options.debug = !matches.opt_present("no-debug");
    options.adjust_max_depth = matches.opt_present("auto-depth");
    if matches.opt_present("depth") {
        options.max_depth = matches.opt_str("depth").and_then(|s|s.parse::<usize>().ok());
    }
    if matches.opt_present("seconds") {
        options.max_seconds = matches.opt_str("seconds").and_then(|s|s.parse::<f32>().ok());
    }
    if matches.opt_present("moves") {
        options.max_moves = matches.opt_str("moves").and_then(|s|s.parse::<usize>().ok());
    }

    options.multi_threaded = false;
    #[cfg(feature="rayon")]
    if matches.opt_present("rayon") {
        options.multi_threaded = true;
        if let Some(threads) = matches.opt_str("threads").and_then(|s|s.parse::<usize>().ok()) {
            rayon::ThreadPoolBuilder::new().num_threads(threads).build_global().unwrap();
        }
    }

    let mut game = Game::new(options);

    if matches.opt_present("benchmark") {
        if let Some(max_seconds) = game.options().max_seconds {
            println!("Running benchmark (expect delay of up to {:.0} seconds)...", max_seconds*1.4);
            if let Some(max_depth) = game.run_benchmark(None) {
                println!("Benchmark adjusted max depth to {max_depth}");
            }
        }
    }

    loop {
        println!();
        game.console_pretty_print();
        println!();

        if let Some(winner) = game.end_game_result() {
            println!("{} wins in {} moves!", winner, game.total_moves());
            break;
        }

        match play_type {
            PlayType::Auto => {
                game.console_computer_play_turn();
            },
            PlayType::Defend => {
                game.console_computer_play_turn();
                println!();
                game.console_pretty_print();
                println!();
                game.console_human_play_turn();
            },
            PlayType::Attack => {
                game.console_human_play_turn();
                println!();
                game.console_pretty_print();
                println!();
                game.console_computer_play_turn();
            }
        }
    }
}