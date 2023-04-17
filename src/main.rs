use ai_wargame::{Game, UnitType, Unit, Cell};

fn main() {
    let mut game = Game::default();

    game[(3,4)] = Cell::Unit{player: game.player(),unit: Unit::new(UnitType::AI)};

    loop {
        println!("{}",game);
        if let Some((from,to)) = game.get_move_from_stdin() {
            if game.move_unit(from, to) {
                game.next_player();
            } else {
                println!("Invalid move!");
            }
        }
    }
}