use crate::game::card::Placeable;
use crate::game::{card::CardBehavior, events::action::Action};

#[derive(Debug)]
pub struct Trap {
    name: String,
    description: String,
    on_play_action: Option<Action>,
    reveal_action: Option<Action>,
    cost: u16,
    display_image_asset_string: String,
}

impl CardBehavior for Trap {
    fn cost(&self) -> u16 {
        self.cost
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn display_image_asset_string(&self) -> &str {
        &self.display_image_asset_string
    }
}

impl Trap {
    pub fn new(
        name: String,
        cost: u16,
        description: String,
        place_action: Option<Action>,
        reveal_action: Option<Action>,
        display_image_asset_string: String,
    ) -> Self {
        Self {
            name,
            description,
            cost,
            on_play_action: place_action,
            reveal_action,
            display_image_asset_string,
        }
    }
}
