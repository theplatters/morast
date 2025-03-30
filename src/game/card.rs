use abilities::Abilities;
use in_play_id::InPlayID;
use macroquad::math::I16Vec2;

use super::{events::event_scheduler::GameScheduler, game_action::GameAction, player::PlayerID};
pub mod abilities;
pub mod card_id;
pub mod card_reader;
pub mod card_registry;
pub mod in_play_id;

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
    abilities: Vec<Abilities>,
}

impl Card {
    pub fn on_turn_start(&self, scheduler: &mut GameScheduler, owner: PlayerID, id: InPlayID) {
        for GameAction { function, speed } in &self.turn_begin_action {
            match speed {
                super::game_action::Timing::Now => {
                    scheduler.schedule_now(owner, id, function.to_owned(), 1)
                }
                super::game_action::Timing::End(timing) => {
                    scheduler.schedule_at_end(*timing, owner, id, function.to_owned(), 1)
                }
                super::game_action::Timing::Start(timing) => {
                    scheduler.schedule_at_start(*timing, owner, id, function.to_owned(), 1)
                }
            }
        }
    }

    pub fn on_turn_end(&self, scheduler: &mut GameScheduler, owner: PlayerID, id: InPlayID) {
        for GameAction { function, speed } in &self.turn_end_action {
            match speed {
                super::game_action::Timing::Now => {
                    scheduler.schedule_now(owner, id, function.to_owned(), 1)
                }
                super::game_action::Timing::End(timing) => {
                    scheduler.schedule_at_end(*timing, owner, id, function.to_owned(), 1)
                }
                super::game_action::Timing::Start(timing) => {
                    scheduler.schedule_at_start(*timing, owner, id, function.to_owned(), 1)
                }
            }
        }
    }

    pub fn on_place(&self, scheduler: &mut GameScheduler, owner: PlayerID, id: InPlayID) {
        for GameAction { function, speed } in &self.play_action {
            match speed {
                super::game_action::Timing::Now => {
                    scheduler.schedule_now(owner, id, function.to_owned(), 1)
                }
                super::game_action::Timing::End(timing) => {
                    scheduler.schedule_at_end(*timing, owner, id, function.to_owned(), 1)
                }
                super::game_action::Timing::Start(timing) => {
                    scheduler.schedule_at_start(*timing, owner, id, function.to_owned(), 1)
                }
            }
        }
    }

    pub fn get_attack_pattern(&self) -> &Vec<I16Vec2> {
        return &self.attack;
    }
}
impl crate::engine::janet_handler::types::janetenum::ToVoidPointer for Card {}
