const BOARD_DIM: Dim = 10;
const BOARD_SIZE: usize = BOARD_DIM as usize*BOARD_DIM as usize;

type Board = [Cell;BOARD_SIZE];
type Dim = i8;
type Coord = (Dim,Dim);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Game {
    player: Player,
    board: Board,
    dim: Dim,
    total_moves: usize,
    drop_prob: Option<f32>,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            player: Default::default(),
            board: [Default::default();BOARD_SIZE],
            dim : BOARD_DIM,
            total_moves : 0,
            drop_prob : None,
        }
    }
}

impl Game {
    pub fn new(drop_prob: Option<f32>) -> Self {
        let md = BOARD_DIM-1;
        let mut game = Self::default();
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
    pub fn get_cell(&self, coord : (i8, i8)) -> Option<&Cell> {
        if Self::is_valid_position(coord) {
            Some(&self.board[Self::to_index(coord)])
        } else {
            None
        }
    }
    pub fn get_cell_mut(&mut self, coord : (i8, i8)) -> Option<&mut Cell> {
        if Self::is_valid_position(coord) {
            Some(&mut self.board[Self::to_index(coord)])
        } else {
            None
        }
    }
    fn to_index((row, col): (i8, i8)) -> usize {
        row as usize*BOARD_DIM as usize+col as usize
    }
    pub fn get_2_cells_mut(&mut self, coord0: Coord, coord1: Coord) -> Option<(&mut Cell, &mut Cell)> {
        if Self::is_valid_position(coord0) &&
            Self::is_valid_position(coord1) &&
            coord0 != coord1
        {
            let ref_mut_0;
            let ref_mut_1;
            unsafe {
                ref_mut_0 = &mut *(self.board.get_unchecked_mut(Self::to_index(coord0)) as *mut _);
                ref_mut_1 = &mut *(self.board.get_unchecked_mut(Self::to_index(coord1)) as *mut _);
            }
            Some((ref_mut_0, ref_mut_1))
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
        self.player = match self.player {
            Player::Blue => Player::Red,
            Player::Red => Player::Blue,
        };
        self.total_moves += 1;
        self.player
    }
    pub fn is_valid_position((row,col) : (i8, i8)) -> bool {
        row >= 0 && col >= 0 && row < BOARD_DIM && col < BOARD_DIM
    }
    pub fn is_valid_move(&mut self, from: Coord, to: Coord) -> bool {
        Self::neighbors(from, to) &&
        self[to].is_empty() && self[from].is_unit() &&
        self.player() == self[from].player().unwrap()
    }
    pub fn neighbors(coord0 : Coord, coord1 : Coord) -> bool {
        coord0 != coord1 &&
        Self::is_valid_position(coord0) && Self::is_valid_position(coord1) && 
        (coord1.0 - coord0.0).abs() <= 1 && (coord1.1 - coord0.1).abs() <= 1
    }
    pub fn in_range(range: u8, coord0 : Coord, coord1 : Coord) -> bool {
        coord0 == coord1 || // we consider our own position as in range
        Self::is_valid_position(coord0) && 
        Self::is_valid_position(coord1) && 
        (coord1.0 - coord0.0).abs() as u8 <= range && 
        (coord1.1 - coord0.1).abs() as u8 <= range
    }
    pub fn move_unit(&mut self, from: Coord, to: Coord) -> bool {
        if self.is_valid_move(from, to) {
            self[to] = self[from];
            self[from] = Cell::Empty;
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
    //                         if Self::is_valid_position(coord_target) && self[coord_target].is_unit() && coord_target != coord_source
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
    pub fn winner(&self) -> Option<Option<Player>>{
        let mut ai_red = false;
        let mut ai_blue = false;
        for c in self.board.iter() {
            if let Some((player,unit)) = c.unit() {
                if player == &Player::Red && unit.unit_type == UnitType::AI {
                    ai_red = true;
                }
                if player == &Player::Blue && unit.unit_type == UnitType::AI {
                    ai_blue = true;
                }
            }
            if ai_blue && ai_red { break; }
        }
        if ai_blue && !ai_red {
            Some(Some(Player::Blue))
        } else if ai_red && !ai_blue {
            Some(Some(Player::Red))
        } else if !ai_red && !ai_blue {
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
        let valid = if Self::in_range(1, from, to) && 
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
                let (source, target) = self.get_2_cells_mut(from, to).unwrap();
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
        for col in 0..BOARD_DIM {
            print!(" {:>2} ",col);
        }
        println!();
        for row in 0..BOARD_DIM {
            print!("{:>2}: ",(row as u8 +'A' as u8) as char);
            for col in 0..BOARD_DIM {
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

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.player().to_char())?;
        for c in self.board.iter() {
            write!(f,":{}",c)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_compact_string())
    }
}

trait DisplayFirstLetter : std::fmt::Display {
    fn to_char(&self) -> char {
        self.to_string().chars().next().unwrap()
    }
}

impl DisplayFirstLetter for UnitType {}
impl DisplayFirstLetter for Player {}

impl std::ops::Index<Coord> for Game {
    type Output = Cell;
    fn index(&self, coord: Coord) -> & Self::Output {
        // self.get_cell(coord).unwrap_or(&Cell::Outside)
        self.get_cell(coord).expect("expected valid coordinates")
    }
}

impl std::ops::IndexMut<Coord> for Game {
    fn index_mut(&mut self, coord: Coord) -> &mut Self::Output {
        // self.get_cell_mut(coord).unwrap_or(unsafe{&mut TEMP_CELL})
        self.get_cell_mut(coord).expect("expected valid coordinates")
    }
}

#[derive(Debug, PartialEq, PartialOrd, Default, Clone, Copy)]
#[derive(derive_more::Display)]
pub enum Player {
    #[default]
    Blue,
    Red,
}

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub enum Cell {
    #[default]
    Empty,
    Unit { player:Player, unit:Unit },
}

impl Cell {
    pub const fn new() -> Self {
        Self::Empty
    }
    pub fn new_unit(player: Player, unit_type: UnitType) -> Self {
        Self::Unit { 
            player,
            unit: Unit::new(unit_type),
        }
    }
    pub fn is_empty(&self) -> bool {
        *self == Self::Empty
    }
    pub fn is_unit(&self) -> bool {
        match self {
            Self::Unit { player: _, unit: _ } => true,
            _ => false,
        }
    }
    pub fn player(&self) -> Option<Player> {
        match self {
            Self::Unit { player, unit: _ } => Some(*player),
            _ => None,
        }
    }
    pub fn unit(&self) -> Option<(&Player,&Unit)> {
        match self {
            Self::Unit { player, unit } => Some((player,unit)),
            _ => None,
        }
    }
    pub fn unit_mut(&mut self) -> Option<(&mut Player,&mut Unit)> {
        match self {
            Self::Unit { player, unit } => Some((player,unit)),
            _ => None,
        }
    }
    pub fn remove_dead(&mut self) {
        if let Some((_, unit)) = self.unit() {
            if unit.health == 0 {
                *self = Cell::Empty;
            }
        }
    }
    pub fn interact(&mut self, target: &mut Self) {
        let (player_source,unit_source) = self.unit_mut().unwrap();
        let (player_target,unit_target) = target.unit_mut().unwrap();
        if player_source != player_target {
            // it's an opposing unit so we try to damage it (it will damage us back)
            unit_source.apply_damage(unit_target);
            unit_target.apply_damage(unit_source);
            self.remove_dead();
            target.remove_dead();
        } else {
            // it's our unit so we try to repair it
            unit_source.apply_repair(unit_target);
        }
    }
    pub fn to_pretty_compact_string(&self) -> String {
        if self.is_empty() {
            String::from(" . ")
        } else {
            self.to_compact_string()
        }
    }
    pub fn to_compact_string(&self) -> String {
        use Cell::*;
        match self {
            Empty => String::from(""),
            Unit { player, unit } => format!("{}{}{:1}",
                player.to_char().to_ascii_lowercase(),
                unit.unit_type.to_char().to_ascii_uppercase(),unit.health),
        }
    }
}

type Health = u8;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Unit {
    unit_type : UnitType,
    health : Health,
}

#[derive(Debug, PartialEq, PartialOrd,Default, Clone, Copy)]
#[derive(derive_more::Display)]
pub enum UnitType {
    AI,
    Hacker,
    Repair,
    Tank,
    Drone,
    #[default]
    Soldier,
}

impl Default for Unit {
    fn default() -> Self {
        Self::new(UnitType::default())
    }
}

impl Unit {
    pub fn new(unit_type : UnitType) -> Self {
        use UnitType::*;
        let health = match unit_type {
            AI => 5,
            Hacker => 3,
            Repair => 2,
            Tank => 9,
            Drone => 6,
            Soldier => 4,
        };
        Self { unit_type, health }
    }
    pub fn apply_repair(&mut self, target: &mut Self) -> u8 {
        let repair = self.repair(target);
        if repair + target.health < 9 {
            target.health += repair;
        } else {
            target.health = 9;
        }
        repair
    }
    pub fn repair(&self, target: &Self) -> Health {
        use UnitType::*;
        match self.unit_type {
            Repair => match target.unit_type {
                AI => 3,
                Hacker => 1,
                Repair => 2,
                Tank => 1,
                Drone => 1,
                Soldier => 1,
            },
            _ => 0,
        }
    }
    pub fn apply_damage(&mut self, target: &mut Self) -> u8 {
        let damage = self.damage(target);
        if damage < target.health {
            target.health -= damage;
        } else {
            target.health = 0;
        }
        damage
    }
    pub fn damage(&self, target: &Self) -> Health {
        use UnitType::*;
        match self.unit_type {
            AI => match target.unit_type {
                AI => 1,
                Hacker => 1,
                Repair => 1,
                Tank => 3,
                Drone => 3,
                Soldier => 3,
            },
            Hacker => match target.unit_type {
                AI => 4,
                Hacker => 1,
                Repair => 2,
                Tank => 1,
                Drone => 1,
                Soldier => 1,
            },
            Repair => match target.unit_type {
                AI => 0,
                Hacker => 1,
                Repair => 1,
                Tank => 0,
                Drone => 0,
                Soldier => 0,
            },
            Tank => match target.unit_type {
                AI => 1,
                Hacker => 1,
                Repair => 1,
                Tank => 2,
                Drone => 2,
                Soldier => 3,
            },
            Drone => match target.unit_type {
                AI => 1,
                Hacker => 1,
                Repair => 1,
                Tank => 6,
                Drone => 2,
                Soldier => 1,
            },
            Soldier => match target.unit_type {
                AI => 2,
                Hacker => 2,
                Repair => 1,
                Tank => 2,
                Drone => 5,
                Soldier => 2,
            },
        }
    }
}

