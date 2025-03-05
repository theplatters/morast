use std::ffi::c_void;

use log::debug;
use macroquad::math::I16Vec2;

use crate::engine::janet_handler::bindings::janet_wrap_pointer;

use super::{
    events::event_scheduler::GameScheduler, game_action::GameAction, game_context::GameContext,
};
pub mod card_id;
pub mod card_reader;
pub mod card_registry;

#[derive(Debug)]
pub struct Card {
    pub name: String,
    pub movement: Vec<I16Vec2>,
    pub attack: Vec<I16Vec2>,
    pub attack_strength: u16,
    pub defense: u16,
    play_action: Vec<GameAction>,
    turn_begin_action: Vec<GameAction>,
    turn_end_action: Vec<GameAction>,
    draw_action: Vec<GameAction>,
    discard_action: Vec<GameAction>,
}

impl Card {
    pub fn on_turn_start(&self, scheduler: &mut GameScheduler) {
        debug!("Calling on_turn_start");
        //for ele in &self.turn_begin_action {
        //    scheduler.schedule_now(owner, ele.speed);
        //}
    }

    pub fn on_turn_end(&self, scheduler: &mut GameScheduler) {
        debug!("Calling on_turn_end");
        //for ele in &self.turn_begin_action {
        //    scheduler.schedule_now(owner, ele.speed);
        //}
    }

    pub fn on_place(&self, scheduler: &mut GameScheduler) {
        //for ele in &self.turn_begin_action {
        //    scheduler.schedule_now(owner, ele.speed);
        //}
    }

    pub fn get_attack_pattern(&self) -> &Vec<I16Vec2> {
        return &self.attack;
    }
}
impl crate::engine::janet_handler::types::janetenum::ToVoidPointer for Card {}
