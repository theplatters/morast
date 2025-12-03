use macroquad::math::I16Vec2;

use crate::game::{
    events::action::Action, player::PlayerID,
    turn_controller::play_command_builder::PlayCommandBuilder,
};

pub struct PlayCommand {
    effect: PlayCommandEffect,
    owner: PlayerID,
}

impl PlayCommand {
    fn builder() -> PlayCommandBuilder {
        PlayCommandBuilder::new()
    }

    pub(crate) fn new(effect: PlayCommandEffect, owner: PlayerID) -> Self {
        Self { effect, owner }
    }
}

pub enum PlayCommandEffect {
    PlaceCreature {
        card_index: usize,
        position: I16Vec2,
    },
    CastSpell {
        card_index: usize,
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

impl From<PlayCommand> for Action {
    fn from(value: PlayCommand) -> Self {
        let mut action_builder = Action::builder();
        let player_id = value.owner;
        action_builder = match value.effect {
            PlayCommandEffect::PlaceCreature {
                card_index,
                position,
            } => action_builder.place_creature(card_index, position, player_id),
            PlayCommandEffect::CastSpell { card_index } => {
                action_builder.cast_spell(card_index, player_id)
            }
            PlayCommandEffect::PlaceTrap {
                card_index,
                position,
            } => action_builder.place_trap(card_index, position, player_id),
            PlayCommandEffect::MoveCreature { from, to } => {
                action_builder.move_creature(from, to, player_id)
            }
            PlayCommandEffect::EndTurn => action_builder.end_turn(),
        };
        action_builder
            .play_command_speed()
            .build()
            .expect("Could not build place creature action")
    }
}
