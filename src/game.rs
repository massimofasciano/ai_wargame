use crate::{Coord, UnitType, BoardCell, Dim, Player, Board, DisplayFirstLetter, Action, ActionOutcome, CoordPair, BoardCellData, HeuristicScore, DEFAULT_MAX_DEPTH, DEFAULT_BOARD_DIM, heuristics::{self, MIN_HEURISTIC_SCORE, MAX_HEURISTIC_SCORE}, Heuristics, DEFAULT_MIN_DEPTH, IsUsefulInfo, DEFAULT_MAX_MOVES, DEFAULT_MAX_SECONDS};

use anyhow::anyhow;
use smart_default::SmartDefault;
use rand::{seq::{SliceRandom}};
use std::sync::Arc;
use instant::Instant;
use std::io::Write as IoWrite;
use std::io::Result as IoResult;

#[cfg(feature="stats")]
use std::{sync::Mutex, collections::HashMap};

#[cfg(feature="rayon")]
use rayon::prelude::*;

pub mod console;
pub mod web;

#[derive(Debug, Clone)]
pub struct Game {
    state: GameState,
    options: Arc<GameOptions>,
    #[cfg(feature="stats")]
    stats: Arc<Mutex<GameStats>>,
}

#[derive(Debug, Clone)]
pub struct GameState {
    player: Player,
    board: Board,
    total_moves: usize,
    deadlock : bool,
}

impl GameState {
    fn new(dim: Dim) -> Self {
        Self {
            player: Default::default(),
            board: Board::new(dim),
            total_moves: 0,
            deadlock: false,
        }
    }
}
impl Default for GameState {
    fn default() -> Self {
        Self::new(DEFAULT_BOARD_DIM)
    }
}

impl GameState {
    pub fn into_shallow_copy(self) -> Self {
        Self {
            player: self.player,
            total_moves: self.total_moves,
            board: self.board,
            deadlock: self.deadlock,
        }
    }
}

