use macroquad::math::I16Vec2;

use crate::{
    engine::janet_handler::types::{janetenum::JanetEnum, tuple::Tuple},
    game::error::Error,
};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum TargetingType {
    SingleTile, // Click a tile
    Tiles { amount: u8 },
    Area { radius: u8 }, // Area around clicked tile
    Line { length: u8 }, // Line from caster
    Caster,              // Targets the card itself
    AreaAroundCaster { radius: u16 },
    AllEnemies, // All enemy units
}

impl TargetingType {
    pub fn requires_selection(&self) -> bool {
        matches!(
            self,
            Self::SingleTile | Self::Tiles { .. } | Self::Area { .. } | Self::Line { .. }
        )
    }

    pub(crate) fn required_targets(&self) -> u8 {
        if let Self::Tiles { amount } = self {
            *amount
        } else if matches!(
            self,
            Self::SingleTile | Self::Area { .. } | Self::Line { .. }
        ) {
            1
        } else {
            0
        }
    }

    pub fn verify(&self, targets: &[I16Vec2]) -> bool {
        todo!()
    }
}

impl TryFrom<String> for TargetingType {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "caster" => Ok(Self::Caster),
            "single-tile" => Ok(Self::SingleTile),
            "all-enemies" => Ok(Self::AllEnemies),
            _ => Err(Error::Cast(format!(
                "String {} is not supported as a targeting type",
                value
            ))),
        }
    }
}

impl TryFrom<Tuple> for TargetingType {
    type Error = Error;

    fn try_from(value: Tuple) -> Result<Self, Self::Error> {
        let targeting_type = value
            .get(0)
            .map_err(Error::EngineError)?
            .into_string()
            .ok_or(Error::Cast("Expected String for targeting type".into()))?;

        match targeting_type.as_str() {
            "area" => todo!(),
            "area-around-caster" => Ok(TargetingType::AreaAroundCaster {
                radius: value
                    .get(1)
                    .and_then(|v| v.try_into())
                    .map_err(Error::EngineError)?,
            }),
            "tiles" => todo!(),
            "line" => todo!(),
            _ => Err(Error::Cast(format!(
                "Targeting type {} is not implemented",
                targeting_type
            ))),
        }
    }
}

impl TryFrom<JanetEnum> for TargetingType {
    type Error = Error;

    fn try_from(value: JanetEnum) -> Result<Self, Self::Error> {
        match value {
            JanetEnum::String(s) => s.try_into(),
            JanetEnum::Tuple(t) => t.try_into(),
            _ => Err(Error::Cast(format!(
                "JanetEnum {} is not supported as a targeting type",
                value
            ))),
        }
    }
}
