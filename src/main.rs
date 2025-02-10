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
    use std::time::Instant;
    let now = Instant::now();

    let env = Environment::new();
    env.read_script("scripts/loader.janet")
        .expect("Could not find file");
    let soldier = read_card(&env, "soldier").unwrap_or_else(|er| panic!("{:?}", er));
    let bowmen = read_card(&env, "bowmen").unwrap_or_else(|er| panic!("{:?}", er));
    let tower = read_card(&env, "tower").unwrap_or_else(|er| panic!("{:?}", er));

    print!("{:?}", soldier);
    print!("{:?}", bowmen);
    print!("{:?}", tower);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
