use std::str::FromStr;

use bevy::ecs::component::Component;
use serde::{Deserialize, Serialize};

use crate::error::GameError;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Abilities {
    Flying,
    Jumping,
    Digging,
}

/// Component holding a card's keyword abilities.
#[derive(Component, Debug, Clone, Default, PartialEq, Eq)]
pub struct CardAbilities(pub Vec<Abilities>);

impl FromStr for Abilities {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fly" => Ok(Self::Flying),
            "jump" => Ok(Self::Jumping),
            "dig" => Ok(Self::Digging),
            _ => Err(GameError::Cast(format!("Ability not found {}", s))),
        }
    }
}
