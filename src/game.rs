use crate::{Coord, UnitType, Cell, Dim, Player, Unit, Board, DisplayFirstLetter};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Game {
    player: Player,
    board: Board,
    dim: Dim,
    total_moves: usize,
    drop_prob: Option<f32>,
}

impl Game {
    pub fn new(dim: Dim, drop_prob: Option<f32>) -> Self {
        let mut game = Self {
            player: Player::default(),
            board: Board::new(dim),
            dim,
            total_moves : 0,
            drop_prob,
        };
        let md = dim-1;
        let ai = Unit::new(UnitType::AI);
        let hacker = Unit::new(UnitType::Hacker);
        let repair = Unit::new(UnitType::Repair);
        let tank = Unit::new(UnitType::Tank);
        let soldier = Unit::new(UnitType::Soldier);
        let drone = Unit::new(UnitType::Drone);
        let init = vec![
            (0,2,&ai), (0,md-2,&ai),
            (1,2,&tank), (1,md-2,&tank),
            (0,1,&repair), (0,md-1,&repair),
            (0,3,&hacker), (0,md-3,&hacker),
            (1,1,&soldier), (1,md-1,&soldier),
            (1,3,&drone), (1,md-3,&drone),
        ];
        for (row,col,unit) in init {
            game[(row,col)] = Cell::Unit{player: Player::Blue,unit: unit.clone()};
            game[(md-row,col)] = Cell::Unit{player: Player::Red,unit: unit.clone()};
        }
        game.drop_prob = drop_prob;
        game
    }
    pub fn dim(&self) -> Dim {
        self.dim
    }
    pub fn get_cell(&self, coord: (i8, i8)) -> Option<&Cell> {
        if self.is_valid_position(coord) {
            Some(&self.board.get(coord).unwrap())
        } else {
            None
        }
    }
    pub fn get_cell_mut(&mut self, coord: (i8, i8)) -> Option<&mut Cell> {
        if self.is_valid_position(coord) {
            self.board.get_mut(coord)
        } else {
            None
        }
    }
    pub fn set_cell(&mut self, coord: (i8, i8), value: Cell) {
        if self.is_valid_position(coord) {
            self.board[coord] = value;
        }
    }
    pub fn get_two_cells_mut(&mut self, coord0: Coord, coord1: Coord) -> Option<[&mut Cell;2]> {
        if self.is_valid_position(coord0) &&
            self.is_valid_position(coord1) &&
            coord0 != coord1
        {
            self.board.get_two_mut(coord0, coord1)
        } else {
            None
        }
    }
    pub fn player(&self) -> Player {
        self.player
    }
    pub fn total_moves(&self) -> usize {
        self.total_moves
    }
    pub fn next_player(&mut self) -> Player {
        self.player = self.player.next();
        self.total_moves += 1;
        self.player
    }
    pub fn is_valid_position(&self, (row,col) : (i8, i8)) -> bool {
        row >= 0 && col >= 0 && row < self.dim && col < self.dim
    }
    pub fn is_valid_move(&mut self, from: Coord, to: Coord) -> bool {
        self.neighbors(from, to) &&
        self[to].is_empty() && self[from].is_unit() &&
        self.player() == self[from].player().unwrap()
    }
    pub fn neighbors(&self, coord0 : Coord, coord1 : Coord) -> bool {
        coord0 != coord1 &&
        self.is_valid_position(coord0) && self.is_valid_position(coord1) && 
        (coord1.0 - coord0.0).abs() <= 1 && (coord1.1 - coord0.1).abs() <= 1
    }
    pub fn in_range(&self, range: u8, coord0 : Coord, coord1 : Coord) -> bool {
        coord0 == coord1 || // we consider our own position as in range
        self.is_valid_position(coord0) && 
        self.is_valid_position(coord1) && 
        (coord1.0 - coord0.0).abs() as u8 <= range && 
        (coord1.1 - coord0.1).abs() as u8 <= range
    }
    pub fn move_unit(&mut self, from: Coord, to: Coord) -> bool {
        if self.is_valid_move(from, to) {
            self[to] = self[from];
            self[from] = Cell::default();
            true
        } else {
            false
        }
    }
    pub fn remove_dead(&mut self) {
        for cell in self.board.iter_mut() {
            cell.remove_dead();
        }
    }
    // pub fn resolve_conflicts(&mut self) {
    //     for row in 0..BOARD_DIM {
    //         for col in 0..BOARD_DIM {
    //             let coord_source = (row,col);
    //             if self[coord_source].is_unit() {
    //                 for rd in -1..=1 {
    //                     for cd in -1..=1 {
    //                         let coord_target = (row + rd, col + cd);
    //                         if self.is_valid_position(coord_target) && self[coord_target].is_unit() && coord_target != coord_source
    //                         {
    //                             let (source, target) = self.get_2_cells_mut(coord_source, coord_target).unwrap();
    //                             let (player_source,unit_source) = source.unit_mut().unwrap();
    //                             let (player_target,unit_target) = target.unit_mut().unwrap();
    //                             if player_source != player_target {
    //                                 // opponents
    //                                 unit_source.apply_damage(unit_target);
    //                             } else {
    //                                 // friends
    //                                 unit_source.apply_repair(unit_target);
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
    pub fn check_if_winner(&self) -> Option<Option<Player>>{
        assert_eq!(Player::cardinality(),2);
        let mut p_all = Player::all();
        let p1 = p_all.next().unwrap();
        let p2 = p_all.next().unwrap();
        let mut ai_p1 = false;
        let mut ai_p2 = false;
        for c in self.board.iter() {
            if let Some((player,unit)) = c.unit() {
                if player == &p1 && unit.unit_type == UnitType::AI {
                    ai_p1 = true;
                }
                if player == &p2 && unit.unit_type == UnitType::AI {
                    ai_p2 = true;
                }
            }
            if ai_p1 && ai_p2 { break }
        }
        if ai_p1 && !ai_p2 {
            Some(Some(p1))
        } else if ai_p2 && !ai_p1 {
            Some(Some(p2))
        } else if !ai_p2 && !ai_p1 {
            Some(None)
        } else {
            None
        }
    }
    pub fn parse_move_stdin(&self) -> Option<(Coord,Coord)> {
        use std::io::Write;
        print!("{} player, enter your next move [ex: a6 d9] : ",self.player());
        std::io::stdout().flush().unwrap();
        Self::parse_move(&std::io::stdin().lines().next().unwrap().unwrap())
    }
    pub fn parse_move(move_str: &str) -> Option<(Coord,Coord)> {
        use regex::Regex;
        let re = Regex::new(r#"[ \(\[]*([A-Za-z])[ ,;]*(\d+)[ \)\]]*[;,]*[ \(\[]*([A-Za-z])[ ,;]*(\d+)[ \)\]]*"#).unwrap();
        if let Some(caps) = re.captures(move_str) {
            assert_eq!(caps.len(),5);
            let r1 = caps[1].chars().next().unwrap().to_ascii_uppercase() as Dim - 65;
            let c1 = caps[2].parse::<Dim>().unwrap();
            let r2 = caps[3].chars().next().unwrap().to_ascii_uppercase() as Dim - 65;
            let c2 = caps[4].parse::<Dim>().unwrap();
            Some(((r1,c1),(r2,c2)))
        } else {
            None
        }
    }
    pub fn random_drop(&mut self) -> bool {
        if self.drop_prob.is_none() {
            return false;
        }
        use rand::Rng;
        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() < self.drop_prob.unwrap() {
            use UnitType::*;
            let unit_types = [Hacker,Repair];
            let unit_type = unit_types[rng.gen_range(0..unit_types.len())];
            let dest = (rng.gen_range(0..self.dim()),rng.gen_range(0..self.dim()));
            let mut new = Cell::new_unit(self.player(), unit_type);
            println!("random drop of type {} at {}!",unit_type,Self::coord_to_string(dest));
            let target = &mut self[dest];
            if target.is_unit() {
                new.interact(target);
                println!("random interaction at {}!",Self::coord_to_string(dest));
            }
            if target.is_empty() {
                *target = new;
                println!("random insertion at {}!",Self::coord_to_string(dest));
            } 
            return true;
        }
        false
    }
    pub fn perform_action(&mut self, from: Coord, to: Coord) -> bool {
        let valid = if self.in_range(1, from, to) && 
            self[from].is_unit() && 
            self.player() == self[from].player().unwrap() 
        {
            // it's our turn and we are acting on our own unit
            if from == to {
                // destination is same as source => we wish to skip this move
                true
            } else if self[to].is_empty() {
                // destination empty so this is a move
                self.move_unit(from, to)
            } else if self[to].is_unit() {
                // destination is a unit
                let [source, target] = self.get_two_cells_mut(from, to).unwrap();
                let (player_source,unit_source) = source.unit_mut().unwrap();
                let (player_target,unit_target) = target.unit_mut().unwrap();
                if player_source != player_target {
                    // it's an opposing unit so we try to damage it (it will damage us back)
                    unit_source.apply_damage(unit_target);
                    unit_target.apply_damage(unit_source);
                    source.remove_dead();
                    target.remove_dead();
                } else {
                    // it's our unit so we try to repair it
                    unit_source.apply_repair(unit_target);
                }
                true
            } else {
                false
            }
        } else {
            false
        };
        if valid {
            self.random_drop();
            self.next_player();
        };
        valid
    }
    pub fn pretty_print(&self) {
        println!("Next player: {}",self.player());
        print!("    ");
        for col in 0..self.dim {
            print!(" {:>2} ",col);
        }
        println!();
        for row in 0..self.dim {
            print!("{:>2}: ",(row as u8 +'A' as u8) as char);
            for col in 0..self.dim {
                let cell = self[(row,col)];
                print!(" {}",cell.to_pretty_compact_string());
            }
            println!();
        }
    }
    pub fn coord_to_string((row, col) : Coord) -> String {
        format!("{}{}",(row as u8 +'A' as u8) as char, col)
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new(10, None)
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.player().to_first_letter())?;
        for c in self.board.iter() {
            write!(f,":{}",c)?;
        }
        Ok(())
    }
}

impl std::ops::Index<Coord> for Game {
    type Output = Cell;
    fn index(&self, coord: Coord) -> & Self::Output {
        self.get_cell(coord).expect("expected valid coordinates")
    }
}

impl std::ops::IndexMut<Coord> for Game {
    fn index_mut(&mut self, coord: Coord) -> &mut Self::Output {
        self.get_cell_mut(coord).expect("expected valid coordinates")
    }
}

