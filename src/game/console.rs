use crate::{Game, Coord, UnitType};

use std::io::Write as IoWrite;
use std::io::{stdout,stdin};

impl Game {
    pub fn console_pretty_print(&self) {
        self.pretty_print(&mut stdout()).expect("no errors on stdout");
        stdout().flush().expect("no errors on stdout");
    }
    pub fn console_read_move(&self) -> Result<(Coord,Coord),String> {
        print!("{} player, enter your next move: ",self.player());
        stdout().flush().expect("no errors on stdout");
        let input = stdin().lines().next().unwrap().expect("no errors on stdin");
        let parsed = Self::parse_move(&input);
        parsed.ok_or(input)
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
        self.computer_play_turn(Some(&mut stdout())).expect("no errors on stdout");
        stdout().flush().expect("no errors on stdout");
    }
}
