use macroquad::math::I16Vec2;

use crate::{
    engine::janet_handler::types::{janetenum::JanetEnum, table::Table},
    game::{
        actions::{
            action::{Action, SpellSpeed},
            action_builder::{ActionBuilderError, ActionPrototypeBuilder},
            action_context::ActionContext,
            action_effect::Condition,
            targeting::TargetingType,
            timing::ActionTiming,
        },
        board::effect::EffectType,
        card::card_id::CardID,
        error::Error,
        janet_action::JanetAction,
    },
};

#[derive(Debug, Clone)]
pub struct ActionPrototype {
    pub action: ActionEffectPrototype,
    pub timing: ActionTiming,
    pub speed: SpellSpeed,
    pub can_be_countered: bool,
    pub optional: bool,
}

impl ActionPrototype {
    pub fn finalize(self, action_context: &ActionContext) -> Result<Action, ActionBuilderError> {
        Action::from_prototype(self, action_context)
    }

    pub fn requires_selection(&self) -> bool {
        self.action.requires_selection()
    }

    pub fn targeting_type(&self) -> TargetingType {
        self.action.targeting_type()
    }
}

impl TryFrom<JanetEnum> for ActionPrototype {
    type Error = Error;

    fn try_from(action: JanetEnum) -> Result<Self, Self::Error> {
        let JanetEnum::Table(elements) = action else {
            return Err(Error::Cast("Action value is not a table".into()));
        };

        let Some(timing_janet) = elements.get("timing") else {
            return Err(Error::Incomplete("Timing not found"));
        };

        let Some(action_type_table) = elements.get_table("action") else {
            return Err(Error::Incomplete("Action type not found"));
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

        ActionPrototype::builder()
            .with_speed(speed)
            .with_timing(timing)
            .with_action(action_type)
            .build()
            .map_err(Error::ActionBuilderError)
    }
}

#[derive(Debug, Clone)]
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

    Conditional {
        condition: Condition,
        then_action: Box<ActionEffectPrototype>,
        else_action: Option<Box<ActionEffectPrototype>>,
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
            ActionEffectPrototype::Conditional {
                then_action,
                else_action,
                ..
            } => {
                then_action.requires_selection()
                    || else_action
                        .as_ref()
                        .is_some_and(|action| action.requires_selection())
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
            ActionEffectPrototype::Conditional {
                then_action,
                else_action,
                ..
            } => if then_action.targeting_type() != TargetingType::None {
                then_action.targeting_type()
            } else {
                else_action.as_ref().and_then(|a| Some(a.targeting_type())).unwrap_or(TargetingType::None)
            }
            ,

            // These cases should never be reached due to has_targeting_type() check
            _ => TargetingType::None,
        }
    }
}

impl ActionPrototype {
    pub fn builder() -> ActionPrototypeBuilder {
        ActionPrototypeBuilder::new()
    }
}

impl TryFrom<Table> for ActionEffectPrototype {
    type Error = Error;

    fn try_from(value: Table) -> Result<Self, Self::Error> {
        let Some(JanetEnum::String(action_type)) = value.get("type") else {
            return Err(Error::Incomplete("Type not found"));
        };

        match action_type.as_str() {
            "apply-effect" => {
                let Some(effect_type) = value.get("effect") else {
                    return Err(Error::Incomplete("Action Effect not found"));
                };

                let Some(effect_duration) = value.get("duration") else {
                    return Err(Error::Incomplete("Effect duration not found"));
                };

                let Some(targeting_type) = value.get("targeting") else {
                    return Err(Error::Incomplete("Targeting Type not found"));
                };

                Ok(Self::ApplyEffect {
                    effect: effect_type.try_into()?,
                    duration: effect_duration.try_into().map_err(Error::EngineError)?,
                    targeting_type: targeting_type.try_into()?,
                })
            }
            "get-gold" => {
                let Some(amount) = value.get("amount") else {
                    return Err(Error::Incomplete("Amount not found"));
                };

                Ok(Self::AddGold {
                    amount: amount.try_into().map_err(Error::EngineError)?,
                })
            }
            "move-creature" => {
                let Some(direction) = value.get("direction") else {
                    return Err(Error::Incomplete("Direction not found"));
                };

                Ok(Self::MoveCreature {
                    direction: direction.try_into().map_err(Error::EngineError)?,
                })
            }
            "damage" => {
                let Some(amount) = value.get("amount") else {
                    return Err(Error::Incomplete("Amount not found"));
                };

                let Some(targeting_type) = value.get("targeting") else {
                    return Err(Error::Incomplete("Targeting Type not found"));
                };

                Ok(Self::DealDamage {
                    targeting_type: targeting_type.try_into()?,
                    amount: amount.try_into().map_err(Error::EngineError)?,
                })
            }
            "destroy" => {
                let Some(targeting_type) = value.get("targeting") else {
                    return Err(Error::Incomplete("Targeting Type not found"));
                };

                Ok(Self::DestroyCreature {
                    targeting_type: targeting_type.try_into()?,
                })
            }
            "heal" => {
                let Some(targeting_type) = value.get("targeting") else {
                    return Err(Error::Incomplete("Targeting Type not found"));
                };

                let Some(amount) = value.get("amount") else {
                    return Err(Error::Incomplete("Amount not found"));
                };

                Ok(Self::HealCreature {
                    amount: amount.try_into().map_err(Error::EngineError)?,
                    targeting_type: targeting_type.try_into()?,
                })
            }

            _ => Err(Error::Cast("TargetingType not found".into())),
        }
    }
}
