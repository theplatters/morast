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
    println!("The current directory is {:#?}", path.display());

    let mut game = Game::new().await;
    game.advance_turn();
    game.advance_turn();

    println!(
        "{:#?}",
        game.context
            .get_player_gold(PlayerID::new(0))
            .expect("nlakdv")
    );
}
