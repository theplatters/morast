use std::str::FromStr;

use crate::game::error::Error;

#[derive(Debug)]
pub enum Abilities {
    Flying,
    Jumping,
    Digging,
}

impl FromStr for Abilities {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fly" => Ok(Self::Flying),
            "jump" => Ok(Self::Jumping),
            "dig" => Ok(Self::Digging),
            _ => Err(Error::Cast(format!("Ability not found {}", s))),
        }
    }
}
