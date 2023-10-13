use crate::{Coord, UnitType, BoardCell, Dim, Player, Board, DisplayFirstLetter, Action, ActionOutcome, CoordPair, BoardCellData, HeuristicScore, DEFAULT_MAX_DEPTH, DEFAULT_BOARD_DIM, heuristics::{self, MIN_HEURISTIC_SCORE, MAX_HEURISTIC_SCORE}, Heuristics, DEFAULT_MIN_DEPTH, IsUsefulInfo, DEFAULT_MAX_MOVES, DEFAULT_MAX_SECONDS};

#[cfg(feature="stats")]
use crate::{number_digits_precision_to_string, rescale_number_to_string};

use anyhow::anyhow;
use smart_default::SmartDefault;
use rand::seq::SliceRandom;
use std::sync::Arc;
use instant::Instant;
use std::io::Write as IoWrite;
use std::io::Result as IoResult;

#[cfg(feature="stats")]
use itertools::Itertools;

#[cfg(feature="stats")]
use std::{sync::Mutex, collections::HashMap};

#[cfg(feature="rayon")]
use rayon::prelude::*;

pub mod console;
pub mod web;

#[cfg(feature="broker")]
pub mod broker;

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
    attacker_has_ai: bool,
    defender_has_ai: bool,
}

