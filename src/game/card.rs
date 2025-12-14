use bevy::ecs::component::Component;

use crate::game::card::creature::Creature;
use crate::game::card::spell_card::Spell;
use crate::game::card::trap_card::Trap;

pub mod abilities;
pub mod card_builder;
pub mod card_id;
pub mod card_reader;
pub mod card_registry;
pub mod card_type;
pub mod creature;
pub mod deck_builder;
pub mod in_play_id;
pub mod spell_card;
pub mod trap_card;

#[derive(Debug)]
pub enum Card {
    Creature(Creature),
    Spell(Spell),
    Trap(Trap),
}

#[derive(Component)]
pub struct CreatureCard;

#[derive(Component)]
pub struct SpellCard;

#[derive(Component)]
pub struct TrapCard;

pub trait CardBehavior {
    fn cost(&self) -> u16;
    fn description(&self) -> &str;
    fn name(&self) -> &str;
    fn display_image_asset_string(&self) -> &str;
    // Add other common methods here
}

impl Card {
    fn can_be_placed(&self) -> bool {
        matches!(self, Card::Creature(_) | Card::Trap(_))
    }
}

impl CardBehavior for Card {
    fn cost(&self) -> u16 {
        match self {
            Card::Creature(c) => c.cost(),
            Card::Spell(c) => c.cost(),
            Card::Trap(c) => c.cost(),
        }
    }

    fn description(&self) -> &str {
        match self {
            Card::Creature(c) => c.description(),
            Card::Spell(c) => c.description(),
            Card::Trap(c) => c.description(),
        }
    }
    fn name(&self) -> &str {
        match self {
            Card::Creature(c) => c.name(),
            Card::Spell(c) => c.name(),
            Card::Trap(c) => c.name(),
        }
    }

    fn display_image_asset_string(&self) -> &str {
        match self {
            Card::Creature(c) => c.display_image_asset_string(),
            Card::Spell(c) => c.display_image_asset_string(),
            Card::Trap(c) => c.display_image_asset_string(),
        }
    }
}
