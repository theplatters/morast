use crate::engine::janet_handler::types::function::Function;

pub struct Card {
    name: String,
    play_action: Function,
    draw_action: Function,
    discard_action: Function,
}

impl crate::engine::janet_handler::types::janetenum::ToVoidPointer for Card {}