impl GameState {
    fn new(dim: Dim) -> Self {
        Self {
            player: Default::default(),
            board: Board::new(dim),
            total_moves: 0,
            deadlock: false,
            attacker_has_ai: false,
            defender_has_ai: false,
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
            attacker_has_ai: self.attacker_has_ai,
            defender_has_ai: self.defender_has_ai,
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
    total_nodes: usize,
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
    pub multi_threaded : bool,
    #[default(true)]
    pub rand_traversal : bool,
    #[default(true)]
    pub pruning: bool,
    #[default(1)]
    pub parallel_levels : usize,
    pub broker : Option<String>,
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
        game.state.attacker_has_ai = true;
        game.state.defender_has_ai = true;
        game
    }
    pub fn into_shallow_copy(self) -> Self {
        Self {
            state: self.state.into_shallow_copy(),
            options: self.options,
            #[cfg(feature="stats")]
            stats: self.stats,
        }
    }
    pub fn dim(&self) -> Dim {
        self.options.dim
    }
    #[cfg(feature="stats")]
    pub fn stats(&self) -> Arc<Mutex<GameStats>> {
        self.stats.clone()
    }
    #[cfg(feature="stats")]
    pub fn set_new_stats(&mut self) {
        self.stats = Default::default();
    }
    #[cfg(feature="stats")]
    pub fn reset_stats(&mut self) {
        let mut stats = self.stats.lock().expect("lock should work");
        *stats = Default::default();
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
        debug_assert!(is_valid,"({},{}) is not valid for a {}x{} board",row,col,self.dim(),self.dim());
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
        let wins_by_default = Player::Defender;
        if self.options.max_moves.is_some() && self.total_moves() >= self.options.max_moves.unwrap() {
            return Some(wins_by_default)
        } 
        match (self.state.attacker_has_ai,self.state.defender_has_ai) {
            (true, true) => None,
            (true, false) => Some(Player::Attacker),
            (false, true) => Some(Player::Defender),
            (false, false) => Some(wins_by_default),
        }
    }
    pub fn parse_move(move_str: &str) -> Option<(Coord,Coord)> {
        use regex::Regex;
        let re = Regex::new(r"[ \(\[]*([A-Za-z])[ ,;]*(\d+)[ \)\]]*[;,]*[ \(\[]*([A-Za-z])[ ,;]*(\d+)[ \)\]]*").unwrap();
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
    pub fn empty_coords(&self) -> impl Iterator<Item = Coord> + '_ {
        self.state.board.empty_coords()
    }
    pub fn player_coords(&self, player: Player) -> impl Iterator<Item = Coord> + '_ {
        self.state.board.player_coords(player)
    }
    pub fn remove_dead(&mut self, coord: Coord) {
        if let Some(cell) = self.get_cell(coord) {
            if let Some((player, unit)) = cell.player_unit() {
                if unit.health == 0 {
                    if unit.unit_type == UnitType::AI {
                        match *player {
                            Player::Attacker => self.state.attacker_has_ai = false,
                            Player::Defender => self.state.defender_has_ai = false,
                        }
                    }
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
    pub fn possible_actions_from_coord(&self, source : Coord) -> impl Iterator<Item=Action> + '_ {
        let rect_iter = source.rect_around(1).rect_iter();
        rect_iter.filter_map(move|target|self.action_from_coords(source, target).ok())
    }
    pub fn player_unit_coords(&self, player: Player) -> impl Iterator<Item = (Coord,&BoardCell)> + '_ {
        self.state.board.iter_player_unit_coords(player)
    }
    pub fn player_units(&self, player: Player) -> impl Iterator<Item = &BoardCell> + '_ {
        self.state.board.iter_player_units(player)
    }
    pub fn units(&self) -> impl Iterator<Item = &BoardCell> + '_ {
        self.state.board.iter_units()
    }
    pub fn unit_coords(&self) -> impl Iterator<Item = (Coord,&BoardCell)> + '_ {
        self.state.board.iter_unit_coords()
    }
    pub fn unit_coord_pairs(&self) -> impl Iterator<Item = (CoordPair,&BoardCell,&BoardCell)> + '_ {
        self.state.board.iter_unit_coords().flat_map(|(from,from_unit)| 
            self.state.board.iter_unit_coords().filter_map(move|(to,to_unit)| 
                if from==to {
                    None
                } else {
                    Some((CoordPair::new(from,to),from_unit,to_unit))
                }))
    }
    pub fn heuristic(&self, player: Player, maximizing_player: bool, depth: usize, opt_end_game_result: Option<Option<Player>>) -> HeuristicScore {
        let result = if let Some(end_game_result) = opt_end_game_result {
            end_game_result
        } else {
            self.end_game_result()
        };
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
        #[cfg(not(feature="stats"))]
        let _ = depth;
        #[cfg(feature="stats")]
        {   // update total count for this depth
            let mut stats = self.stats.lock().expect("lock should work");
            if let Some(count) = stats.depth_counts.remove(&depth) {
                stats.depth_counts.insert(depth, count+1);
            } else {
                stats.depth_counts.insert(depth, 1);
            }
        }
        score
    }
    pub fn minimax_alpha_beta(&self, maximizing_player: bool, player: Player, depth: usize, alpha: HeuristicScore, beta: HeuristicScore, start_time: Instant) -> (HeuristicScore, Option<Action>, f32) {
        #[cfg(feature="stats")]
        {
            self.stats.lock().expect("should get a lock").total_nodes += 1;
        }
        let mut timeout = false;
        if let Some(max_seconds) = self.options.max_seconds {
            let elapsed_seconds = Instant::now().duration_since(start_time).as_secs_f32();
            if elapsed_seconds > max_seconds {
                timeout = true;
            }
        }
        let mut opt_end_game_result : Option<Option<Player>> = None;
        if timeout && self.options.min_depth.is_some() && depth >= self.options.min_depth.unwrap()
            || self.options.max_depth.is_some() && depth >= self.options.max_depth.unwrap()
            || { 
                let end_game_result = self.end_game_result();
                opt_end_game_result=Some(end_game_result); 
                end_game_result.is_some()
            } 
        {
            (self.heuristic(player,maximizing_player,depth,opt_end_game_result),None,depth as f32)
        } else {
            let mut best_action = None;
            let mut best_score;
            let mut total_depth = 0.0;
            let mut total_count = 0;
            let mut possible_actions = self.player_unit_coords(self.player())
                .map(|(coord,_)| coord)
                .flat_map(|coord|self.possible_actions_from_coord(coord))
                .collect::<Vec<_>>();
            if self.options.rand_traversal {
                possible_actions.shuffle(&mut rand::thread_rng());
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
                let (score, _, rec_avg_depth) = possible_game.minimax_alpha_beta(!maximizing_player, player, depth+1, alpha, beta, start_time);
                total_depth += rec_avg_depth;
                total_count += 1;
                if maximizing_player && score >= best_score || !maximizing_player && score <= best_score {
                    best_score = score;
                    best_action = Some(possible_action);
                }
                if self.options.pruning {
                    if maximizing_player {
                        if best_score > beta { break; }
                        alpha = std::cmp::max(alpha, best_score);
                    } else {
                        if best_score < alpha { break; }
                        beta = std::cmp::min(beta, best_score);
                    }
                }
            }
            if total_count == 0 {
                (self.heuristic(player,maximizing_player,depth,opt_end_game_result),None,depth as f32)
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
    pub fn minimax_alpha_beta_par(&self, maximizing_player: bool, player: Player, depth: usize, alpha_parent: HeuristicScore, beta_parent: HeuristicScore, start_time: Instant) -> (HeuristicScore, Option<Action>, f32) {
        assert!(self.options.parallel_levels > 0,"this function should not be called if parallel levels is 0");
        #[cfg(feature="stats")]
        {
            self.stats.lock().expect("should get a lock").total_nodes += 1;
        }
        let mut timeout = false;
        if let Some(max_seconds) = self.options.max_seconds {
            let elapsed_seconds = Instant::now().duration_since(start_time).as_secs_f32();
            if elapsed_seconds > max_seconds {
                timeout = true;
            }
        }
        let mut opt_end_game_result : Option<Option<Player>> = None;
        if timeout && self.options.min_depth.is_some() && depth >= self.options.min_depth.unwrap()
            || self.options.max_depth.is_some() && depth >= self.options.max_depth.unwrap()
            || { 
                let end_game_result = self.end_game_result();
                opt_end_game_result=Some(end_game_result); 
                end_game_result.is_some()
            } 
        {
            (self.heuristic(player,maximizing_player,depth,opt_end_game_result),None,depth as f32)
        } else {
            #[derive(Default,Clone,Copy)]
            struct State {
                best_action: Option<Action>,
                best_score: HeuristicScore,
                total_depth: f32,
                total_count: usize,
                alpha: HeuristicScore,
                beta: HeuristicScore,
            }
            let mut state = State::default();
            state.alpha = alpha_parent;
            state.beta = beta_parent;
            let mut possible_actions = self.player_unit_coords(self.player())
                .flat_map(|coord|self.possible_actions_from_coord(coord))
                .collect::<Vec<_>>();
            if self.options.rand_traversal {
                possible_actions.shuffle(&mut rand::thread_rng());
            }
            if maximizing_player {
                state.best_score = heuristics::MIN_HEURISTIC_SCORE;
            } else {
                state.best_score = heuristics::MAX_HEURISTIC_SCORE;
            }
            if let Some(state_result) = possible_actions.into_par_iter().fold_with(state, |mut state, possible_action| {
                let mut prune = false;
                if self.options.pruning {
                    if maximizing_player {
                        if state.best_score > state.beta { prune=true; }
                    } else {
                        if state.best_score < state.alpha { prune=true; }
                    }
                }
                if !prune {
                    let mut possible_game = self.clone();
                    possible_game.play_turn_from_action(possible_action).expect("action should be valid");
                    let (score, _, rec_avg_depth) = if self.options.parallel_levels-1 > depth {
                        possible_game.minimax_alpha_beta_par(!maximizing_player, player, depth+1, state.alpha, state.beta, start_time)
                    } else {
                        possible_game.minimax_alpha_beta(!maximizing_player, player, depth+1, state.alpha, state.beta, start_time)
                    };
                    state.total_depth += rec_avg_depth;
                    state.total_count += 1;
                    if maximizing_player && score >= state.best_score || !maximizing_player && score <= state.best_score {
                        state.best_score = score;
                        state.best_action = Some(possible_action);
                    }
                    if self.options.pruning {
                        if maximizing_player {
                            state.alpha = std::cmp::max(state.alpha, state.best_score);
                        } else {
                            state.beta = std::cmp::min(state.beta, state.best_score);
                        }
                    }
                }
                state
            }).reduce_with(|mut state,state2| {
                if state2.best_score > state.best_score {
                    state.best_score = state2.best_score;
                    state.best_action = state2.best_action;
                }
                state.total_depth += state2.total_depth;
                state.total_count += state2.total_count;
                state
            }) {
                state = state_result;
            }
            if state.total_count == 0 {
                (self.heuristic(player,maximizing_player,depth,opt_end_game_result),None,depth as f32)
            } else {
                #[cfg(feature="stats")]
                {   // branching stats
                    let mut stats = self.stats.lock().expect("should get a lock");
                    stats.total_moves_per_effective_branch += state.total_count;
                    stats.total_effective_branches += 1;
                }
                (state.best_score, state.best_action, state.total_depth / state.total_count as f32)
            }
        }
    }
    pub fn suggest_action(&mut self) -> (HeuristicScore, Option<Action>, f32, f32) {
        let start_time = Instant::now();
        #[cfg(not(feature="rayon"))]
        let (score, suggestion, avg_depth) = 
            self.minimax_alpha_beta(true, self.player(), 0, MIN_HEURISTIC_SCORE, MAX_HEURISTIC_SCORE, start_time);
        #[cfg(feature="rayon")]
        let (score, suggestion, avg_depth) = 
            if self.options().multi_threaded && self.options.parallel_levels > 0 
            {
                self.minimax_alpha_beta_par(true, self.player(), 0, MIN_HEURISTIC_SCORE, MAX_HEURISTIC_SCORE, start_time)
            } else {
                self.minimax_alpha_beta(true, self.player(), 0, MIN_HEURISTIC_SCORE, MAX_HEURISTIC_SCORE, start_time)
            };
        let elapsed_seconds = Instant::now().duration_since(start_time).as_secs_f32();
        (score,suggestion,elapsed_seconds,avg_depth)
    }
    pub fn run_benchmark(&mut self, opt_max_seconds: Option<f32>) -> Option<usize> {
        let avg_branching_factor = 6.5; // adjust this manually based on historical data
        let mut max_depth = DEFAULT_MIN_DEPTH;
        let max_seconds = if let Some(max_seconds) = opt_max_seconds {
            max_seconds
        } else if let Some (max_seconds) = self.options().max_seconds {
            max_seconds*1.2
        } else {
            return None;
        };
        loop {
            max_depth += 1;
            let mut options = self.clone_options();
            options.max_seconds = Some(max_seconds);
            options.max_depth = Some(max_depth);
            let mut test_game = self.clone();
            test_game.set_options(options);
            #[cfg(feature="stats")]
            test_game.set_new_stats();
            let (_,_,elapsed_seconds,_avg_depth) = test_game.suggest_action();
            if elapsed_seconds > max_seconds*0.95 {
                max_depth -= 1;
                break;
            }
            if elapsed_seconds > max_seconds/avg_branching_factor {
                break;
            }
        }
        let mut options = self.clone_options();
        options.max_depth = Some(max_depth);
        self.set_options(options);
        Some(max_depth)
    }
    pub fn adjust_max_depth(&mut self, elapsed_seconds: f32, avg_depth: f32) {
        let avg_branching_factor = 6.5; // adjust this manually based on historical data
        let mut options = self.clone_options();
        if options.max_depth.is_some() && avg_depth < options.max_depth.unwrap() as f32 * 0.8 &&
            options.max_seconds.is_some() && elapsed_seconds > self.options.max_seconds.unwrap()*0.95 
        {
            options.max_depth = Some(options.max_depth.unwrap()-1);
        } else if options.max_depth.is_some() && options.max_seconds.is_some() && 
            elapsed_seconds < self.options.max_seconds.unwrap() / (avg_branching_factor * 1.2)
        {
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
                writeln!(w,"Current max search depth: {}",max_depth)?;
            }
            if let Some(max_seconds) = self.options.max_seconds {
                writeln!(w,"Current max search time: {:.1} sec",max_seconds)?;
            }
            #[cfg(feature="stats")]
            {
                let stats = self.stats.lock().expect("should get a lock");
                let (dc, counts_total) = stats.depth_counts.iter().fold((0,0),|(dc,ct),(d,c)| (dc+d*c,ct+c));
                if counts_total > 0 {
                    writeln!(w,"Cumulative evals: {}",rescale_number_to_string(counts_total as f32))?;
                    writeln!(w,"Cumulative % evals by depth: {}", stats.depth_counts.iter()
                        .sorted_by_key(|x| x.0)
                        .filter_map(|(k,v)|{
                            let pct = *v as f64 * 100.0 / counts_total as f64;
                            if pct < 0.1 {
                                None
                            } else {
                                Some(format!("{k}={}%", number_digits_precision_to_string(pct,1)))
                            }
                        }).join(" "))?;
                    writeln!(w,"Cumulative evals by depth: {}", stats.depth_counts.iter()
                        .sorted_by_key(|x| x.0)
                        .map(|(k,v)|{
                                format!("{k}={}", rescale_number_to_string(*v as f32))
                        }).join(" "))?;
                    writeln!(w,"Average eval depth: {:.1}",dc as f32/counts_total as f32)?;
                }
                if self.total_moves() > 0 {
                    writeln!(w,"Average time per move: {:.1}",stats.total_seconds/self.total_moves() as f32)?; 
                }
                if stats.total_effective_branches > 0 {
                    writeln!(w,"Average branching factor: {:.1}",stats.total_moves_per_effective_branch as f32/stats.total_effective_branches as f32)?; 
                }
                if (counts_total > 0 || stats.total_nodes > 0) && stats.total_seconds > 0.0 {
                    write!(w,"Perf. ")?;
                    if counts_total > 0 && stats.total_seconds > 0.0 {
                        write!(w,"Evals: {}/s  ",rescale_number_to_string(counts_total as f32/stats.total_seconds))?; 
                    }
                    if stats.total_nodes > 0 && stats.total_seconds > 0.0 {
                        write!(w,"Nodes: {}/s",rescale_number_to_string(stats.total_nodes as f32/stats.total_seconds))?; 
                    }
                    writeln!(w)?;
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
            write!(w,"{:>2}: ",(row as u8 + b'A') as char)?;
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
                if outcome.is_useful_info() {
                    writeln!(w,"{}", outcome)?;
                }
        }
            Ok(true)
        } else {
            Ok(false)
        }
    }
    pub fn computer_play_turn(&mut self, opt_w: Option<&mut impl IoWrite>) -> IoResult<Option<Action>> {
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
                    if outcome.is_useful_info() {
                        writeln!(w,"{}", outcome)?;
                    }
                    if self.options.debug {
                        writeln!(w,"Compute time: {:.1} sec", elapsed_seconds)?;
                        writeln!(w,"Average depth: {:.1}", avg_depth)?;
                        writeln!(w,"Heuristic score: {}", score)?;
                    }
                }
                Ok(Some(best_action))
            } else {
                panic!("play turn should work");
            }
        } else {
            self.set_deadlock(true);
            Ok(None)
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

