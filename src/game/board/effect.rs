use std::str::FromStr;

use crate::game::player::PlayerID;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub enum EffectType {
    Slow,
    Weakening,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub struct Effect {
    effect_type: EffectType,
    duration: u16,
    owner: PlayerID,
}

impl Effect {
    pub fn new(effect_type: EffectType, duration: u16, owner: PlayerID) -> Self {
        Self {
            effect_type,
            duration,
            owner,
        }
    }

    pub fn effect_type(&self) -> EffectType {
        self.effect_type
    }

    pub fn duration(&self) -> u16 {
        self.duration
    }

    pub fn decrease_duration(&mut self) {
        self.duration -= 1;
    }
}

impl FromStr for EffectType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "slow" => Ok(Self::Slow),
            "weakening" => Ok(Self::Weakening),
            _ => Err(()),
        }
    }
}