#[cfg(feature="stats")]
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
    #[default(Some(DEFAULT_MAX_DEPTH))]
    pub max_depth: Option<usize>,
    #[default(Some(DEFAULT_MIN_DEPTH))]
    pub min_depth: Option<usize>,
    #[default(Some(DEFAULT_MAX_MOVES))]
    pub max_moves: Option<usize>,
    #[default(Some(DEFAULT_MAX_SECONDS))]
    pub max_seconds: Option<f32>,
    pub heuristics: Heuristics,
    #[default(true)]
    pub mutual_damage: bool,
    pub debug : bool,
    #[default(true)]
    pub adjust_max_depth : bool,
    pub move_while_engaged : bool,
    pub move_while_engaged_full_health : bool,
    #[default(true)]
    pub move_only_forward : bool,
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
            #[cfg(feature="stats")]
            stats: Default::default(),
        };
        assert!(dim >= 4,"initial setup requires minimum of 4x4 board");
        use UnitType::*;
        let init_p1 = vec![
            (0,0,AI),(0,1,Virus),(0,2,Program),
            (1,0,Virus),(1,1,Firewall),
            (2,0,Program),
        ];
        let init_p2 = vec![
            (0,0,AI),(0,1,Tech),(0,2,Firewall),
            (1,0,Tech),(1,1,Program),
            (2,0,Firewall),
        ];
        assert_eq!(Player::cardinality(),2);
        let mut p_all = Player::all();
        let p1 = p_all.next().unwrap();
        let p2 = p_all.next().unwrap();
        for (row,col,unit_type) in init_p1 {
            game.set_cell((dim-1-row,dim-1-col),BoardCell::new_unit(p1, unit_type));
        }
        for (row,col,unit_type) in init_p2 {
            game.set_cell((row,col),BoardCell::new_unit(p2, unit_type));
        }
        game
    }
    pub fn into_shallow_copy(self) -> Self {
        Self {
            state: self.state.into_shallow_copy(),
            options: self.options.clone(),
            #[cfg(feature="stats")]
            stats: self.stats.clone(),
        }
    }
    pub fn dim(&self) -> Dim {
        self.options.dim
    }
    pub fn stats(&self) -> Arc<Mutex<GameStats>> {
        self.stats.clone()
    }
    pub fn options(&self) -> Arc<GameOptions> {
        self.options.clone()
    }
    pub fn clone_options(&self) -> GameOptions {
        self.options.as_ref().clone()
    }
    pub fn set_options(&mut self, options: GameOptions) {
        self.options = Arc::new(options);
    }
    pub fn set_deadlock(&mut self, deadlock: bool) {
        self.state.deadlock = deadlock;
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
    pub fn next_turn(&mut self) -> Player {
        self.state.player = self.state.player.next();
        self.state.total_moves += 1;
        self.state.player
    }
    pub fn into_next_turn(self) -> Self {
        let mut next = self.into_shallow_copy();
        next.state.player = next.state.player.next();
        next.state.total_moves += 1;
        next
    }
    pub fn is_valid_position(&self, coord : Coord) -> bool {
        let (row,col) = coord.to_tuple();
        let is_valid = row >= 0 && col >= 0 && row < self.dim() && col < self.dim();
        debug_assert_eq!(is_valid,true,"({},{}) is not valid for a {}x{} board",row,col,self.dim(),self.dim());
        is_valid
    }
    pub fn is_valid_move(&self, from: Coord, to: Coord) -> bool {
        self.are_in_range(from, to, 1) &&
        self[to].is_empty() && self[from].is_unit() &&
        self.player() == self[from].player().unwrap() &&
        (self.options.move_while_engaged || self.can_move_while_engaged(from)
            || (self.options.move_while_engaged_full_health && self.is_full_health(from))
            || !self.is_engaged(from)) &&
        (!self.options.move_only_forward || self.can_move_back(from) || self.is_moving_forward(from,to))
    }
    pub fn is_full_health(&self, coord: Coord) -> bool {
        if let Some(cell) = self.get_cell(coord) {
            if let Some(unit) = cell.unit() {
                if unit.health == unit.initial_health() {
                    return true;
                }
            }
        }
        false
    }
    pub fn can_move_back(&self, coord: Coord) -> bool {
        if let Some(cell) = self.get_cell(coord) {
            if let Some(unit) = cell.unit() {
                if unit.can_move_back() {
                    return true;
                }
            }
        }
        false
    }
    pub fn can_move_while_engaged(&self, coord: Coord) -> bool {
        if let Some(cell) = self.get_cell(coord) {
            if let Some(unit) = cell.unit() {
                if unit.can_move_while_engaged() {
                    return true;
                }
            }
        }
        false
    }
    pub fn are_in_range(&self, from : Coord, to : Coord, range: Dim) -> bool {
        self.is_valid_position(from) && 
        self.is_valid_position(to) && 
        from.is_in_range(to, range)
    }
    pub fn is_moving_forward(&self, from : Coord, to : Coord) -> bool {
        if self.player().is_attacker() {
            from.row-to.row > 0 || from.col-to.col > 0
        } else {
            to.row-from.row > 0 || to.col-from.col > 0
        }
    }
    pub fn is_engaged(&self, coord: Coord) -> bool {
        let my_cell = self.get_cell(coord);
        if my_cell.is_none() {
            return false;
        }
        let my_player = my_cell.unwrap().player();
        if my_player.is_none() {
            return false;
        }
        let my_player = my_player.unwrap();
        coord.iter_neighbors().any(|neighbor|{
            if let Some(cell) = self.get_cell(neighbor) {
                if let Some(player) = cell.player() {
                    my_player != player
                } else {
                    false
                }
            } else {
                false
            }
        })
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
    pub fn end_game_result(&self) -> Option<Player>{
        assert_eq!(Player::cardinality(),2);
        if self.state.deadlock {
            // if deadlocked, we couldn't play a move so other player wins
            return Some(self.player().next())
        } 
        let p1 = Player::default();
        let p2 = p1.next();
        let wins_by_default = p2;
        if self.options.max_moves.is_some() && self.total_moves() >= self.options.max_moves.unwrap() {
            return Some(wins_by_default)
        } 
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
            Some(wins_by_default)
        } else {
            None
        }
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
            Action::SelfDestruct { from } => {
                self.unit_self_destruct(from)
            }
        }
    }
    pub fn play_turn_from_action(&mut self, action: Action) -> Result<(Player,Action,ActionOutcome),anyhow::Error> {
        let outcome = self.perform_action(action);
        if let Ok(outcome) = outcome {
            let player = self.player();
            self.next_turn();
            Ok((player,action,outcome))
        } else {
            Err(anyhow!("invalid action"))
        }
    }
    pub fn play_turn_from_coords(&mut self, from: impl Into<Coord>, to: impl Into<Coord>) -> Result<(Player,Action,ActionOutcome),anyhow::Error> {
        if let Ok(action) = self.action_from_coords(from, to) {
            self.play_turn_from_action(action)
        } else {
            Err(anyhow!("invalid coordinates or move"))
        }
    }
    pub fn unit_combat(&mut self, from: Coord, to: Coord) -> Result<ActionOutcome,anyhow::Error> {
        if self.are_in_range(from, to, 1) && 
            self[from].is_unit() && 
            self[to].is_unit() 
        {
            let mutual_damage = self.options.mutual_damage;
            let [source, target] = self.get_two_cell_data_mut(from, to).unwrap();
            let (player_source,unit_source) = source.player_unit_mut().unwrap();
            let (player_target,unit_target) = target.player_unit_mut().unwrap();
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
        if self.are_in_range(from, to, 1) && 
            self[from].is_unit() && 
            self[to].is_unit() 
        {
            let [source, target] = self.get_two_cell_data_mut(from, to).unwrap();
            let (player_source,unit_source) = source.player_unit_mut().unwrap();
            let (player_target,unit_target) = target.player_unit_mut().unwrap();
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
    pub fn unit_self_destruct(&mut self, from: Coord) -> Result<ActionOutcome,anyhow::Error> {
        if self.is_valid_position(from) && self[from].is_unit() {
            let mut total_damage = 0;
            for to in from.rect_around(1).rect_iter() {
                if from == to || !self.is_valid_position(to) || self[to].is_empty() {
                    continue;
                }
                let [source, target] = self.get_two_cell_data_mut(from, to).unwrap();
                let (_,unit_source) = source.player_unit_mut().unwrap();
                let (_,unit_target) = target.player_unit_mut().unwrap();
                total_damage += unit_source.apply_self_destruct(unit_target);
                self.remove_dead(to);
            }
            let (_, source) = self.get_cell_data_mut(from).expect("not empty").player_unit_mut().expect("not empty");
            source.kill();
            self.remove_dead(from);
            Ok(ActionOutcome::SelfDestructed{total_damage})
        } else {
            Err(anyhow!("invalid coordinates"))
        }
    }
    pub fn action_from_coords(&self, from: impl Into<Coord>, to: impl Into<Coord>) -> Result<Action,anyhow::Error> {
        let (from, to) = (from.into(),to.into());
        if self.are_in_range(from, to, 1) && 
            self[from].is_unit() && 
            self.player() == self[from].player().unwrap()
        {
            // it's our turn and we are acting on our own unit
            if from == to {
                // destination is same as source => self destruction!
                Ok(Action::SelfDestruct { from })
            } else if self.is_valid_move(from, to) {
                // destination empty and move validated (not engaged, etc...)
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
    pub fn heuristic(&self, player: Player, maximizing_player: bool, _depth: usize) -> HeuristicScore {
        let result = self.end_game_result();
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
                let heuristic = match (player.is_attacker(), maximizing_player) {
                    (true, true) => self.options.heuristics.attacker_max.clone(),
                    (true, false) => self.options.heuristics.attacker_min.clone(),
                    (false, true) => self.options.heuristics.defender_max.clone(),
                    (false, false) => self.options.heuristics.defender_min.clone(),
                };
                heuristic(self,player)
            }
        };
        #[cfg(feature="stats")]
        {   // update total count for this depth
            let depth = _depth;
            let mut stats = self.stats.lock().expect("lock should work");
            if let Some(count) = stats.depth_counts.remove(&depth) {
                stats.depth_counts.insert(depth, count+1);
            } else {
                stats.depth_counts.insert(depth, 1);
            }
        }
        score
    }
    pub fn suggest_action_rec(&self, maximizing_player: bool, player: Player, depth: usize, alpha: HeuristicScore, beta: HeuristicScore, start_time: Instant) -> (HeuristicScore, Option<Action>, f32) {
        let mut timeout = false;
        if let Some(max_seconds) = self.options.max_seconds {
            let elapsed_seconds = Instant::now().duration_since(start_time).as_secs_f32();
            if elapsed_seconds > max_seconds {
                timeout = true;
            }
        }
        if timeout && self.options.min_depth.is_some() && depth >= self.options.min_depth.unwrap()
            || self.options.max_depth.is_some() && depth >= self.options.max_depth.unwrap()
            || self.end_game_result().is_some() {
            (self.heuristic(player,maximizing_player,depth),None,depth as f32)
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
                if maximizing_player && score >= best_score || !maximizing_player && score <= best_score {
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
                (self.heuristic(player,maximizing_player,depth),None,depth as f32)
            } else {
                #[cfg(feature="stats")]
                {   // branching stats
                    let mut stats = self.stats.lock().expect("should get a lock");
                    stats.total_moves_per_effective_branch += total_count;
                    stats.total_effective_branches += 1;
                }
                (best_score, best_action, total_depth / total_count as f32)
            }
        }
    }
    #[cfg(feature="rayon")]
    pub fn suggest_action_rec_par(&self, maximizing_player: bool, player: Player, depth: usize, alpha: HeuristicScore, beta: HeuristicScore, start_time: Instant) -> (HeuristicScore, Option<Action>, f32) {
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
    pub fn suggest_action(&mut self) -> (HeuristicScore, Option<Action>, f32, f32) {
        let start_time = Instant::now();
        #[cfg(not(feature="rayon"))]
        let (score, suggestion, avg_depth) = 
            self.suggest_action_rec(true, self.player(), 0, MIN_HEURISTIC_SCORE, MAX_HEURISTIC_SCORE, start_time);
        #[cfg(feature="rayon")]
        let (score, suggestion, avg_depth) = 
            self.suggest_action_rec_par(true, self.player(), 0, MIN_HEURISTIC_SCORE, MAX_HEURISTIC_SCORE, start_time);
        let elapsed_seconds = Instant::now().duration_since(start_time).as_secs_f32();
        (score,suggestion,elapsed_seconds,avg_depth)
    }
    pub fn adjust_max_depth(&mut self, elapsed_seconds: f32, avg_depth: f32) {
        let branching_factor = 12; // we could update this live
        let mut options = self.clone_options();
        if options.max_depth.is_some() && avg_depth < options.max_depth.unwrap() as f32 * 0.9 {
            options.max_depth = Some(options.max_depth.unwrap()-1);
        } else if options.max_depth.is_some() && options.max_seconds.is_some() && elapsed_seconds < self.options.max_seconds.unwrap() / (branching_factor as f32 * 1.2) {
            options.max_depth = Some(options.max_depth.unwrap()+1);
        }
        self.set_options(options);
    }
    pub fn pretty_print_info(&self, w: &mut impl IoWrite) -> IoResult<()> {
        if let Some(max_moves) = self.options.max_moves {
            if self.total_moves() >= max_moves {
                writeln!(w,"maximum moves played ({})",max_moves)?;
            } else {
                writeln!(w,"{}/{} moves played",self.total_moves(),max_moves)?;
            }
        } else {
            writeln!(w,"{} moves played",self.total_moves())?;
        }
        if self.options.debug {
            if let Some(max_depth) = self.options.max_depth {
                writeln!(w,"Max search depth: {}",max_depth)?;
            }
            if let Some(max_seconds) = self.options.max_seconds {
                writeln!(w,"Max search time: {:.1} sec",max_seconds)?;
            }
            #[cfg(feature="stats")]
            {
                let stats = self.stats.lock().expect("should get a lock");
                writeln!(w,"Total evals at each depth: {:?}",stats.depth_counts)?;
                let (dc, ct) = stats.depth_counts.iter().fold((0,0),|(dc,ct),(d,c)| (dc+d*c,ct+c));
                if ct > 0 {
                    writeln!(w,"Average eval depth: {:.1}",dc as f32/ct as f32)?;
                }
                if self.total_moves() > 0 {
                    writeln!(w,"Average eval time: {:.1}",stats.total_seconds as f32/self.total_moves() as f32)?; 
                }
                if stats.total_effective_branches > 0 {
                    writeln!(w,"Average branching factor: {:.1}",stats.total_moves_per_effective_branch as f32/stats.total_effective_branches as f32)?; 
                }
            }            
            writeln!(w,"Next player: {}",self.player())?;
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
    pub fn human_play_turn_from_coords(&mut self, opt_w: Option<&mut impl IoWrite>, from: impl Into<Coord>, to: impl Into<Coord>) -> IoResult<bool> {
        if let Ok((player, action, outcome)) = self.play_turn_from_coords(from, to) {
            if let Some(w) = opt_w {
                writeln!(w,"{}: {}", player, action)?;
                if self.options.debug {
                    if outcome.is_useful_info() {
                        writeln!(w,"{}", outcome)?;
                    }
                }
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }
    pub fn computer_play_turn(&mut self, opt_w: Option<&mut impl IoWrite>) -> IoResult<()> {
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
                if let Some(w) = opt_w {
                    writeln!(w,"{}: {}", player, action)?;
                    if self.options.debug {
                        if outcome.is_useful_info() {
                            writeln!(w,"{}", outcome)?;
                        }
                        writeln!(w,"Compute time: {:.1} sec", elapsed_seconds)?;
                        writeln!(w,"Average depth: {:.1}", avg_depth)?;
                        writeln!(w,"Heuristic score: {}", score)?;
                    }
                }
            } else {
                panic!("play turn should work");
            }
        } else {
            self.set_deadlock(true);
        }
        Ok(())
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

