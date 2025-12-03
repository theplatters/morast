use macroquad::math::I16Vec2;

use crate::game::{
    board::effect::Effect,
    card::card_id::CardID,
    events::{
        action::{Action, ActionTiming, SpellSpeed},
        action_builder::{ActionBuilderError, ActionPrototypeBuilder},
        action_context::ActionContext,
        action_effect::{Condition, TargetingType},
    },
    janet_action::JanetAction,
};

#[derive(Debug)]
pub struct ActionPrototype {
    pub action: ActionEffectPrototype,
    pub timing: ActionTiming,
    pub speed: SpellSpeed,
    pub can_be_countered: bool,
}

impl ActionPrototype {
    fn finalize(self, action_context: ActionContext) -> Result<Action, ActionBuilderError> {
        Action::from_prototype(self, action_context)
    }
}

#[derive(Debug, Clone)]
pub enum ActionEffectPrototype {
    // Basic game actions
    PlaceCreature,
    CastSpell,
    PlaceTrap,
    MoveCreature,
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
        amount: i64,
    },
    ApplyEffect {
        effect: Effect,
        targeting_type: TargetingType,
        duration: u32,
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
    fn has_targeting_type(&self) -> bool {
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
            | ActionEffectPrototype::MoveCreature
            | ActionEffectPrototype::DrawCards { .. }
            | ActionEffectPrototype::AddGold { .. }
            | ActionEffectPrototype::SummonCreature { .. } => false,
            ActionEffectPrototype::Sequence(actions) => {
                actions.iter().any(|action| action.has_targeting_type())
            }
            ActionEffectPrototype::Conditional {
                then_action,
                else_action,
                ..
            } => {
                then_action.has_targeting_type()
                    || else_action
                        .as_ref()
                        .is_some_and(|action| action.has_targeting_type())
            }
            ActionEffectPrototype::Custom { targeting_type, .. } => {
                targeting_type.requires_selection()
            }
            ActionEffectPrototype::EndTurn => false,
        }
    }

    /// Returns the targeting type if this action requires targeting
    fn targeting_type(&self) -> Option<TargetingType> {
        if !self.has_targeting_type() {
            return None;
        }

        match self {
            ActionEffectPrototype::DealDamage { targeting_type, .. }
            | ActionEffectPrototype::HealCreature { targeting_type, .. }
            | ActionEffectPrototype::ApplyEffect { targeting_type, .. }
            | ActionEffectPrototype::Custom { targeting_type, .. }
            | ActionEffectPrototype::DestroyCreature { targeting_type } => {
                if targeting_type.requires_selection() {
                    Some(*targeting_type)
                } else {
                    None
                }
            }

            // For composite actions, return the first targeting type found
            // Note: This assumes only one action in a sequence requires targeting
            // You might want to handle this differently based on your game's needs
            ActionEffectPrototype::Sequence(actions) => {
                actions.iter().find_map(|action| action.targeting_type())
            }
            ActionEffectPrototype::Conditional {
                then_action,
                else_action,
                ..
            } => then_action.targeting_type().or_else(|| {
                else_action
                    .as_ref()
                    .and_then(|action| action.targeting_type())
            }),

            // These cases should never be reached due to has_targeting_type() check
            _ => None,
        }
    }
}

impl ActionPrototype {
    pub fn builder() -> ActionPrototypeBuilder {
        ActionPrototypeBuilder::new()
    }
}
