use game::{card::card_id::CardID, player::PlayerID, Game};
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
    game.place(CardID::new(0), I16Vec2::new(2, 2), PlayerID::new(1));
    game.place(CardID::new(1), I16Vec2::new(4, 2), PlayerID::new(0));
    game.end_turn();

    debug!(
        "{:#?}",
        game.context
            .get_player_gold(PlayerID::new(0))
            .expect("nlakdv")
    );

    let elapsed = now.elapsed();
    println!("ElapsedXD: {:.2?}", elapsed);
    loop {
        clear_background(RED);
        game.context.draw_board();
        next_frame().await
    }
}
