use macroquad::math::{I16Vec2, U16Vec2};

use crate::game::{
    card::{in_play_id::InPlayID, CardBehavior},
    error::Error,
    events::event_scheduler::GameScheduler,
    game_action::{self, GameAction, TargetingType},
    player::PlayerID,
};

#[derive(Debug)]
pub struct Spell {
    name: String,
    cost: u16,
    description: String,
    on_play_action: Vec<GameAction>,
    display_image_asset_string: String,
}

impl CardBehavior for Spell {
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

impl Spell {
    pub fn cost(&self) -> u16 {
        self.cost
    }

    pub fn new(
        name: String,
        description: String,
        cost: u16,
        on_play_action: Vec<GameAction>,
        display_image_asset_string: String,
    ) -> Self {
        Self {
            name: name.to_owned(),
            description,
            cost,
            on_play_action,
            display_image_asset_string,
        }
    }

    pub fn on_play(
        &self,
        scheduler: &mut GameScheduler,
        owner: PlayerID,
        id: InPlayID,
        targets: Vec<I16Vec2>,
    ) {
        for GameAction {
            function,
            speed,
            targeting: _,
        } in &self.on_play_action
        {
            match speed {
                game_action::Timing::Now => scheduler.schedule_now_with_targets(
                    owner,
                    id,
                    function.to_owned(),
                    1,
                    targets.clone(),
                ),
                game_action::Timing::End(timing) => scheduler.schedule_at_end_with_targets(
                    *timing,
                    owner,
                    id,
                    function.to_owned(),
                    1,
                    targets.clone(),
                ),
                game_action::Timing::Start(timing) => scheduler.schedule_at_start_with_targets(
                    *timing,
                    owner,
                    id,
                    function.to_owned(),
                    1,
                    targets.clone(),
                ),
            }
        }
    }

    pub(crate) fn get_targeting_type(&self) -> TargetingType {
        // Return the targeting type of the primary action
        self.on_play_action
            .first()
            .map(|action| action.targeting.clone())
            .unwrap_or(TargetingType::None)
    }
}
