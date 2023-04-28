use crate::{Coord, UnitType, BoardCell, Dim, Player, Board, DisplayFirstLetter, Action, ActionOutcome, CoordPair, DropOutcome, IsUsefulInfo, BoardCellData, HeuristicScore, DEFAULT_MAX_DEPTH, DEFAULT_BOARD_DIM, heuristics::{self, MIN_HEURISTIC_SCORE, MAX_HEURISTIC_SCORE}, Heuristics};
use anyhow::anyhow;
use smart_default::SmartDefault;
use rand::{Rng,seq::{IteratorRandom, SliceRandom}};
use std::{time::SystemTime, sync::{Arc, Mutex}, collections::HashMap};
#[cfg(feature="rayon")]
use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct Game {
    state: GameState,
    options: Arc<GameOptions>,
    stats: Arc<Mutex<GameStats>>,
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

#[derive(Debug, Clone, Default)]
pub struct GameStats {
    depth_counts : HashMap<usize,usize>,
    total_seconds : f32,
    total_effective_branches : usize,
    total_moves_per_effective_branch : usize,
}

#[derive(Debug, Clone, SmartDefault)]
pub struct GameOptions {
    #[default(DEFAULT_BOARD_DIM)]
    pub dim: Dim,
    pub drop_prob: Option<f32>,
    #[default(Some(DEFAULT_MAX_DEPTH))]
    pub max_depth: Option<usize>,
    pub max_moves: Option<usize>,
    pub max_seconds: Option<f32>,
    pub heuristics: Heuristics,
    pub mutual_damage: bool,
    pub debug : bool,
    pub adjust_max_depth : bool,
}

impl Default for Game {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl Game {
    pub fn new(options: GameOptions) -> Self 
    {
        let dim = options.dim;
        let mut game = Self {
            state: GameState::new(dim),
            options: Arc::new(options),
            stats: Default::default(),
        };
        assert!(dim >= 4,"initial setup requires minimum of 4x4 board");
        use UnitType::*;
        let init = vec![
            (0,0,AI),(0,1,Virus),(0,2,Program),
            (1,0,Tech),(1,1,Firewall),
            (2,0,Program),
        ];
        assert_eq!(Player::cardinality(),2);
        let mut p_all = Player::all();
        let p1 = p_all.next().unwrap();
        let p2 = p_all.next().unwrap();
        for (row,col,unit_type) in init {
            game.set_cell((row,col),BoardCell::new_unit(p2, unit_type));
            game.set_cell((dim-1-row,dim-1-col),BoardCell::new_unit(p1, unit_type));
        }
        game
    }
    pub fn dim(&self) -> Dim {
        self.options.dim
    }
    pub fn options(&self) -> GameOptions {
        self.options.as_ref().clone()
    }
    pub fn set_options(&mut self, options: GameOptions) {
        self.options = Arc::new(options);
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
    pub fn player_score(&self, player: Player) -> HeuristicScore {
        self.units().map(|cell| cell.score(player)).sum()
    }
    pub fn best_score_player(&self) -> Player {
        let player_score = self.player_score(self.player());
        if player_score > 0 {
            self.player()
        } else if player_score < 0 {
            self.player().next()
        } else {
            // if score is equal, the player that started second gets the points
            Player::default().next()
        }
    }
    pub fn end_game_result(&self) -> Option<Player>{
        if self.options.max_moves.is_some() && self.total_moves() >= self.options.max_moves.unwrap() {
            // max moves reached: draw
            // println!("DEBUG: reached max moves!");
            return Some(self.best_score_player())
        } 
        assert_eq!(Player::cardinality(),2);
        let mut p_all = Player::all();
        let p1 = p_all.next().unwrap();
        let p2 = p_all.next().unwrap();
        let mut ai_p1 = false;
        let mut ai_p2 = false;
        for c in self.state.board.iter_units() {
            if let Some((player,unit)) = c.player_unit() {
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
            Some(p1)
        } else if ai_p2 && !ai_p1 {
            Some(p2)
        } else if !ai_p2 && !ai_p1 {
            Some(self.best_score_player())
        } else {
            None
        }
    }
    pub fn parse_move_stdin(&self) -> Result<(Coord,Coord),String> {
        use std::io::Write;
        print!("{} player, enter your next move: ",self.player());
        std::io::stdout().flush().unwrap();
        let input = std::io::stdin().lines().next().unwrap().unwrap();
        let parsed = Self::parse_move(&input);
        parsed.ok_or(input)
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
        if self.options.drop_prob.is_some() && rng.gen::<f32>() < self.options.drop_prob.unwrap() {
            let unit_type = *[UnitType::Virus,UnitType::Tech].choose(&mut rng).expect("expect a hacker or repair");
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
            if let Some((_, unit)) = cell.player_unit() {
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
            println!("-> {}: {}", player, action);
            if self.options.debug {
                if outcome.is_useful_info() {
                    println!("# {}", outcome);
                }
                if drop_outcome.is_useful_info() {
                    println!("# {}", drop_outcome);
                }
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
            let mutual_damage = self.options.mutual_damage;
            let [source, target] = self.get_two_cell_data_mut(from, to).unwrap();
            let (player_source,unit_source) = source.unit_mut().unwrap();
            let (player_target,unit_target) = target.unit_mut().unwrap();
            if player_source != player_target {
                // it's an opposing unit so we try to damage it
                let mut damage_to_source = 0;
                if mutual_damage {
                    damage_to_source = unit_target.apply_damage(unit_source);
                }
                let damage_to_target = unit_source.apply_damage(unit_target);
                self.remove_dead(from);
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
                // destination is same as source => we wish to skip this move
                // Ok(Action::Pass)
                // destination is same as source => skip/pass disabled
                Err(anyhow!("can't pass"))
            } else if self[to].is_empty() {
                // destination empty so this is a move
                Ok(Action::Move { from, to })
            } else if self[to].is_unit() {
                // destination is a unit
                let (player_source,unit_source) = self[from].player_unit().unwrap();
                let (player_target,unit_target) = self[to].player_unit().unwrap();
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
        if self.options.debug {
            if let Some(max_moves) = self.options.max_moves {
                if self.total_moves() >= max_moves {
                    println!("# maximum moves played ({})",max_moves);
                } else {
                    println!("# {}/{} moves played",self.total_moves(),max_moves);
                }
            } else {
                println!("# {} moves played",self.total_moves());
            }
            if let Some(max_depth) = self.options.max_depth {
                println!("# Max search depth: {}",max_depth);
            }
            if let Some(max_seconds) = self.options.max_seconds {
                println!("# Max search time: {:.1} sec",max_seconds);
            }
            {
                let stats = self.stats.lock().expect("should get a lock");
                println!("# Total evals at each depth: {:?}",stats.depth_counts);
                let (dc, ct) = stats.depth_counts.iter().fold((0,0),|(dc,ct),(d,c)| (dc+d*c,ct+c));
                if ct > 0 {
                    println!("# Average eval depth: {:.1}",dc as f32/ct as f32);
                }
                if self.total_moves() > 0 {
                    println!("# Average eval time: {:.1}",stats.total_seconds as f32/self.total_moves() as f32); 
                }
                if stats.total_effective_branches > 0 {
                    println!("# Average branching factor: {:.1}",stats.total_moves_per_effective_branch as f32/stats.total_effective_branches as f32); 
                }
            }            
            println!("# Next player: {}",self.player());
        }
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
    pub fn unit_coord_pairs<'a>(&'a self) -> impl Iterator<Item = CoordPair> + 'a {
        self.state.board.iter_unit_coords().flat_map(|from| 
            self.state.board.iter_unit_coords().filter_map(move|to| 
                if from==to {None} else {Some(CoordPair::new(from,to))}))
    }
    pub fn heuristic(&self, player: Player, depth: usize) -> HeuristicScore {
        let result = self.end_game_result();
        // println!("vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv");
        // println!("DEBUG: for_player={:?} result={:?}",for_player,result);
        let moves = self.total_moves() as HeuristicScore;
        let score = match result {
            Some(winner) => {
                if winner == player {
                    // quicker win is better
                    HeuristicScore::MAX - moves  
                } else {
                    // later loss is better
                    HeuristicScore::MIN + moves
                }
            }
            // not finished so call appropriate heuristic
            None => {
                let heuristic = if player.is_attacker() {
                    self.options.heuristics.attacker.clone()
                } else {
                    self.options.heuristics.defender.clone()
                };
                heuristic(self,player)
            }
        };
        {   // update total count for this depth
            let mut stats = self.stats.lock().expect("lock should work");
            if let Some(count) = stats.depth_counts.remove(&depth) {
                stats.depth_counts.insert(depth, count+1);
            } else {
                stats.depth_counts.insert(depth, 1);
            }
        }
        // println!("score: {}",score);
        // self.pretty_print();
        // println!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
        score
    }
    pub fn suggest_action_rec(&self, maximizing_player: bool, player: Player, depth: usize, alpha: HeuristicScore, beta: HeuristicScore, start_time: SystemTime) -> (HeuristicScore, Option<Action>, f32) {
        let mut timeout = false;
        if let Some(max_seconds) = self.options.max_seconds {
            let elapsed_seconds = SystemTime::now().duration_since(start_time).unwrap().as_secs_f32();
            if elapsed_seconds > max_seconds {
                timeout = true;
            }
        }
        if timeout || self.options.max_depth.is_some() && depth >= self.options.max_depth.unwrap() || self.end_game_result().is_some() {
            (self.heuristic(player,depth),None,depth as f32)
        } else {
            let mut best_action = None;
            let mut best_score;
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
            if maximizing_player {
                best_score = heuristics::MIN_HEURISTIC_SCORE;
            } else {
                best_score = heuristics::MAX_HEURISTIC_SCORE;
            }
            let mut alpha = alpha;
            let mut beta = beta;
            for possible_action in possible_actions {
                let mut possible_game = self.clone();
                possible_game.play_turn_from_action(possible_action).expect("action should be valid");
                let (score, _, rec_avg_depth) = possible_game.suggest_action_rec(!maximizing_player, player, depth+1, alpha, beta, start_time);
                total_depth += rec_avg_depth;
                total_count += 1;
                // println!("DEBUG: depth={} best={:?} new={:?} new_action={:?}",depth, best_score,score,possible_action);
                if maximizing_player && score > best_score || !maximizing_player && score < best_score {
                    best_score = score;
                    best_action = Some(possible_action);
                }
                if maximizing_player {
                    if best_score > beta { break; }
                    alpha = std::cmp::max(alpha, best_score);
                } else {
                    if best_score < alpha { break; }
                    beta = std::cmp::min(beta, best_score);
                }
            }
            if total_count == 0 {
                (self.heuristic(player,depth),None,depth as f32)
                // (best_score, best_action, depth as f32)
            } else {
                self.stats.lock().expect("should get a lock").total_moves_per_effective_branch += total_count;
                self.stats.lock().expect("should get a lock").total_effective_branches += 1;
                (best_score, best_action, total_depth / total_count as f32)
            }
        }
    }
    #[cfg(feature="rayon")]
    pub fn suggest_action_rec_par(&self, maximizing_player: bool, player: Player, depth: usize, alpha: HeuristicScore, beta: HeuristicScore, start_time: SystemTime) -> (HeuristicScore, Option<Action>, f32) {
        assert_eq!(maximizing_player,true,"call only at top level");
        let mut best_action = None;
        let mut best_score = heuristics::MIN_HEURISTIC_SCORE;
        let mut total_depth = 0.0;
        let mut total_count = 0;
        let possible_actions = self.player_unit_coords(self.player()).flat_map(|coord|self.possible_actions_from_coord(coord));
        cfg_if::cfg_if! {
            if #[cfg(feature = "rand-actions")] {
                let mut possible_actions = possible_actions.collect::<Vec<_>>();
                let mut rng = rand::thread_rng();
                possible_actions.shuffle(&mut rng);
            } else {
                let possible_actions = possible_actions.collect::<Vec<_>>();
            }
        }
        let possible_games = possible_actions.par_iter().map(|&possible_action|{
            let mut possible_game = self.clone();
            possible_game.play_turn_from_action(possible_action).expect("action should be valid");
            let suggest = possible_game.suggest_action_rec(!maximizing_player, player, depth+1, alpha, beta, start_time);
            (suggest,possible_action)
        }).collect::<Vec<_>>();
        for ((score, _, rec_avg_depth),possible_action) in possible_games {
            total_depth += rec_avg_depth;
            total_count += 1;
            // println!("DEBUG: depth={} best={:?} new={:?} new_action={:?}",depth, best_score,score,possible_action);
            if score > best_score {
                best_score = score;
                best_action = Some(possible_action);
            }
        }
        if total_count == 0 {
            (best_score, best_action, depth as f32)
        } else {
            (best_score, best_action, total_depth / total_count as f32)
        }
    }
    pub fn suggest_action(&mut self) -> (HeuristicScore, Action, f32, f32) {
        let start_time = SystemTime::now();
        #[cfg(not(feature="rayon"))]
        let (score,suggestion, avg_depth) = 
            self.suggest_action_rec(true, self.player(), 0, MIN_HEURISTIC_SCORE, MAX_HEURISTIC_SCORE, start_time);
        #[cfg(feature="rayon")]
        let (score,suggestion, avg_depth) = 
            self.suggest_action_rec_par(true, self.player(), 0, MIN_HEURISTIC_SCORE, MAX_HEURISTIC_SCORE, start_time);
        let elapsed_seconds = SystemTime::now().duration_since(start_time).unwrap().as_secs_f32();
        (score,suggestion.expect("don't know what to do!"),elapsed_seconds,avg_depth)
    }
    pub fn adjust_max_depth(&mut self, elapsed_seconds: f32, avg_depth: f32) {
        let branching_factor = 5; // we could update this live
        let mut options = self.options();
        if options.max_depth.is_some() && avg_depth < options.max_depth.unwrap() as f32 * 0.9 {
            options.max_depth = Some(options.max_depth.unwrap()-1);
        } else if options.max_depth.is_some() && options.max_seconds.is_some() && elapsed_seconds < self.options.max_seconds.unwrap() / (branching_factor as f32 * 1.2) {
            options.max_depth = Some(options.max_depth.unwrap()+1);
        }
        self.set_options(options);
    }
    pub fn computer_play_turn(&mut self) {
        let mut options = self.options();
        options.drop_prob = None;
        let mut computer_game = self.clone();
        computer_game.set_options(options);
        let (score,best_action,elapsed_seconds,avg_depth) = computer_game.suggest_action();
        self.stats.lock().expect("should get the lock").total_seconds += elapsed_seconds;
        if self.options.adjust_max_depth {
            self.adjust_max_depth(elapsed_seconds, avg_depth);
        }
        if let Ok((player, action, outcome,drop_outcome)) = self.play_turn_from_action(best_action) {
            println!("-> {}: {}", player, action);
            if self.options.debug {
                if outcome.is_useful_info() {
                    println!("# {}", outcome);
                }
                if drop_outcome.is_useful_info() {
                    println!("# {}", drop_outcome);
                }
                println!("# Compute time: {:.1} sec", elapsed_seconds);
                println!("# Average depth: {:.1}", avg_depth);
                println!("# Score: {}", score);
            }
        } else {
            panic!("play turn should work");
        }
    }
    pub fn console_play_turn_stdin(&mut self) {
        loop {
            match self.parse_move_stdin() {
                Ok((from,to)) => {
                    if self.console_play_turn(from, to) {
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
    }
    pub fn console_quick_suggestion(&self) {
        let mut options = self.options();
        options.max_depth = Some(4);
        options.max_seconds = Some(0.5);
        let mut game_suggest = self.clone();
        game_suggest.set_options(options);
        let (_, suggestion,_,_) = game_suggest.suggest_action();
        println!("Suggestion: {}",suggestion);
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

