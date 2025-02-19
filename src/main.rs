use game::{player::PlayerID, Game};
use macroquad::prelude::*;

mod engine;
mod game;

fn window_config() -> Conf {
    Conf {
        window_title: "Honeycomb Pattern".to_owned(),
        window_width: 800,
        window_height: 800,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let path = std::env::current_dir().unwrap();
    debug!("The current directory is {:#?}", path.display());

    use std::time::Instant;
    let now = Instant::now();
    let mut game = Game::new().await;
    for i in 0..100 {
        game.end_turn();
    }

    debug!(
        "{:#?}",
        game.context
            .get_player_gold(PlayerID::new(0))
            .expect("nlakdv")
    );

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
