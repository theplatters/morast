use crate::game::{
    actions::{action_context::ActionContext, action_prototype::ActionPrototype},
    player::PlayerID,
    turn_controller::play_command::{PlayCommand, PlayCommandEffect},
};

pub struct PlayCommandBuilder {
    effect: Option<PlayCommandEffect>,
    owner: Option<PlayerID>,
}

impl<'a> PlayCommandBuilder {
    pub fn new() -> Self {
        Self {
            effect: None,
            owner: None,
        }
    }

    pub fn with_effect(mut self, effect: PlayCommandEffect) -> Self {
        self.effect = Some(effect);
        self
    }

    pub fn build_action(mut self, action: ActionPrototype, action_context: ActionContext) -> Self {
        self.effect = Some(PlayCommandEffect::BuildAction {
            action,
            action_context,
        });
        self
    }

    pub fn with_owner(mut self, owner: PlayerID) -> Self {
        self.owner = Some(owner);
        self
    }

    pub fn build(self) -> PlayCommand {
        PlayCommand::new(self.effect.unwrap(), self.owner.unwrap())
    }
}
