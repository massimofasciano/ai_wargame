use crate::{Coord, UnitType, BoardCell, Dim, Player, Board, DisplayFirstLetter, Action, ActionOutcome, CoordPair, DropOutcome, IsUsefulInfo, BoardCellData, HeuristicScore, DEFAULT_MAX_DEPTH, DEFAULT_HEURISTIC, Heuristic, DEFAULT_BOARD_DIM};
use anyhow::anyhow;
use rand::{Rng,seq::{IteratorRandom, SliceRandom}};
use std::rc::Rc;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct Game {
    state: GameState,
    info: Rc<GameInfo>,
}

#[derive(Debug, Clone)]
pub struct GameState {
    player: Player,
    board: Board,
    total_moves: usize,
}

impl GameState {
    fn new(dim: Dim) -> Self {
        Self {
            player: Default::default(),
            board: Board::new(dim),
            total_moves: 0,
        }
    }
}
impl Default for GameState {
    fn default() -> Self {
        Self::new(DEFAULT_BOARD_DIM)
    }
}

#[derive(Debug, Clone)]
pub struct GameInfo {
    dim: Dim,
    drop_prob: Option<f32>,
    max_depth: Option<usize>,
    max_moves: Option<usize>,
    max_seconds: Option<f32>,
    heuristics: GameHeuristics,
}

impl GameInfo {
    fn new(dim: Dim, drop_prob: Option<f32>, max_depth: Option<usize>, max_moves: Option<usize>, 
            max_seconds: Option<f32>, heuristics: GameHeuristics) -> Self 
    {
        Self { dim, drop_prob, max_depth, max_moves, max_seconds, heuristics }
    }
}
impl Default for GameInfo {
    fn default() -> Self {
        Self::new(DEFAULT_BOARD_DIM,None,Some(DEFAULT_MAX_DEPTH),None,None,Default::default())
    }
}

#[derive(Clone)]
pub struct GameHeuristics {
    attacker: Heuristic,
    defender: Heuristic,
}

impl GameHeuristics {
    fn new(attacker: Heuristic, defender: Heuristic) -> Self {
        Self { attacker, defender }
    }
}
impl Default for GameHeuristics {
    fn default() -> Self {
        Self::new(DEFAULT_HEURISTIC,DEFAULT_HEURISTIC)
    }
}

impl std::fmt::Debug for GameHeuristics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:#?}",vec!["attacker_heuristic","defender_heuristic"])
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new(DEFAULT_BOARD_DIM, DEFAULT_HEURISTIC, DEFAULT_HEURISTIC, 
            None, Some(DEFAULT_MAX_DEPTH),None,None)
    }
}

