use macroquad::math::I16Vec2;

use crate::game::{
    card::card_registry::CardRegistry, error::Error, events::event_scheduler::GameScheduler,
    game_context::GameContext, player::PlayerID,
};

#[derive(Debug)]
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
}

impl PlayCommand {
    pub fn execute(
        &self,
        context: &mut GameContext,
        player_id: PlayerID,
        card_registry: &CardRegistry,
        scheduler: &mut GameScheduler,
    ) -> Result<(), Error> {
        match self {
            PlayCommand::PlaceCreature {
                card_index,
                position,
            } => context.execute_creature_placement(
                player_id,
                *card_index,
                *position,
                card_registry,
                scheduler,
            ),
            PlayCommand::CastSpell {
                card_index,
                targets,
            } => context.execute_spell_cast(
                player_id,
                *card_index,
                targets,
                card_registry,
                scheduler,
            ),
            PlayCommand::PlaceTrap {
                card_index,
                position,
            } => context.execute_trap_placement(
                player_id,
                *card_index,
                *position,
                card_registry,
                scheduler,
            ),
            PlayCommand::MoveCreature { from, to } => context.move_card(from, to, card_registry),
            PlayCommand::EndTurn => context.execute_end_turn(scheduler, card_registry),
        }
    }
}
