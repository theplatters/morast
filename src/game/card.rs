use std::ffi::c_void;

use macroquad::math::{I16Vec2, U16Vec2};

use crate::engine::janet_handler::{bindings::janet_wrap_pointer, types::function::Function};

use super::{events::event_scheduler::GameScheduler, game_context::GameContext, Game};
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
    turn_begin_action: Function,
    turn_end_action: Function,
    draw_action: Function,
    discard_action: Function,
}

impl Card {
    pub fn on_turn_start(&self, game: &mut GameContext, scheduler: &mut GameScheduler) {
        println!("Calling on_turn_start");
        unsafe {
            self.turn_begin_action
                .eval::<GameContext>(&[
                    janet_wrap_pointer(game as *mut GameContext as *mut c_void),
                    janet_wrap_pointer(scheduler as *mut GameScheduler as *mut c_void),
                ])
                .expect("Calling the function failed");
        }
    }
}
impl crate::engine::janet_handler::types::janetenum::ToVoidPointer for Card {}
