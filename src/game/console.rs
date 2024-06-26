use crate::{Game, Coord, UnitType};

use std::io::Write as IoWrite;
use std::io::{stdout,stdin};

impl Game {
    pub fn console_pretty_print(&self) {
        self.pretty_print(&mut stdout()).expect("no errors on stdout");
        stdout().flush().expect("no errors on stdout");
    }
    pub fn console_read_move(&self) -> Result<(Coord,Coord),String> {
        if self.options().broker.is_none() {
            print!("{} player, enter your next move: ",self.player());
            stdout().flush().expect("no errors on stdout");
            let input = stdin().lines().next().unwrap().expect("no errors on stdin");
            let parsed = Self::parse_move(&input);
            parsed.ok_or(input)
        } else {
            #[cfg(feature="broker")]
            match self.broker_get_move() {
                Ok(Some(coords)) => {
                    Ok((coords.from,coords.to))
                },
                Ok(None) => {
                    Err("broker retry".to_string())
                },
                Err(error) => {
                    eprintln!("{}",error);
                    std::process::exit(1);
                },
            }
            #[cfg(not(feature="broker"))]
            {
                eprintln!("broker feature not enabled");
                std::process::exit(1);
            }
        }
    }
    pub fn console_human_play_turn_from_coords(&mut self, from: impl Into<Coord>, to: impl Into<Coord>) -> bool {
        let result = self.human_play_turn_from_coords(Some(&mut stdout()), from, to).expect("no errors on stdout");
        stdout().flush().expect("no errors on stdout");
        result
    }
    pub fn console_human_play_turn(&mut self) {
        let mut options = self.clone_options();
        options.max_depth = Some(4);
        options.max_seconds = Some(0.5);
        let mut game_suggest = self.clone();
        game_suggest.set_options(options);
        if let (_, Some(suggestion),_,_) = game_suggest.suggest_action() {
            println!("Suggestion: {}",suggestion);
            if self.options().broker.is_some() {
                println!("Getting next move with auto-retry from game broker...");
            }
            loop {
                match self.console_read_move() {
                    Ok((from,to)) => {
                        if self.console_human_play_turn_from_coords(from, to) {
                            break;
                        } else {
                            println!("Invalid move!");
                            println!();
                        }
                    },
                    Err(s) if s == "quit" || s == "exit" => {
                        std::process::exit(0);
                    },
                    Err(s) if s == "broker retry" => {
                        // println!("Trying broker again in 100ms");
                        std::thread::sleep(instant::Duration::from_millis(100));
                    },
                    _ => {
                        println!();
                        println!("Enter source coordinates followed by target for action (move, attack, repair).");
                        println!("If source=target it means self-destruct."); 
                        println!("example input: a6 d9"); 
                        println!();
                        println!("Damage table:");
                        let legend = Some("from / to");
                        let width = 10;
                        let tfmt = ToString::to_string;
                        Self::console_table(width, 
                            UnitType::damage_table(legend,tfmt,tfmt));
                        println!();
                        println!("Repair table:");
                        Self::console_table(width, 
                            UnitType::repair_table(legend,tfmt,tfmt));
                        println!();
                        println!("Self destruct damage: {} per adjacent unit (including diagonals and firendlies)",UnitType::self_destruct_string());
                        println!();
                    }
                }
            }
        } else {
            self.state.deadlock = true;
        }
    }
    pub fn console_table(width: usize, table: Vec<Vec<String>>) {
        for row in table {
            for cell in row {
                print!("{:>width$}",cell);
            }
            println!();
        }
    }
    pub fn console_computer_play_turn(&mut self) {
        let opt_action = self.computer_play_turn(Some(&mut stdout())).expect("no errors on stdout");
        stdout().flush().expect("no errors on stdout");
        #[cfg(not(feature="broker"))]
        let _ = opt_action;
        #[cfg(feature="broker")]
        if self.options().broker.is_some() && opt_action.is_some() {
            if let Some(coord_pair) = opt_action.unwrap().into_coord_pair() {
                if let Err(error) = self.broker_post_move(coord_pair) {
                    eprintln!("Could not post move to broker: {error}");
                    std::process::exit(1);
                }
            }
        }
    }
}
