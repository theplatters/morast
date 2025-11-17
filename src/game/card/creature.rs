use macroquad::math::I16Vec2;

use crate::game::{
    card::{abilities::Abilities, in_play_id::InPlayID, Named, Placeable},
    events::event_scheduler::GameScheduler,
    game_action::{self, GameAction},
    player::PlayerID,
};

#[derive(Debug)]
pub struct Creature {
    pub name: String,
    pub movement: Vec<I16Vec2>,
    pub movement_points: u16,
    pub attack: Vec<I16Vec2>,
    pub attack_strength: u16,
    pub defense: u16,
    pub cost: u16,
    pub play_action: Vec<GameAction>,
    pub turn_begin_action: Vec<GameAction>,
    pub turn_end_action: Vec<GameAction>,
    pub draw_action: Vec<GameAction>,
    pub discard_action: Vec<GameAction>,
    pub abilities: Vec<Abilities>,
    pub description: String,
}

impl Named for Creature {
    fn name(&self) -> &str {
        &self.name
    }
}
impl Placeable for Creature {
    fn on_place(&self, scheduler: &mut GameScheduler, owner: PlayerID, id: InPlayID) {
        for GameAction { function, speed } in &self.play_action {
            match speed {
                game_action::Timing::Now => {
                    scheduler.schedule_now(owner, id, function.to_owned(), 1)
                }
                game_action::Timing::End(timing) => {
                    scheduler.schedule_at_end(*timing, owner, id, function.to_owned(), 1)
                }
                game_action::Timing::Start(timing) => {
                    scheduler.schedule_at_start(*timing, owner, id, function.to_owned(), 1)
                }
            }
        }
    }
}

impl Creature {
    pub fn on_turn_start(&self, scheduler: &mut GameScheduler, owner: PlayerID, id: InPlayID) {
        for GameAction { function, speed } in &self.turn_begin_action {
            match speed {
                game_action::Timing::Now => {
                    scheduler.schedule_now(owner, id, function.to_owned(), 1)
                }
                game_action::Timing::End(timing) => {
                    scheduler.schedule_at_end(*timing, owner, id, function.to_owned(), 1)
                }
                game_action::Timing::Start(timing) => {
                    scheduler.schedule_at_start(*timing, owner, id, function.to_owned(), 1)
                }
            }
        }
    }

    pub fn on_turn_end(&self, scheduler: &mut GameScheduler, owner: PlayerID, id: InPlayID) {
        for GameAction { function, speed } in &self.turn_end_action {
            match speed {
                game_action::Timing::Now => {
                    scheduler.schedule_now(owner, id, function.to_owned(), 1)
                }
                game_action::Timing::End(timing) => {
                    scheduler.schedule_at_end(*timing, owner, id, function.to_owned(), 1)
                }
                game_action::Timing::Start(timing) => {
                    scheduler.schedule_at_start(*timing, owner, id, function.to_owned(), 1)
                }
            }
        }
    }

    pub fn get_attack_pattern(&self) -> &Vec<I16Vec2> {
        &self.attack
    }

    pub(crate) fn get_movement_pattern(&self) -> &Vec<I16Vec2> {
        &self.movement
    }
}
