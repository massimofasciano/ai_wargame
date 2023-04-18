use ai_wargame::Game;

fn main() {
    // let drop_prob = None;
    let drop_prob = Some(0.05);
    let mut game = Game::new(drop_prob);

    loop {
        println!("{}",game);

        if let Some(winner) = game.winner() {
            println!("{} in {} moves!", if winner.is_none() {
                "draw".to_string()
            } else {
                format!("{} wins",winner.unwrap())
            }, game.total_moves());
            break;
        }

        // loop {
        //     use rand::Rng;
        //     let mut rng = rand::thread_rng();
        //     let md = game.dim();
        //     let from = (rng.gen_range(0..md), rng.gen_range(0..md));
        //     let to = (rng.gen_range(0..md), rng.gen_range(0..md));
        //     if !game.perform_action(from, to) {
        //         // println!("Invalid move!");
        //         continue;
        //     }
        //     break;
        // }

        if let Some((from,to)) = game.parse_move_stdin() {
            if !game.perform_action(from, to) {
                println!("Invalid move!");
            }
        }

    }
}