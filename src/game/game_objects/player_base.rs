use crate::game::player::PlayerID;

pub enum PlayerBaseStatus {
    Alive,
    Destroyed,
}

#[derive(Debug)]
pub struct PlayerBase {
    owner: PlayerID,
    pub health: u16,
}

impl PlayerBase {
    pub fn new(owner: PlayerID) -> Self {
        Self { owner, health: 10 }
    }

    pub fn damage(&mut self, amount: u16) -> PlayerBaseStatus {
        match self.health.checked_sub(amount) {
            Some(result) => {
                self.health = result;
                PlayerBaseStatus::Alive
            }
            None => PlayerBaseStatus::Destroyed,
        }
    }
}
