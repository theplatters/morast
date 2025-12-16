use bevy::{
    ecs::{bundle::Bundle, component::Component},
    math::I16Vec2,
};

use crate::{
    engine::janet_handler::types::{janetenum::JanetEnum, table::Table},
    game::{
        actions::{
            action_builder::ActionPrototypeBuilder, spell_speed::SpellSpeed,
            targeting::TargetingType, timing::ActionTiming,
        },
        board::effect::EffectType,
        card::card_id::CardID,
        error::GameError,
        janet_action::JanetAction,
    },
};

#[derive(Component, Debug, Clone)]
pub struct Counterable;

#[derive(Bundle, Debug, Clone)]
pub struct GameAction {
    pub action: ActionEffectPrototype,
    pub timing: ActionTiming,
    pub speed: SpellSpeed,
}

#[derive(Component)]
pub struct Pending;

#[derive(Component)]
pub struct NeedsTargeting;

#[derive(Component)]
pub struct ReadyToExecute;

impl TryFrom<JanetEnum> for GameAction {
    type Error = GameError;

    fn try_from(action: JanetEnum) -> Result<Self, Self::Error> {
        let JanetEnum::Table(elements) = action else {
            return Err(GameError::Cast("Action value is not a table".into()));
        };

        let Some(timing_janet) = elements.get("timing") else {
            return Err(GameError::Incomplete("Timing not found"));
        };

        let Some(action_type_table) = elements.get_table("action") else {
            return Err(GameError::Incomplete("Action type not found"));
        };

        let speed: SpellSpeed = elements
            .get("speed")
            .map(|speed| speed.try_into())
            .transpose()?
            .unwrap_or_default();

        let timing = timing_janet.try_into().expect("Timing out of bound");
        let action_type = action_type_table
            .try_into()
            .expect("ActionEffectPrototype out of bound");

        GameAction::builder()
            .with_speed(speed)
            .with_timing(timing)
            .with_action(action_type)
            .build()
            .map_err(GameError::ActionBuilderError)
    }
}

#[derive(Component, Debug, Clone)]
pub enum ActionEffectPrototype {
    // Basic game actions
    PlaceCreature,
    CastSpell,
    PlaceTrap,
    MoveCreature {
        direction: I16Vec2,
    },
    EndTurn,

    // Atomic game effects (what cards actually do)
    DealDamage {
        targeting_type: TargetingType,
        amount: u16,
    },
    HealCreature {
        targeting_type: TargetingType,
        amount: u16,
    },
    DrawCards {
        count: u16,
    },
    AddGold {
        amount: u16,
    },
    ApplyEffect {
        effect: EffectType,
        duration: u16,
        targeting_type: TargetingType,
    },
    SummonCreature {
        creature_id: CardID,
    },

    DestroyCreature {
        targeting_type: TargetingType,
    },

    // Composite actions
    Sequence(Vec<ActionEffectPrototype>),

    // Targeting actions
    Custom {
        action: Box<JanetAction>,
        targeting_type: TargetingType,
    },
}

impl ActionEffectPrototype {
    /// Returns true if this action effect requires targeting from the player
    fn requires_selection(&self) -> bool {
        match self {
            ActionEffectPrototype::DealDamage { targeting_type, .. }
            | ActionEffectPrototype::HealCreature { targeting_type, .. }
            | ActionEffectPrototype::ApplyEffect { targeting_type, .. }
            | ActionEffectPrototype::DestroyCreature { targeting_type } => {
                targeting_type.requires_selection()
            }
            ActionEffectPrototype::PlaceCreature
            | ActionEffectPrototype::PlaceTrap
            | ActionEffectPrototype::CastSpell
            | ActionEffectPrototype::MoveCreature { .. }
            | ActionEffectPrototype::DrawCards { .. }
            | ActionEffectPrototype::AddGold { .. }
            | ActionEffectPrototype::SummonCreature { .. } => false,
            ActionEffectPrototype::Sequence(actions) => {
                actions.iter().any(|action| action.requires_selection())
            }
            ActionEffectPrototype::Custom { targeting_type, .. } => {
                targeting_type.requires_selection()
            }
            ActionEffectPrototype::EndTurn => false,
        }
    }

