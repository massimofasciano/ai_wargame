use ai_wargame::{Game, UnitType, Unit, Cell};

fn main() {
    let mut game = Game::default();

    game[(3,4)] = Cell::Unit{player: game.player(),unit: Unit::new(UnitType::AI)};

    println!("{}",game);
}