use ai_wargame::{Game, UnitType, Unit, Cell};

fn main() {
    let mut game = Game::default();

    game[(3,4)] = Cell::Unit{player: game.next_player(),unit: Unit::new(UnitType::AI)};
    game[(4,5)] = Cell::Unit{player: game.next_player(),unit: Unit::new(UnitType::Hacker)};
    game[(6,5)] = Cell::Unit{player: game.next_player(),unit: Unit::new(UnitType::Repair)};
    game[(2,5)] = Cell::Unit{player: game.next_player(),unit: Unit::new(UnitType::Tank)};
    game[(0,5)] = Cell::Unit{player: game.next_player(),unit: Unit::new(UnitType::Soldier)};
    game[(1,1)] = Cell::Unit{player: game.next_player(),unit: Unit::new(UnitType::Drone)};

    loop {
        println!("{}",game);
        if let Some((from,to)) = game.get_move_from_stdin() {
            if game.move_unit(from, to) {
                game.resolve_conflicts();
                game.remove_dead();
                game.next_player();
            } else {
                println!("Invalid move!");
            }
        }
    }
}