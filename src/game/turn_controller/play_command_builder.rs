use crate::game::{
    player::PlayerID,
    turn_controller::play_command::{PlayCommand, PlayCommandEffect},
};

pub struct PlayCommandBuilder {
    effect: Option<PlayCommandEffect>,
    owner: Option<PlayerID>,
}

impl PlayCommandBuilder {
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

    pub fn with_owner(mut self, owner: PlayerID) -> Self {
        self.owner = Some(owner);
        self
    }

    pub fn build(self) -> PlayCommand {
        PlayCommand::new(self.effect.unwrap(), self.owner.unwrap())
    }
}
