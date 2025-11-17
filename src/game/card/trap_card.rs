use crate::game::{
    card::{Named, Placeable},
    game_action::{self, GameAction},
    Game,
};

#[derive(Debug)]
pub struct Trap {
    name: String,
    place_action: Vec<GameAction>,
    reveal_action: Vec<GameAction>,
    cost: u16,
}

impl Placeable for Trap {
    fn on_place(
        &self,
        scheduler: &mut crate::game::events::event_scheduler::GameScheduler,
        owner: crate::game::player::PlayerID,
        id: super::in_play_id::InPlayID,
    ) {
        for GameAction { function, speed } in &self.place_action {
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

    fn cost(&self) -> u16 {
        self.cost
    }
}

impl Named for Trap {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Trap {
    pub fn new(
        name: &str,
        cost: u16,
        place_action: Vec<GameAction>,
        reveal_action: Vec<GameAction>,
    ) -> Self {
        Self {
            name: name.to_owned(),
            cost,
            place_action,
            reveal_action,
        }
    }
}
