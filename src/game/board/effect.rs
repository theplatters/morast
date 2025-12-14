use std::str::FromStr;

use bevy::ecs::component::Component;

use crate::{
    engine::janet_handler::types::janetenum::JanetEnum,
    game::{components::player_components::Player, error::Error, player::PlayerID},
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub enum EffectType {
    Slow,
    Weakening,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub struct Effect {
    pub effect_type: EffectType,
    duration: u16,
    pub owner: Player,
}

impl Effect {
    pub fn new(effect_type: EffectType, duration: u16, owner: Player) -> Self {
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
        self.duration = self.duration.saturating_sub(1);
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

impl TryFrom<JanetEnum> for EffectType {
    type Error = Error;

    fn try_from(value: JanetEnum) -> Result<Self, Self::Error> {
        match &value {
            JanetEnum::Int(0) | JanetEnum::UInt(0) => Ok(Self::Slow),
            JanetEnum::Int(1) | JanetEnum::UInt(1) => Ok(Self::Weakening),

            JanetEnum::String(s) if s == "slow" => Ok(Self::Slow),
            JanetEnum::String(s) if s == "weakening" => Ok(Self::Weakening),

            JanetEnum::Int(_) | JanetEnum::UInt(_) | JanetEnum::String(_) => {
                Err(Error::Cast("Effect type not impleneted".into()))
            }
            _ => Err(Error::Cast("Unsupported janet type for effect type".into())),
        }
    }
}
