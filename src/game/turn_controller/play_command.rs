use macroquad::math::I16Vec2;

use crate::game::{
    card::card_id::CardID,
    events::{action::Action, action_effect::GameAction},
};

pub enum PlayCommand {
    PlaceCreature {
        card_index: usize,
        position: I16Vec2,
    },
    CastSpell {
        card_index: usize,
        targets: Vec<I16Vec2>,
    },
    PlaceTrap {
        card_index: usize,
        position: I16Vec2,
    },
    MoveCreature {
        from: I16Vec2,
        to: I16Vec2,
    },
    EndTurn,
    ExecuteActionWithTargets {
        action: Box<dyn GameAction>,
        targets: Vec<I16Vec2>,
    },
}

impl PlayCommand {}

impl GameAction for PlayCommand {
    fn can_execute(
        &self,
        context: &crate::game::game_context::GameContext,
    ) -> Result<(), crate::game::error::Error> {
        todo!()
    }

    fn has_targeting_type(&self) -> bool {
        todo!()
    }

    fn targeting_type(&self) -> Option<crate::game::events::action_effect::TargetingType> {
        todo!()
    }

    fn execute(
        &self,
        context: &mut crate::game::game_context::GameContext,
        card_registry: &crate::game::card::card_registry::CardRegistry,
    ) -> Result<crate::game::events::action_effect::ExecutionResult, crate::game::error::Error>
    {
        todo!()
    }
}
