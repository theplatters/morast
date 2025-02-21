use std::ffi::c_void;

use log::debug;
use macroquad::math::I16Vec2;

use crate::engine::janet_handler::{bindings::janet_wrap_pointer, types::function::Function};

use super::{events::event_scheduler::GameScheduler, game_context::GameContext};
pub mod card_id;
pub mod card_reader;
pub mod card_registry;

#[derive(Clone, Debug)]
pub struct Card {
    pub name: String,
    pub movement: Vec<I16Vec2>,
    pub attack: Vec<I16Vec2>,
    pub attack_strength: u16,
    defense: u16,
    play_action: Function,
    turn_begin_action: Function,
    turn_end_action: Function,
    draw_action: Function,
    discard_action: Function,
}

impl Card {
    pub fn on_turn_start(&self, game: &mut GameContext, scheduler: &mut GameScheduler) {
        debug!("Calling on_turn_start");
        unsafe {
            self.turn_begin_action
                .eval::<GameContext>(&[
                    janet_wrap_pointer(game as *mut GameContext as *mut c_void),
                    janet_wrap_pointer(scheduler as *mut GameScheduler as *mut c_void),
                ])
                .unwrap_or_else(|_| panic!("Calling the function failed {:?}", self.name));
        }
    }

    pub fn on_turn_end(&self, game: &mut GameContext, scheduler: &mut GameScheduler) {
        debug!("Calling on_turn_end");
        unsafe {
            self.turn_end_action
                .eval::<GameContext>(&[
                    janet_wrap_pointer(game as *mut GameContext as *mut c_void),
                    janet_wrap_pointer(scheduler as *mut GameScheduler as *mut c_void),
                ])
                .unwrap_or_else(|_| panic!("Calling the function failed {:?}", self.name));
        }
    }

    pub fn on_place(&self, game: &mut GameContext, scheduler: &mut GameScheduler) {
        unsafe {
            self.play_action
                .eval::<GameContext>(&[
                    janet_wrap_pointer(game as *mut GameContext as *mut c_void),
                    janet_wrap_pointer(scheduler as *mut GameScheduler as *mut c_void),
                ])
                .unwrap_or_else(|_| panic!("Calling the function failed {:?}", self.name));
        }
    }

    pub fn get_attack_pattern(&self) -> &Vec<I16Vec2> {
        return &self.attack;
    }
}
impl crate::engine::janet_handler::types::janetenum::ToVoidPointer for Card {}
