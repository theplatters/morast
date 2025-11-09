use game::Game;

mod config;
use config::window_config;
mod engine;
mod game;

#[macroquad::main(window_config)]
async fn main() {
    let mut game = Game::new().await;

    game.main_loop().await;
}