    /// Returns the targeting type if this action requires targeting
    fn targeting_type(&self) -> TargetingType {
        match self {
            ActionEffectPrototype::DealDamage { targeting_type, .. }
            | ActionEffectPrototype::HealCreature { targeting_type, .. }
            | ActionEffectPrototype::ApplyEffect { targeting_type, .. }
            | ActionEffectPrototype::Custom { targeting_type, .. }
            | ActionEffectPrototype::DestroyCreature { targeting_type } => *targeting_type,

            ActionEffectPrototype::MoveCreature { .. } => TargetingType::SingleTile,

            // For composite actions, return the first targeting type found
            // Note: This assumes only one action in a sequence requires targeting
            // You might want to handle this differently based on your game's needs
            ActionEffectPrototype::Sequence(actions) => actions
                .iter()
                .next()
                .and_then(|a| Some(a.targeting_type()))
                .unwrap_or(TargetingType::None),

            // These cases should never be reached due to has_targeting_type() check
            _ => TargetingType::None,
        }
    }
}

impl GameAction {
    pub fn builder() -> ActionPrototypeBuilder {
        ActionPrototypeBuilder::new()
    }
}

impl TryFrom<Table> for ActionEffectPrototype {
    type Error = GameError;

    fn try_from(value: Table) -> Result<Self, Self::Error> {
        let Some(JanetEnum::String(action_type)) = value.get("type") else {
            return Err(GameError::Incomplete("Type not found"));
        };

        match action_type.as_str() {
            "apply-effect" => {
                let Some(effect_type) = value.get("effect") else {
                    return Err(GameError::Incomplete("Action Effect not found"));
                };

                let Some(effect_duration) = value.get("duration") else {
                    return Err(GameError::Incomplete("Effect duration not found"));
                };

                let Some(targeting_type) = value.get("targeting") else {
                    return Err(GameError::Incomplete("Targeting Type not found"));
                };

                Ok(Self::ApplyEffect {
                    effect: effect_type.try_into()?,
                    duration: effect_duration.try_into().map_err(GameError::EngineError)?,
                    targeting_type: targeting_type.try_into()?,
                })
            }
            "get-gold" => {
                let Some(amount) = value.get("amount") else {
                    return Err(GameError::Incomplete("Amount not found"));
                };

                Ok(Self::AddGold {
                    amount: amount.try_into().map_err(GameError::EngineError)?,
                })
            }
            "move-creature" => {
                let Some(direction) = value.get("direction") else {
                    return Err(GameError::Incomplete("Direction not found"));
                };

                Ok(Self::MoveCreature {
                    direction: direction.try_into().map_err(GameError::EngineError)?,
                })
            }
            "damage" => {
                let Some(amount) = value.get("amount") else {
                    return Err(GameError::Incomplete("Amount not found"));
                };

                let Some(targeting_type) = value.get("targeting") else {
                    return Err(GameError::Incomplete("Targeting Type not found"));
                };

                Ok(Self::DealDamage {
                    targeting_type: targeting_type.try_into()?,
                    amount: amount.try_into().map_err(GameError::EngineError)?,
                })
            }
            "destroy" => {
                let Some(targeting_type) = value.get("targeting") else {
                    return Err(GameError::Incomplete("Targeting Type not found"));
                };

                Ok(Self::DestroyCreature {
                    targeting_type: targeting_type.try_into()?,
                })
            }
            "heal" => {
                let Some(targeting_type) = value.get("targeting") else {
                    return Err(GameError::Incomplete("Targeting Type not found"));
                };

                let Some(amount) = value.get("amount") else {
                    return Err(GameError::Incomplete("Amount not found"));
                };

                Ok(Self::HealCreature {
                    amount: amount.try_into().map_err(GameError::EngineError)?,
                    targeting_type: targeting_type.try_into()?,
                })
            }

            _ => Err(GameError::Cast("TargetingType not found".into())),
        }
    }
}
