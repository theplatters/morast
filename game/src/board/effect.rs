use std::str::FromStr;

use bevy::ecs::{bundle::Bundle, component::Component, entity::Entity};

use crate::{board::tile::EffectsOnTile, player::Player};
use serde::{Deserialize, Serialize};

#[derive(Component, Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Serialize, Deserialize)]
pub enum EffectType {
    Slow,
    Weakening,
}

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq)]
pub struct EffectDuration(pub u16);
impl EffectDuration {
    pub(crate) fn decrease(&mut self) {
        self.0 = self.0.saturating_sub(1);
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

#[derive(Component, Debug)]
#[relationship(relationship_target = EffectsOnTile)]
pub struct EffectTile {
    #[relationship]
    pub position: Entity,
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


