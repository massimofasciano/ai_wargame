const BOARD_DIM: i8 = 8;
const BOARD_SIZE: usize = BOARD_DIM as usize*BOARD_DIM as usize;
static mut TEMP_CELL : Cell = Cell::new();

type Board = [Cell;BOARD_SIZE];
type Coord = (i8,i8);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Game {
    player: Player,
    board: Board,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            player: Default::default(),
            board: [Default::default();BOARD_SIZE],
        }
    }
}

impl Game {
    pub fn new() -> Self {
        let mut game = Self::default();
        // let init = [Repair,AI,Hacker,Hacker,AI,Repair,Soldier,Tank,Drone,Drone,Tank,Soldier];
        // let init_iter = init.iter();
        // for i in 1..7 {
        //     game[(0,i)]=Cell::Unit{player:Cell::Player,unit: init_iter.next().unwrap();
        // }
        // for i  in 1..7 {
        //     game[(1,i)]=init_iter.next().unwrap();
        // }
        game
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
    pub fn get_2_cells_mut(&mut self, cell0: Coord, cell1: Coord) -> Option<(&mut Cell, &mut Cell)> {
        if Self::is_valid_position(cell0) &&
            Self::is_valid_position(cell1) &&
            cell0 != cell1
        {
            let ref_mut_0;
            let ref_mut_1;
            unsafe {
                ref_mut_0 = &mut *(self.board.get_unchecked_mut(Self::to_index(cell0)) as *mut _);
                ref_mut_1 = &mut *(self.board.get_unchecked_mut(Self::to_index(cell1)) as *mut _);
            }
            Some((ref_mut_0, ref_mut_1))
        } else {
            None
        }
    }
    pub fn player(&self) -> Player {
        self.player
    }
    pub fn next_player(&mut self) -> Player {
        self.player = match self.player {
            Player::Blue => Player::Red,
            Player::Red => Player::Blue,
        };
        self.player
    }
    pub fn is_valid_position((row,col) : (i8, i8)) -> bool {
        row >= 0 && col >= 0 && row < BOARD_DIM && col < BOARD_DIM
    }
    pub fn get_move_from_stdin(&self) -> Option<(Coord,Coord)> {
        println!("Player {}, enter next move (1 coord per line, 4 lines)...",self.player());
        let r1 = std::io::stdin().lines().next().unwrap().unwrap().parse::<i8>();
        let c1 = std::io::stdin().lines().next().unwrap().unwrap().parse::<i8>();
        let r2 = std::io::stdin().lines().next().unwrap().unwrap().parse::<i8>();
        let c2 = std::io::stdin().lines().next().unwrap().unwrap().parse::<i8>();
        if r1.is_ok() && c1.is_ok() && r2.is_ok() && c2.is_ok() {
            Some(((r1.unwrap(),c1.unwrap()),(r2.unwrap(),c2.unwrap())))
        } else {
            None
        }
    }
    pub fn is_valid_move(&mut self, from: Coord, to: Coord) -> bool {
        Self::is_valid_position(to)
            && Self::is_valid_position(from)
            && self[to].is_empty() && self[from].is_unit() 
            && Self::neighbors(from, to) &&
            self.player() == self[from].player().unwrap()
    }
    pub fn neighbors(cell0 : Coord, cell1 : Coord) -> bool {
        Self::is_valid_position(cell0) &&
        Self::is_valid_position(cell1) && (
            ((cell1.0 - cell0.0).abs() == 1 && (cell1.1 == cell0.1)) ||
            ((cell1.1 - cell0.1).abs() == 1 && (cell1.0 == cell0.0))
        )
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
            if let Some((_, unit)) = cell.unit_mut() {
                if unit.health == 0 {
                    *cell = Cell::Empty;
                }
            }
        }
    }
    pub fn resolve_conflicts(&mut self) {
        for row in 0..BOARD_DIM {
            for col in 0..BOARD_DIM {
                if self[(row,col)].is_unit() {
                    for (rd, cd) in [(-1,0),(1,0),(0,-1),(0,1)] {
                        let row_target = row + rd;
                        let col_target = col + cd; 
                        if Self::is_valid_position((row_target,col_target)) 
                            && self[(row_target,col_target)].is_unit() 
                        {
                            let (source, target) = self.get_2_cells_mut((row,col), (row_target,col_target)).unwrap();
                            let (player_source,unit_source) = source.unit_mut().unwrap();
                            let (player_target,unit_target) = target.unit_mut().unwrap();
                            if player_source != player_target {
                                // opponents
                                unit_source.apply_damage(unit_target);
                            } else {
                                // friends
                                unit_source.apply_repair(unit_target);
                            }
                        }
                    }
                }
            }
        }
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f,"Next player: {}",self.player())?;
        write!(f,"    ")?;
        for col in 0..BOARD_DIM {
            write!(f," {:>2} ",col)?;
        }
        writeln!(f,"")?;
        for row in 0..BOARD_DIM {
            write!(f,"{:>2}: ",row)?;
            for col in 0..BOARD_DIM {
                write!(f," {}",self[(row,col)])?;
            }
            writeln!(f,"")?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Cell::*;
        write!(f, "{}", match self {
            Empty => " . ".to_string(),
            Blocked => "***".to_string(),
            Outside => "out".to_string(),
            Supplies => "sup".to_string(),
            Unit { player, unit } => format!("{}{}",player,unit),
        })
    }
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.unit_type)?;
        if self.health < 10 {
            write!(f, "{}", self.health)?;
        } else {
            write!(f, "!")?;
        }
        Ok(())
    }
}

impl std::fmt::Display for UnitType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use UnitType::*;
        write!(f, "{}", match self {
            AI => "A",
            Hacker => "H",
            Repair => "R",
            Tank => "T",
            Drone => "D",
            Soldier => "S",
        })
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Player::Blue => "b",
            Player::Red => "r",
        })
    }
}

impl std::ops::Index<Coord> for Game {
    type Output = Cell;
    fn index(&self, coord: Coord) -> & Self::Output {
        self.get_cell(coord).unwrap_or(&Cell::Outside)
    }
}

impl std::ops::IndexMut<Coord> for Game {
    fn index_mut(&mut self, coord: Coord) -> &mut Self::Output {
        self.get_cell_mut(coord).unwrap_or(unsafe{&mut TEMP_CELL})
    }
}

#[derive(Debug, PartialEq, PartialOrd, Default, Clone, Copy)]
pub enum Player {
    #[default]
    Blue,
    Red,
}

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub enum Cell {
    #[default]
    Empty,
    Blocked,
    Outside,
    Supplies,
    Unit { player:Player, unit:Unit },
}

impl Cell {
    pub const fn new() -> Self {
        Self::Empty
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
}

type Health = u8;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Unit {
    unit_type : UnitType,
    health : Health,
}

#[derive(Debug, PartialEq, PartialOrd,Default, Clone, Copy)]
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
    pub fn apply_repair(&mut self, target: &mut Self) {
        let repair = self.repair(target);
        if repair + target.health < 9 {
            target.health += repair;
        } else {
            target.health = 9;
        }
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
    pub fn apply_damage(&mut self, target: &mut Self) {
        let damage = self.damage(target);
        if damage < target.health {
            target.health -= damage;
        } else {
            target.health = 0;
        }
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

