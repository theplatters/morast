use macroquad::math::I16Vec2;

use crate::game::{card::in_play_id::InPlayID, player::PlayerID};

#[derive(Debug, Clone)]
pub struct ActionContext {
    pub source: Option<InPlayID>, // Which in-play card/creature caused this action
    pub position: Option<I16Vec2>, // Position on the board (for placing/summoning)
    pub player_id: Option<PlayerID>, // Owning player
    pub targets: Option<Vec<I16Vec2>>, // Explicit target tiles/positions
    pub card_index: Option<usize>, // Index of the card in hand/deck
    pub caster_position: Option<I16Vec2>,
    pub priority: u32,
    // You can extend with other runtime info as needed
}

impl ActionContext {
    pub fn new() -> Self {
        Self {
            source: None,
            position: None,
            player_id: None,
            targets: None,
            card_index: None,
            priority: 0,
            caster_position: None,
        }
    }

    pub fn with_caster_position(mut self, pos: I16Vec2) -> Self {
        self.caster_position = Some(pos);
        self
    }

    pub fn with_source(mut self, source: InPlayID) -> Self {
        self.source = Some(source);
        self
    }

    pub fn with_position(mut self, pos: I16Vec2) -> Self {
        self.position = Some(pos);
        self
    }

    pub fn with_player(mut self, player: PlayerID) -> Self {
        self.player_id = Some(player);
        self
    }

    pub fn with_targets(mut self, targets: Vec<I16Vec2>) -> Self {
        self.targets = Some(targets);
        self
    }

    pub fn with_index(mut self, index: usize) -> Self {
        self.card_index = Some(index);
        self
    }

    pub fn with_priority(mut self, priority: u32) -> Self {
        self.priority = priority;
        self
    }
}
