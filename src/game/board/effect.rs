use std::str::FromStr;

use bevy::ecs::{bundle::Bundle, component::Component};

use crate::{
    engine::janet_handler::types::janetenum::JanetEnum,
    game::{error::GameError, player::Player},
};

#[derive(Component, Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub enum EffectType {
    Slow,
    Weakening,
}

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq)]
pub struct EffectDuration(pub u16);
impl EffectDuration {
    pub(crate) fn decrease(&mut self) {
        self.0.saturating_sub(1);
    }

    pub(crate) fn over(&self) -> bool {
        self.0 == 0
    }
}

#[derive(Bundle, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Effect {
    pub effect_type: EffectType,
    duration: EffectDuration,
    pub owner: Player,
}

impl Effect {
    pub fn new(effect_type: EffectType, duration: u16, owner: Player) -> Self {
        Self {
            effect_type,
            duration: EffectDuration(duration),
            owner,
        }
    }

    pub fn effect_type(&self) -> EffectType {
        self.effect_type
    }

    pub fn duration(&self) -> u16 {
        self.duration.0
    }

    pub fn decrease_duration(&mut self) {
        self.duration.0 = self.duration.0.saturating_sub(1);
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
    type Error = GameError;

    fn try_from(value: JanetEnum) -> Result<Self, Self::Error> {
        match &value {
            JanetEnum::Int(0) | JanetEnum::UInt(0) => Ok(Self::Slow),
            JanetEnum::Int(1) | JanetEnum::UInt(1) => Ok(Self::Weakening),

            JanetEnum::String(s) if s == "slow" => Ok(Self::Slow),
            JanetEnum::String(s) if s == "weakening" => Ok(Self::Weakening),

            JanetEnum::Int(_) | JanetEnum::UInt(_) | JanetEnum::String(_) => {
                Err(GameError::Cast("Effect type not impleneted".into()))
            }
            _ => Err(GameError::Cast(
                "Unsupported janet type for effect type".into(),
            )),
        }
    }
}
