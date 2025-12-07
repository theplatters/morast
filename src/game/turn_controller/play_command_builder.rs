use crate::game::{
    player::PlayerID,
    turn_controller::play_command::{PlayCommand, PlayCommandEffect},
};

pub struct PlayCommandBuilder<'a> {
    effect: Option<PlayCommandEffect<'a>>,
    owner: Option<PlayerID>,
}

impl<'a> PlayCommandBuilder<'a> {
    pub fn new() -> Self {
        Self {
            effect: None,
            owner: None,
        }
    }

    pub fn with_effect(mut self, effect: PlayCommandEffect<'a>) -> Self {
        self.effect = Some(effect);
        self
    }

    pub fn with_owner(mut self, owner: PlayerID) -> Self {
        self.owner = Some(owner);
        self
    }

    pub fn build(self) -> PlayCommand<'a> {
        PlayCommand::new(self.effect.unwrap(), self.owner.unwrap())
    }
}
