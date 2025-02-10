use engine::janet_handler::controller::Environment;
use game::card::card_reader::read_card;
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
    let env = Environment::new();
    env.read_script("scripts/loader.janet")
        .expect("Could not find file");
    let Some(soldier) = read_card(&env, "soldier") else {
        panic!("Soldier not found");
    };

    print!("{:?}", soldier);
}
