use macroquad::math::I16Vec2;

use crate::game::{
    actions::{
        action::Action, action_builder::ActionBuilder, action_context::ActionContext,
        action_prototype::ActionPrototype,
    },
    error::Error,
    player::PlayerID,
    turn_controller::play_command_builder::PlayCommandBuilder,
};

pub struct PlayCommand<'a> {
    effect: PlayCommandEffect<'a>,
    owner: PlayerID,
}

impl<'a> PlayCommand<'a> {
    fn builder() -> PlayCommandBuilder<'a> {
        PlayCommandBuilder::new()
    }

    pub(crate) fn new(effect: PlayCommandEffect<'a>, owner: PlayerID) -> Self {
        Self { effect, owner }
    }
}

pub enum PlayCommandEffect<'a> {
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
    BuildAction {
        action: Box<ActionPrototype>,
        action_context: &'a ActionContext,
    },
}

impl<'a> TryFrom<PlayCommand<'a>> for Action {
    type Error = Error;
    fn try_from(value: PlayCommand) -> Result<Self, Self::Error> {
        let base_builder = Action::builder()
            .play_command_speed()
            .with_player(value.owner);

        let build = |builder: ActionBuilder| -> Result<Action, Error> {
            builder.build().map_err(Error::ActionBuilderError)
        };

        match value.effect {
            PlayCommandEffect::PlaceCreature {
                card_index,
                position,
            } => build(base_builder.place_creature(card_index, position)),
            PlayCommandEffect::CastSpell { card_index } => {
                build(base_builder.cast_spell(card_index))
            }
            PlayCommandEffect::PlaceTrap {
                card_index,
                position,
            } => build(base_builder.place_trap(card_index, position)),
            PlayCommandEffect::MoveCreature { from, to } => {
                build(base_builder.move_creature(from, to))
            }
            PlayCommandEffect::EndTurn => build(base_builder.end_turn()),
            PlayCommandEffect::BuildAction {
                action,
                action_context,
            } => Action::from_prototype(*action, action_context).map_err(Error::ActionBuilderError),
        }
    }
}
