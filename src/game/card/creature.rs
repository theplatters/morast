use macroquad::math::I16Vec2;

use crate::game::{
    actions::action_prototype::GameAction,
    card::{abilities::Abilities, CardBehavior},
};

#[derive(Debug)]
pub struct Creature {
    pub name: String,
    pub movement: Vec<I16Vec2>,
    pub movement_points: u16,
    pub attack: Vec<I16Vec2>,
    pub attack_strength: u16,
    pub defense: u16,
    pub cost: u16,
    on_play_action: Option<GameAction>,
    pub turn_begin_action: Option<GameAction>,
    pub turn_end_action: Option<GameAction>,
    pub draw_action: Option<GameAction>,
    pub discard_action: Option<GameAction>,
    pub abilities: Vec<Abilities>,
    pub description: String,
    pub display_image_asset_string: String,
}

impl CardBehavior for Creature {
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

impl Creature {
    pub fn new(
        name: String,
        movement: Vec<I16Vec2>,
        movement_points: u16,
        attack: Vec<I16Vec2>,
        attack_strength: u16,
        defense: u16,
        cost: u16,
        play_action: Option<GameAction>,
        turn_begin_action: Option<GameAction>,
        turn_end_action: Option<GameAction>,
        draw_action: Option<GameAction>,
        discard_action: Option<GameAction>,
        abilities: Vec<Abilities>,
        description: String,
        display_image_asset_string: String,
    ) -> Self {
        Self {
            name,
            movement,
            movement_points,
            attack,
            attack_strength,
            defense,
            cost,
            on_play_action: play_action,
            turn_begin_action,
            turn_end_action,
            draw_action,
            discard_action,
            abilities,
            description,
            display_image_asset_string,
        }
    }

    // Helper method to reduce code duplication

    // Only keep getters for computed properties or when you need different return types
    pub fn total_stats(&self) -> u16 {
        self.attack_strength + self.defense
    }

    pub fn is_powerful(&self) -> bool {
        self.attack_strength > 5 || self.defense > 5
    }

    pub fn on_play_action(&self) -> Option<&GameAction> {
        self.on_play_action.as_ref()
    }
}
