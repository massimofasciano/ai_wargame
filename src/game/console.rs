use crate::{Game, Coord, UnitType, IsUsefulInfo};

use std::io::Write as IoWrite;
use std::io::Result as IoResult;
use std::io::{stdout,stdin};

impl Game {
    pub fn pretty_print_info(&self, w: &mut impl IoWrite) -> IoResult<()> {
        if let Some(max_moves) = self.options.max_moves {
            if self.total_moves() >= max_moves {
                writeln!(w,"# maximum moves played ({})",max_moves)?;
            } else {
                writeln!(w,"# {}/{} moves played",self.total_moves(),max_moves)?;
            }
        } else {
            writeln!(w,"# {} moves played",self.total_moves())?;
        }
        if self.options.debug {
            if let Some(max_depth) = self.options.max_depth {
                writeln!(w,"# Max search depth: {}",max_depth)?;
            }
            if let Some(max_seconds) = self.options.max_seconds {
                writeln!(w,"# Max search time: {:.1} sec",max_seconds)?;
            }
            #[cfg(feature="stats")]
            {
                let stats = self.stats.lock().expect("should get a lock");
                writeln!(w,"# Total evals at each depth: {:?}",stats.depth_counts)?;
                let (dc, ct) = stats.depth_counts.iter().fold((0,0),|(dc,ct),(d,c)| (dc+d*c,ct+c));
                if ct > 0 {
                    writeln!(w,"# Average eval depth: {:.1}",dc as f32/ct as f32)?;
                }
                if self.total_moves() > 0 {
                    writeln!(w,"# Average eval time: {:.1}",stats.total_seconds as f32/self.total_moves() as f32)?; 
                }
                if stats.total_effective_branches > 0 {
                    writeln!(w,"# Average branching factor: {:.1}",stats.total_moves_per_effective_branch as f32/stats.total_effective_branches as f32)?; 
                }
            }            
            writeln!(w,"# Next player: {}",self.player())?;
        }
        Ok(())
    }
    pub fn pretty_print_board(&self, w: &mut impl IoWrite) -> IoResult<()> {
        write!(w,"    ")?;
        for col in 0..self.dim() {
            write!(w," {:>2} ",col)?;
        }
        writeln!(w)?;
        for row in 0..self.dim() {
            write!(w,"{:>2}: ",(row as u8 +'A' as u8) as char)?;
            for col in 0..self.dim() {
                let cell = self[Coord::new(row,col)];
                write!(w," {}",cell.to_pretty_compact_string())?;
            }
            writeln!(w)?;
        }
        Ok(())
    }
    pub fn pretty_print(&self, w: &mut impl IoWrite) -> IoResult<()> {
        self.pretty_print_info(w)?;
        writeln!(w)?;
        self.pretty_print_board(w)
    }
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
        if let Ok((player, action, outcome)) = self.play_turn_from_coords(from, to) {
            println!("-> {}: {}", player, action);
            if self.options.debug {
                if outcome.is_useful_info() {
                    println!("# {}", outcome);
                }
            }
            true
        } else {
            false
        }
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
                        println!("example input: a6 d9"); 
                        println!();
                        println!("{}",UnitType::units_description());
                    }
                }
            }
        } else {
            self.state.deadlock = true;
        }
    }
    pub fn console_computer_play_turn(&mut self) {
        let (score,best_action,elapsed_seconds,avg_depth) = self.suggest_action();
        #[cfg(feature="stats")]
        {
            self.stats.lock().expect("should get the lock").total_seconds += elapsed_seconds;
        }
        if self.options.adjust_max_depth {
            self.adjust_max_depth(elapsed_seconds, avg_depth);
        }
        if let Some(best_action) = best_action {
            if let Ok((player, action, outcome)) = self.play_turn_from_action(best_action) {
                println!("-> {}: {}", player, action);
                if self.options.debug {
                    if outcome.is_useful_info() {
                        println!("# {}", outcome);
                    }
                    println!("# Compute time: {:.1} sec", elapsed_seconds);
                    println!("# Average depth: {:.1}", avg_depth);
                    println!("# Heuristic score: {}", score);
                }
            } else {
                panic!("play turn should work");
            }
        } else {
            self.state.deadlock = true;
        }
    }
}
