use macroquad::math::I16Vec2;

use crate::game::{
    card::{abilities::Abilities, in_play_id::InPlayID, CardBehavior, Placeable},
    events::event_scheduler::GameScheduler,
    game_action::{self, JanetAction},
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
    pub play_action: Vec<JanetAction>,
    pub turn_begin_action: Vec<JanetAction>,
    pub turn_end_action: Vec<JanetAction>,
    pub draw_action: Vec<JanetAction>,
    pub discard_action: Vec<JanetAction>,
    pub abilities: Vec<Abilities>,
    pub description: String,
    pub display_image_asset_string: String,
}

impl CardBehavior for Creature {
    fn name(&self) -> &str {
        &self.name
    }

    fn cost(&self) -> u16 {
        self.cost
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn display_image_asset_string(&self) -> &str {
        &self.display_image_asset_string
    }
}

impl Creature {
    pub fn new(
        name: String,
        movement: Vec<I16Vec2>,
        movement_points: u16,
        attack: Vec<I16Vec2>,
        attack_strength: u16,
        defense: u16,
        cost: u16,
        play_action: Vec<JanetAction>,
        turn_begin_action: Vec<JanetAction>,
        turn_end_action: Vec<JanetAction>,
        draw_action: Vec<JanetAction>,
        discard_action: Vec<JanetAction>,
        abilities: Vec<Abilities>,
        description: String,
        display_image_asset_string: String,
    ) -> Self {
        Self {
            name,
            movement,
            movement_points,
            attack,
            attack_strength,
            defense,
            cost,
            play_action,
            turn_begin_action,
            turn_end_action,
            draw_action,
            discard_action,
            abilities,
            description,
            display_image_asset_string,
        }
    }

    pub fn on_play(&self, scheduler: &mut GameScheduler, owner: PlayerID, id: InPlayID) {
        self.schedule_actions(scheduler, owner, id, &self.play_action);
    }

    pub fn on_turn_start(&self, scheduler: &mut GameScheduler, owner: PlayerID, id: InPlayID) {
        self.schedule_actions(scheduler, owner, id, &self.turn_begin_action);
    }

    pub fn on_turn_end(&self, scheduler: &mut GameScheduler, owner: PlayerID, id: InPlayID) {
        self.schedule_actions(scheduler, owner, id, &self.turn_end_action);
    }

    // Helper method to reduce code duplication
    fn schedule_actions(
        &self,
        scheduler: &mut GameScheduler,
        owner: PlayerID,
        id: InPlayID,
        actions: &[JanetAction],
    ) {
        for JanetAction {
            function,
            speed,
            targeting,
        } in actions
        {
            match speed {
                game_action::Timing::Now => {
                    println!("Scheduling event now");
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

    // Only keep getters for computed properties or when you need different return types
    pub fn total_stats(&self) -> u16 {
        self.attack_strength + self.defense
    }

    pub fn is_powerful(&self) -> bool {
        self.attack_strength > 5 || self.defense > 5
    }
}
