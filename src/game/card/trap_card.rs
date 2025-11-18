use crate::game::{
    card::{CardBehavior, Placeable},
    game_action::{self, GameAction},
};

#[derive(Debug)]
pub struct Trap {
    name: String,
    description: String,
    place_action: Vec<GameAction>,
    reveal_action: Vec<GameAction>,
    cost: u16,
    display_image_asset_string: String,
}

impl CardBehavior for Trap {
    fn cost(&self) -> u16 {
        self.cost
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn display_image_asset_string(&self) -> &str {
        &self.display_image_asset_string
    }
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
}

impl Trap {
    pub fn new(
        name: String,
        cost: u16,
        description: String,
        place_action: Vec<GameAction>,
        reveal_action: Vec<GameAction>,
        display_image_asset_string: String,
    ) -> Self {
        Self {
            name,
            description,
            cost,
            place_action,
            reveal_action,
            display_image_asset_string,
        }
    }
}
