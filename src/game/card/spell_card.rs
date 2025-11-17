use crate::game::{
    card::{in_play_id::InPlayID, Named},
    events::event_scheduler::GameScheduler,
    game_action::{self, GameAction},
    player::PlayerID,
};

#[derive(Debug)]
pub struct Spell {
    name: String,
    cost: u16,
    on_play_action: Vec<GameAction>,
}

impl Named for Spell {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Spell {
    pub fn cost(&self) -> u16 {
        self.cost
    }

    pub fn new(name: &str, cost: u16, on_play_action: Vec<GameAction>) -> Self {
        Self {
            name: name.to_owned(),
            cost,
            on_play_action,
        }
    }

    fn on_play(&self, scheduler: &mut GameScheduler, owner: PlayerID, id: InPlayID) {
        for GameAction { function, speed } in &self.on_play_action {
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
