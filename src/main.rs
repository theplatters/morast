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

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    let start = I16Vec2::new(2, 2);
    let mut curr = start;
    loop {
        if is_key_released(KeyCode::Space) {
            game.end_turn();
        }
        if is_key_released(KeyCode::Enter) {
            game.place(CardID::new(0), start, PlayerID::new(1));
        }

        if is_key_released(KeyCode::Q) {
            game.place(CardID::new(1), I16Vec2::new(4, 2), PlayerID::new(0));
        }

        if is_key_released(KeyCode::Right) {
            game.move_card(curr, curr + I16Vec2::new(1, 0));
            curr += I16Vec2::new(1, 0);
        }

        if is_key_released(KeyCode::Left) {
            game.move_card(curr, curr - I16Vec2::new(1, 0));
            curr -= I16Vec2::new(1, 0);
        }

        if is_key_released(KeyCode::Up) {
            game.move_card(curr, curr - I16Vec2::new(0, 1));
            curr -= I16Vec2::new(0, 1);
        }

        if is_key_released(KeyCode::Down) {
            game.move_card(curr, curr + I16Vec2::new(0, 1));
            curr += I16Vec2::new(0, 1);
        }
        clear_background(RED);
        game.context.draw_board();
        next_frame().await
    }
}
