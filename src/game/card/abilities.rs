use std::str::FromStr;

use crate::game::error::GameError;

#[derive(Debug)]
pub enum Abilities {
    Flying,
    Jumping,
    Digging,
}

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
