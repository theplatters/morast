use crate::engine::janet_handler::types::function::Function;

#[derive(Clone)]
pub struct Card {
    name: String,
    play_action: Function,
    draw_action: Function,
    discard_action: Function,
}

unsafe impl Send for Card {}
impl crate::engine::janet_handler::types::janetenum::ToVoidPointer for Card {}
