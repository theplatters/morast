use crate::game::{
    actions::{action::Action, action_prototype::ActionPrototype},
    card::CardBehavior,
};

#[derive(Debug)]
pub struct Spell {
    name: String,
    cost: u16,
    description: String,
    on_play_action: ActionPrototype,
    display_image_asset_string: String,
}

impl CardBehavior for Spell {
    fn name(&self) -> &str {
        &self.name
    }
    fn cost(&self) -> u16 {
        self.cost
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn display_image_asset_string(&self) -> &str {
        &self.display_image_asset_string
    }
}

impl Spell {
    pub fn cost(&self) -> u16 {
        self.cost
    }

    pub fn new(
        name: String,
        description: String,
        cost: u16,
        on_play_action: ActionPrototype,
        display_image_asset_string: String,
    ) -> Self {
        Self {
            name: name.to_owned(),
            description,
            cost,
            on_play_action,
            display_image_asset_string,
        }
    }

    pub fn on_play_action(&self) -> &ActionPrototype {
        &self.on_play_action
    }
}
