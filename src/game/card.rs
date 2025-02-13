use macroquad::math::{I16Vec2, U16Vec2};

use crate::engine::janet_handler::types::function::Function;
pub mod card_id;
pub mod card_reader;
pub mod card_registry;

#[derive(Clone, Debug)]
pub struct Card {
    name: String,
    movement: Vec<I16Vec2>,
    attack: Vec<I16Vec2>,
    attack_strength: u16,
    defense: u16,
    play_action: Function,
    draw_action: Function,
    discard_action: Function,
}

unsafe impl Send for Card {}
impl crate::engine::janet_handler::types::janetenum::ToVoidPointer for Card {}