impl Game {
    pub fn new(dim: Dim, attacker_heuristic: Heuristic, defender_heuristic: Heuristic, 
            drop_prob: Option<f32>, max_depth: Option<usize>, max_moves: Option<usize>, max_seconds: Option<f32>) -> Self 
    {
        let heuristics = GameHeuristics::new(attacker_heuristic,defender_heuristic);
        let mut game = Self {
            state: GameState::new(dim),
            info: Rc::new(GameInfo::new(dim,drop_prob,max_depth,max_moves,max_seconds,heuristics)),
        };
        // assert!(dim >= 4,"initial setup requires minimum of 4x4 board");
        use UnitType::*;
        let md = dim-1;
        // let init = vec![
        //     (0,1,Repair), (0,md-1,Hacker),
        //     (1,1,Soldier), (1,md-1,Drone),
        //     (0,3,Hacker), (0,md-3,Repair),
        //     (1,3,Drone), (1,md-3,Soldier),
        //     (0,2,AI), (0,md-2,AI),
        //     (1,2,Tank), (1,md-2,Tank),
        // ];
        let mid = dim/2;
        let init = vec![
            (1,1,Drone),    (1,mid-1,Hacker), (1,mid,Tank),     (1,md-1,Soldier),
                            (0,mid-1,Repair), (0,mid,AI), 
        ];
        // let init = vec![
        //                     (0,mid,AI), 
        // ];
        assert_eq!(Player::cardinality(),2);
        let mut p_all = Player::all();
        let p1 = p_all.next().unwrap();
        let p2 = p_all.next().unwrap();
        for (row,col,unit_type) in init {
            game.set_cell((row,col),BoardCell::new_unit(p2, unit_type));
            game.set_cell((md-row,col),BoardCell::new_unit(p1, unit_type));
        }
        game
    }
    pub fn dim(&self) -> Dim {
        self.info.dim
    }
    pub fn set_drop_prob(&mut self, drop_prob: Option<f32>) {
        let mut info = self.info.as_ref().clone();
        info.drop_prob = drop_prob;
        self.info = Rc::new(info);
    }
    pub fn remove_cell(&mut self, coord: Coord) -> Option<BoardCell> {
        if self.is_valid_position(coord) {
            self.state.board.remove(coord)
        } else {
            None
        }
    }
    pub fn get_cell(&self, coord: Coord) -> Option<&BoardCell> {
        if self.is_valid_position(coord) {
            Some(self.state.board.get(coord).unwrap())
        } else {
            None
        }
    }
    pub fn get_cell_data_mut(&mut self, coord: Coord) -> Option<&mut BoardCellData> {
        if self.is_valid_position(coord) {
            self.state.board.get_data_mut(coord)
        } else {
            None
        }
    }
    pub fn set_cell(&mut self, coord: impl Into<Coord>, value: BoardCell) {
        let coord = coord.into();
        if self.is_valid_position(coord) {
            self.state.board.set(coord,value);
        }
    }
    pub fn get_two_cell_data_mut(&mut self, coord0: Coord, coord1: Coord) -> Option<[&mut BoardCellData;2]> {
        if self.is_valid_position(coord0) &&
            self.is_valid_position(coord1) &&
            coord0 != coord1
        {
            self.state.board.get_two_data_mut(coord0, coord1)
        } else {
            None
        }
    }
    pub fn player(&self) -> Player {
        self.state.player
    }
    pub fn total_moves(&self) -> usize {
        self.state.total_moves
    }
    pub fn next_player(&mut self) -> Player {
        self.state.player = self.state.player.next();
        self.state.total_moves += 1;
        self.state.player
    }
    pub fn is_valid_position(&self, coord : Coord) -> bool {
        let (row,col) = coord.to_tuple();
        let is_valid = row >= 0 && col >= 0 && row < self.dim() && col < self.dim();
        debug_assert_eq!(is_valid,true,"({},{}) is not valid for a {}x{} board",row,col,self.dim(),self.dim());
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
    // pub fn in_range(&self, range: u8, coord0 : Coord, coord1 : Coord) -> bool {
    //     coord0 == coord1 || // we consider our own position as in range
    //     self.is_valid_position(coord0) && 
    //     self.is_valid_position(coord1) && 
    //     (coord1.row - coord0.row).abs() as u8 <= range && 
    //     (coord1.col - coord0.col).abs() as u8 <= range
    // }
    pub fn in_range(&self, range: u8, from : Coord, to : Coord) -> bool {
        // no diagonals and same position not allowed
        self.is_valid_position(from) && 
        self.is_valid_position(to) && 
        ((to.row - from.row).abs() + (to.col - from.col).abs()) as u8 == range
    }
    pub fn unit_move(&mut self, from: Coord, to: Coord) -> Result<ActionOutcome,anyhow::Error> {
        if self.is_valid_move(from, to) {
            let removed = self.remove_cell(from).unwrap();
            self.set_cell(to,removed);
            Ok(ActionOutcome::Moved { delta: to-from })
        } else {
            Err(anyhow!("not a valid move"))
        }
    }
    pub fn end_game_result(&self) -> Option<Option<Player>>{
        if self.info.max_moves.is_some() && self.total_moves() >= self.info.max_moves.unwrap() {
            // max moves reached: draw
            // println!("DEBUG: reached max moves!");
            return Some(None)
        } 
        assert_eq!(Player::cardinality(),2);
        let mut p_all = Player::all();
        let p1 = p_all.next().unwrap();
        let p2 = p_all.next().unwrap();
        let mut ai_p1 = false;
        let mut ai_p2 = false;
        for c in self.state.board.iter_units() {
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
        CoordPair::from_dim(self.dim())
    }
    pub fn rect_iter(&self) -> impl Iterator<Item = Coord> {
        self.board_rect().rect_iter()
    }
    pub fn empty_coords<'a>(&'a self) -> impl Iterator<Item = Coord> + 'a {
        self.state.board.empty_coords()
    }
    pub fn player_coords<'a>(&'a self, player: Player) -> impl Iterator<Item = Coord> + 'a {
        self.state.board.player_coords(player)
    }
    pub fn random_drop(&mut self) -> DropOutcome {
        let mut rng = rand::thread_rng();
        if self.info.drop_prob.is_some() && rng.gen::<f32>() < self.info.drop_prob.unwrap() {
            let unit_type = *[UnitType::Hacker,UnitType::Repair].choose(&mut rng).expect("expect a hacker or repair");
            if let Some(empty_coord) = self.empty_coords().choose(&mut rng) {
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
                }
            }
        }
    }
    pub fn perform_action(&mut self, action: Action) -> Result<ActionOutcome,anyhow::Error> {
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
    pub fn play_turn_from_action(&mut self, action: Action) -> Result<(Player,Action,ActionOutcome,DropOutcome),anyhow::Error> {
        let outcome = self.perform_action(action);
        if let Ok(outcome) = outcome {
            let drop_outcome = self.random_drop();
            let player = self.player();
            self.next_player();
            Ok((player,action,outcome,drop_outcome))
        } else {
            Err(anyhow!("invalid action"))
        }
    }
    pub fn play_turn_from_coords(&mut self, from: impl Into<Coord>, to: impl Into<Coord>) -> Result<(Player,Action,ActionOutcome,DropOutcome),anyhow::Error> {
        if let Ok(action) = self.action_from_coords(from, to) {
            self.play_turn_from_action(action)
        } else {
            Err(anyhow!("invalid coordinates or move"))
        }
    }
    pub fn console_play_turn(&mut self, from: impl Into<Coord>, to: impl Into<Coord>) -> bool {
        if let Ok((player, action, outcome,drop_outcome)) = self.play_turn_from_coords(from, to) {
            println!("# {} {}", player, action);
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
    pub fn unit_combat(&mut self, from: Coord, to: Coord) -> Result<ActionOutcome,anyhow::Error> {
        if self.in_range(1, from, to) && 
            self[from].is_unit() && 
            self[to].is_unit() 
        {
            let [source, target] = self.get_two_cell_data_mut(from, to).unwrap();
            let (player_source,unit_source) = source.unit_mut().unwrap();
            let (player_target,unit_target) = target.unit_mut().unwrap();
            if player_source != player_target {
                // // it's an opposing unit so we try to damage it (it will damage us back)
                // let damage_to_target = unit_source.apply_damage(unit_target);
                // let damage_to_source = unit_target.apply_damage(unit_source);
                // self.remove_dead(from);
                // self.remove_dead(to);
                // it's an opposing unit so we try to damage it (no return damage)
                let damage_to_target = unit_source.apply_damage(unit_target);
                let damage_to_source = 0;
                self.remove_dead(to);
                Ok(ActionOutcome::Damaged { to_source: damage_to_source, to_target: damage_to_target })
            } else {
                Err(anyhow!("can't attack friendly units"))
            }
        } else {
            Err(anyhow!("out of range or invalid coordinates"))
        }
    }
    pub fn unit_repair(&mut self, from: Coord, to: Coord) -> Result<ActionOutcome,anyhow::Error> {
        if self.in_range(1, from, to) && 
            self[from].is_unit() && 
            self[to].is_unit() 
        {
            let [source, target] = self.get_two_cell_data_mut(from, to).unwrap();
            let (player_source,unit_source) = source.unit_mut().unwrap();
            let (player_target,unit_target) = target.unit_mut().unwrap();
            if player_source == player_target {
                // it's a friendly unit so we can try to repair it
                let repair_amount = unit_source.apply_repair(unit_target);
                Ok(ActionOutcome::Repaired { amount: repair_amount })
            } else {
                Err(anyhow!("can only repair friendly units"))
            }
        } else {
            Err(anyhow!("out of range or invalid coordinates"))
        }
    }
    pub fn action_from_coords(&self, from: impl Into<Coord>, to: impl Into<Coord>) -> Result<Action,anyhow::Error> {
        let (from, to) = (from.into(),to.into());
        if self.in_range(1, from, to) && 
            self[from].is_unit() && 
            self.player() == self[from].player().unwrap() 
        {
            // it's our turn and we are acting on our own unit
            if from == to {
                // // destination is same as source => we wish to skip this move
                // Ok(Action::Pass)
                // destination is same as source => skip/pass disabled
                Err(anyhow!("can't pass"))
            } else if self[to].is_empty() {
                // destination empty so this is a move
                Ok(Action::Move { from, to })
            } else if self[to].is_unit() {
                // destination is a unit
                let (player_source,unit_source) = self[from].unit().unwrap();
                let (player_target,unit_target) = self[to].unit().unwrap();
                if player_source != player_target {
                    // it's an opposing unit so we try to damage it (it will damage us back)
                    if unit_source.can_damage(unit_target) {
                        Ok(Action::Attack { from, to })
                    } else {
                        Err(anyhow!("can't damage unit"))
                    }
                } else {
                    // it's our unit so we try to repair it (if repair not possible then action is not valid)
                    if unit_source.can_repair(unit_target) {
                        Ok(Action::Repair { from, to })
                    } else {
                        Err(anyhow!("can't repair unit"))
                    }
                }
            } else {
                Err(anyhow!("invalid target coordinate"))
            }
        } else {
            Err(anyhow!("not in range or source is not friendly unit"))
        }
    }
    pub fn pretty_print(&self) {
        println!("{} moves played",self.total_moves());
        println!("Next player: {}",self.player());
        print!("    ");
        for col in 0..self.dim() {
            print!(" {:>2} ",col);
        }
        println!();
        for row in 0..self.dim() {
            print!("{:>2}: ",(row as u8 +'A' as u8) as char);
            for col in 0..self.dim() {
                let cell = self[Coord::new(row,col)];
                print!(" {}",cell.to_pretty_compact_string());
            }
            println!();
        }
    }
    pub fn possible_actions_from_coord<'a>(&'a self, source : Coord) -> impl Iterator<Item=Action> + 'a {
        let rect_iter = source.rect_around(1).rect_iter();
        rect_iter.filter_map(move|target|self.action_from_coords(source, target).ok())
    }
    pub fn player_unit_coords<'a>(&'a self, player: Player) -> impl Iterator<Item = Coord> + 'a {
        self.state.board.iter_player_unit_coords(player)
    }
    pub fn player_units<'a>(&'a self, player: Player) -> impl Iterator<Item = &BoardCell> + 'a {
        self.state.board.iter_player_units(player)
    }
    pub fn units<'a>(&'a self) -> impl Iterator<Item = &BoardCell> + 'a {
        self.state.board.iter_units()
    }
    pub fn heuristic(&self, controlling_player: Player) -> HeuristicScore {
        // controlling_player: the player that will make the final move
        // current_player: the player that is making the virtual move at this level of the search tree
        let result = self.end_game_result();
        // if it's player N's turn to play it means we got to this state because of a move by player N-1 (or N+1 as it is cyclic)
        // so we evaluate this state's score from player N+1's perspective
        let for_player = self.player().next();
        // println!("vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv");
        // println!("DEBUG: for_player={:?} result={:?}",for_player,result);
        let moves = self.total_moves() as HeuristicScore;
        let score = match result {
            Some(Some(winner)) => {
                if winner == for_player {
                    // quicker win is better
                    HeuristicScore::MAX - moves  
                } else {
                    // later loss is better
                    HeuristicScore::MIN + moves
                }
            }
            // draw (bad but better than loss, later is better)
            Some(None) => HeuristicScore::MIN / 2 + moves,
            // not finished so call appropriate heuristic
            None => {
                let heuristic = if for_player.is_attacker() {
                    self.info.heuristics.attacker
                } else {
                    self.info.heuristics.defender
                };
                // we subtract moves/10 to make later states of equal value worse
                heuristic(self,for_player) - moves/10
            }
        };
        // println!("score: {}",score);
        // self.pretty_print();
        // println!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
        if controlling_player == for_player {
            -score
        } else {
            score
        }
    }
    pub fn suggest_action_rec(&self, controlling_player: Player, depth: usize, start_time: SystemTime) -> (Option<HeuristicScore>, Option<Action>, f32) {
        let mut timeout = false;
        if let Some(max_seconds) = self.info.max_seconds {
            let elapsed_seconds = SystemTime::now().duration_since(start_time).unwrap().as_secs_f32();
            if elapsed_seconds > max_seconds {
                timeout = true;
            }
        }
        if timeout || self.info.max_depth.is_some() && depth >= self.info.max_depth.unwrap() || self.end_game_result().is_some() {
            (Some(self.heuristic(controlling_player)),None,depth as f32)
        } else {
            let mut best_action = None;
            let mut best_score = None;
            let mut total_depth = 0.0;
            let mut total_count = 0;
            let possible_actions = self.player_unit_coords(self.player()).flat_map(|coord|self.possible_actions_from_coord(coord));
            cfg_if::cfg_if! {
                if #[cfg(feature = "rand-actions")] {
                    let mut possible_actions = possible_actions.collect::<Vec<_>>();
                    let mut rng = rand::thread_rng();
                    possible_actions.shuffle(&mut rng);
                } else {
                }
            }
            for possible_action in possible_actions {
                let mut possible_game = self.clone();
                possible_game.play_turn_from_action(possible_action).expect("action should be valid");
                let (score, _, rec_avg_depth) = possible_game.suggest_action_rec(controlling_player, depth+1, start_time);
                total_depth += rec_avg_depth;
                total_count += 1;
                // println!("DEBUG: depth={} best={:?} new={:?} new_action={:?}",depth, best_score,score,possible_action);
                if best_score.is_none() || score.is_some() && 
                (controlling_player == self.player() && score.unwrap() > best_score.unwrap() ||
                    controlling_player != self.player() && score.unwrap() < best_score.unwrap())
                 {
                    best_score = score;
                    best_action = Some(possible_action);
                }
            }
            if total_count == 0 {
                (None,None,depth as f32)
            } else {
                (best_score, best_action, total_depth / total_count as f32)
            }
        }
    }
    pub fn suggest_action(&self) -> (Option<HeuristicScore>, Action, f32, f32) {
        let start_time = SystemTime::now();
        let (score,suggestion, avg_depth) = self.suggest_action_rec(self.player(), 0, start_time);
        let elapsed_seconds = SystemTime::now().duration_since(start_time).unwrap().as_secs_f32();
        (score,suggestion.expect("don't know what to do!"),elapsed_seconds,avg_depth)
    }
    pub fn computer_play_turn(&mut self) {
        let mut computer_game = self.clone();
        computer_game.set_drop_prob(None);
        let (score,best_action,elapsed_seconds,avg_depth) = computer_game.suggest_action();
        if let Ok((player, action, outcome,drop_outcome)) = self.play_turn_from_action(best_action) {
            println!("# {} {}", player, action);
            if outcome.is_useful_info() {
                println!("# {}", outcome);
            }
            if drop_outcome.is_useful_info() {
                println!("# {}", drop_outcome);
            }
            println!("# Compute time: {:.1} sec", elapsed_seconds);
            println!("# Average depth: {:.1}", avg_depth);
            if score.is_some() { 
                println!("# Score: {}", score.unwrap());
            }
        } else {
            panic!("play turn should work");
        }
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.player().to_first_letter())?;
        for c in self.rect_iter() {
            write!(f,":{}",self.get_cell(c).unwrap())?;
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

