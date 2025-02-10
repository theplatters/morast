use crate::engine::janet_handler::{controller::Environment, types::function::Function};

use super::Card;

pub fn read_card(env: &Environment, name: &str) -> Option<Card> {
    let draw_action = Function::get_method(env, "on-draw", Some(name))?;
    let play_action = Function::get_method(env, "on-play", Some(name))?;
    let discard_action = Function::get_method(env, "on-discard", Some(name))?;
    Some(Card {
        name: name.to_string(),
        draw_action,
        play_action,
        discard_action,
    })
}
