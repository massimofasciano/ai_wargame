use crate::{Coord, UnitType, BoardCell, Dim, Player, Unit, Board, DisplayFirstLetter, Action, ActionOutcome, CoordPair, DropOutcome, IsUsefulInfo, BoardCellRefMut};

use rand::seq::{IteratorRandom, SliceRandom};

#[derive(Debug, PartialEq, Clone)]
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
        assert!(dim >= 4,"initial setup requires minimum of 4x4 board");
        let init = vec![
            (0,1,&repair), (0,md-1,&hacker),
            (1,1,&soldier), (1,md-1,&drone),
            (0,3,&hacker), (0,md-3,&repair),
            (1,3,&drone), (1,md-3,&soldier),
            (0,2,&ai), (0,md-2,&ai),
            (1,2,&tank), (1,md-2,&tank),
        ];
        assert_eq!(Player::cardinality(),2);
        let mut p_all = Player::all();
        let p1 = p_all.next().unwrap();
        let p2 = p_all.next().unwrap();
        for (row,col,unit) in init {
            game.set_cell((row,col),BoardCell::Unit{player: p2, unit: unit.clone()});
            game.set_cell((md-row,col),BoardCell::Unit{player: p1, unit: unit.clone()});
        }
        game.drop_prob = drop_prob;
        game
    }
    pub fn dim(&self) -> Dim {
        self.dim
    }
    pub fn remove_cell(&mut self, coord: Coord) -> Option<BoardCell> {
        if self.is_valid_position(coord) {
            self.board.remove(coord)
        } else {
            None
        }
    }
    pub fn get_cell(&self, coord: Coord) -> Option<&BoardCell> {
        if self.is_valid_position(coord) {
            Some(&self.board.get(coord).unwrap())
        } else {
            None
        }
    }
    // pub fn get_cell_mut(&mut self, coord: Coord) -> Option<&mut BoardCell> {
    //     if self.is_valid_position(coord) {
    //         self.board.get_mut(coord)
    //     } else {
    //         None
    //     }
    // }
    pub fn get_cell_mut(&mut self, coord: Coord) -> Option<BoardCellRefMut> {
        if self.is_valid_position(coord) {
            self.board.get_mut(coord)
        } else {
            None
        }
    }
    pub fn set_cell(&mut self, coord: impl Into<Coord>, value: BoardCell) {
        let coord = coord.into();
        if self.is_valid_position(coord) {
            self.board.set(coord,value);
        }
    }
    pub fn get_two_cells_mut(&mut self, coord0: Coord, coord1: Coord) -> Option<[&mut BoardCell;2]> {
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
    pub fn is_valid_position(&self, coord : Coord) -> bool {
        let (row,col) = coord.to_tuple();
        let is_valid = row >= 0 && col >= 0 && row < self.dim && col < self.dim;
        debug_assert_eq!(is_valid,true,"({},{}) is not valid for a {}x{} board",row,col,self.dim,self.dim);
        is_valid
    }
    pub fn is_valid_move(&mut self, from: Coord, to: Coord) -> bool {
        self.neighbors(from, to) &&
        self[to].is_empty() && self[from].is_unit() &&
        self.player() == self[from].player().unwrap()
    }
    pub fn neighbors(&self, coord0 : Coord, coord1 : Coord) -> bool {
        coord0 != coord1 &&
        self.is_valid_position(coord0) && self.is_valid_position(coord1) && 
        (coord1.row - coord0.row).abs() <= 1 && (coord1.col - coord0.col).abs() <= 1
    }
    pub fn in_range(&self, range: u8, coord0 : Coord, coord1 : Coord) -> bool {
        coord0 == coord1 || // we consider our own position as in range
        self.is_valid_position(coord0) && 
        self.is_valid_position(coord1) && 
        (coord1.row - coord0.row).abs() as u8 <= range && 
        (coord1.col - coord0.col).abs() as u8 <= range
    }
    pub fn unit_move(&mut self, from: Coord, to: Coord) -> Result<ActionOutcome,()> {
        if self.is_valid_move(from, to) {
            let removed = self.remove_cell(from).unwrap();
            self.set_cell(to,removed);
            Ok(ActionOutcome::Moved { delta: to-from })
        } else {
            Err(())
        }
    }
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
            Some((Coord::new(r1,c1),Coord::new(r2,c2)))
        } else {
            None
        }
    }
    pub fn board_rect(&self) -> CoordPair {
        CoordPair::new(Coord::new(0,0),Coord::new(self.dim(), self.dim()))
    }
    pub fn empty_coords<'a>(&'a self) -> impl Iterator<Item = Coord> + 'a {
        self.board_rect().rect_iter().filter(|&c|self[c].is_empty())
    }
    pub fn player_coords<'a>(&'a self, player: Player) -> impl Iterator<Item = Coord> + 'a {
        self.board_rect().rect_iter().filter(move|&c|!self[c].is_empty() && self[c].player().unwrap() == player)
    }
    pub fn random_drop(&mut self) -> DropOutcome {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        if self.drop_prob.is_some() && rng.gen::<f32>() < self.drop_prob.unwrap() {
            let unit_type = *[UnitType::Hacker,UnitType::Repair].choose(&mut rng).expect("expect a hacker or repair");
            if let Some(empty_coord) = self.empty_coords().choose(&mut rng) {
                println!("random drop of type {} at {}!",unit_type,empty_coord);
                self.set_cell(empty_coord, BoardCell::new_unit(self.player(), unit_type));
                DropOutcome::Drop {location:empty_coord, unit_type: unit_type}
            } else {
                DropOutcome::NoDrop
            }
        } else {
            DropOutcome::NoDrop
        }
    }
    pub fn remove_dead(&mut self, coord: Coord) {
        if let Some(cell) = self.get_cell(coord) {
            if let Some((_, unit)) = cell.unit() {
                if unit.health == 0 {
                    self.remove_cell(coord);
                    // *cell = BoardCell::default();
                }
            }
        }
    }
    pub fn perform_action(&mut self, action: Action) -> Result<ActionOutcome,()> {
        match action {
            Action::Pass => Ok(ActionOutcome::Passed),
            Action::Move { from, to } => {
                self.unit_move(from, to)
            }
            Action::Repair { from, to } => {
                self.unit_repair(from, to)
            }
            Action::Attack { from, to } => {
                self.unit_combat(from, to)
            }
        }
    }
    pub fn play_turn_from_action(&mut self, action: Action) -> Result<(Action,ActionOutcome,DropOutcome),()> {
        let outcome = self.perform_action(action);
        if let Ok(outcome) = outcome {
            let drop_outcome = self.random_drop();
            self.next_player();
            Ok((action,outcome,drop_outcome))
        } else {
            Err(())
        }
    }
    pub fn play_turn_from_coords(&mut self, from: impl Into<Coord>, to: impl Into<Coord>) -> Result<(Action,ActionOutcome,DropOutcome),()> {
        if let Ok(action) = self.action_from_coords(from, to) {
            self.play_turn_from_action(action)
        } else {
            Err(())
        }
    }
    pub fn console_play_turn(&mut self, from: impl Into<Coord>, to: impl Into<Coord>) -> bool {
        if let Ok((action, outcome,drop_outcome)) = self.play_turn_from_coords(from, to) {
            println!("# {} {}", self.player(), action);
            if outcome.is_useful_info() {
                println!("# {}", outcome);
            }
            if drop_outcome.is_useful_info() {
                println!("# {}", drop_outcome);
            }
            true
        } else {
            false
        }
    }
    pub fn unit_combat(&mut self, from: Coord, to: Coord) -> Result<ActionOutcome,()> {
        if self.in_range(1, from, to) && 
            self[from].is_unit() && 
            self[to].is_unit() 
        {
            let [source, target] = self.get_two_cells_mut(from, to).unwrap();
            let (player_source,unit_source) = source.unit_mut().unwrap();
            let (player_target,unit_target) = target.unit_mut().unwrap();
            if player_source != player_target {
                // it's an opposing unit so we try to damage it (it will damage us back)
                let damage_to_target = unit_source.apply_damage(unit_target);
                let damage_to_source = unit_target.apply_damage(unit_source);
                self.remove_dead(from);
                self.remove_dead(to);
                Ok(ActionOutcome::Damaged { to_source: damage_to_source, to_target: damage_to_target })
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
    pub fn unit_repair(&mut self, from: Coord, to: Coord) -> Result<ActionOutcome,()> {
        if self.in_range(1, from, to) && 
            self[from].is_unit() && 
            self[to].is_unit() 
        {
            let [source, target] = self.get_two_cells_mut(from, to).unwrap();
            let (player_source,unit_source) = source.unit_mut().unwrap();
            let (player_target,unit_target) = target.unit_mut().unwrap();
            if player_source == player_target {
                // it's a friendly unit so we can try to repair it
                let repair_amount = unit_source.apply_repair(unit_target);
                Ok(ActionOutcome::Repaired { amount: repair_amount })
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
    pub fn action_from_coords(&mut self, from: impl Into<Coord>, to: impl Into<Coord>) -> Result<Action,()> {
        let (from, to) = (from.into(),to.into());
        if self.in_range(1, from, to) && 
            self[from].is_unit() && 
            self.player() == self[from].player().unwrap() 
        {
            // it's our turn and we are acting on our own unit
            if from == to {
                // destination is same as source => we wish to skip this move
                Ok(Action::Pass)
            } else if self[to].is_empty() {
                // destination empty so this is a move
                Ok(Action::Move { from, to })
            } else if self[to].is_unit() {
                // destination is a unit
                let [source, target] = self.get_two_cells_mut(from, to).unwrap();
                let (player_source,unit_source) = source.unit_mut().unwrap();
                let (player_target,unit_target) = target.unit_mut().unwrap();
                if player_source != player_target {
                    // it's an opposing unit so we try to damage it (it will damage us back)
                    if unit_source.can_damage(unit_target) {
                        Ok(Action::Attack { from, to })
                    } else {
                        Err(())
                    }
                } else {
                    // it's our unit so we try to repair it (if repair not possible then action is not valid)
                    if unit_source.can_repair(unit_target) {
                        Ok(Action::Repair { from, to })
                    } else {
                        Err(())
                    }
                }
            } else {
                Err(())
            }
        } else {
            Err(())
        }
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
                let cell = self[Coord::new(row,col)];
                print!(" {}",cell.to_pretty_compact_string());
            }
            println!();
        }
    }
    pub fn coord_to_string(coord : Coord) -> String {
        let (row,col) = coord.to_tuple();
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
    type Output = BoardCell;
    fn index(&self, coord: Coord) -> & Self::Output {
        self.get_cell(coord).expect("expected valid coordinates")
    }
}

