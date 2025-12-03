use crate::game::{
    board::effect::Effect,
    card::card_id::CardID,
    events::{
        action::{Action, ActionTiming, SpellSpeed},
        action_context::ActionContext,
        action_effect::{ActionEffect, Condition, TargetingType},
        action_prototype::{ActionEffectPrototype, ActionPrototype},
        event::Event,
    },
    phases::Phase,
};

// The verification macro
macro_rules! verify_targets {
    ($targeting_type:expr, $targets:expr) => {{
        let targets = $targets;
        match $targeting_type.verify(&targets) {
            true => targets,
            false => return Err(ActionBuilderError::InvalidTargeting),
        }
    }};
}

#[derive(Debug, Clone)]
pub struct ActionPrototypeBuilder {
    action: Option<ActionEffectPrototype>,
    timing: ActionTiming,
    speed: SpellSpeed,
    can_be_countered: bool,
}

impl ActionPrototypeBuilder {
    pub fn new() -> Self {
        Self {
            action: None,
            timing: ActionTiming::Immediate,
            speed: SpellSpeed::Slow,
            can_be_countered: true,
        }
    }

    // Core action setters
    pub fn place_creature(mut self) -> Self {
        self.action = Some(ActionEffectPrototype::PlaceCreature);
        self
    }

    pub fn place_trap(mut self) -> Self {
        self.action = Some(ActionEffectPrototype::PlaceTrap);
        self
    }

    pub fn end_turn(mut self) -> Self {
        self.action = Some(ActionEffectPrototype::EndTurn);
        self
    }

    pub fn cast_spell(mut self) -> Self {
        self.action = Some(ActionEffectPrototype::CastSpell);
        self
    }

    pub fn move_creature(mut self) -> Self {
        self.action = Some(ActionEffectPrototype::MoveCreature);
        self
    }

    pub fn deal_damage(mut self, targeting_type: TargetingType, amount: u16) -> Self {
        self.action = Some(ActionEffectPrototype::DealDamage {
            amount,
            targeting_type,
        });
        self
    }

    pub fn heal_creature(mut self, targeting_type: TargetingType, amount: u16) -> Self {
        self.action = Some(ActionEffectPrototype::HealCreature {
            targeting_type,
            amount,
        });
        self
    }

    pub fn draw_cards(mut self, count: u16) -> Self {
        self.action = Some(ActionEffectPrototype::DrawCards { count });
        self
    }

    pub fn add_gold(mut self, amount: i64) -> Self {
        self.action = Some(ActionEffectPrototype::AddGold { amount });
        self
    }

    pub fn apply_effect(
        mut self,
        effect: Effect,
        targeting_type: TargetingType,
        duration: u32,
    ) -> Self {
        self.action = Some(ActionEffectPrototype::ApplyEffect {
            effect,
            targeting_type,
            duration,
        });
        self
    }

    pub fn summon_creature(mut self, creature_id: CardID) -> Self {
        self.action = Some(ActionEffectPrototype::SummonCreature { creature_id });
        self
    }

    pub fn destroy_creature(mut self, targeting_type: TargetingType) -> Self {
        self.action = Some(ActionEffectPrototype::DestroyCreature { targeting_type });
        self
    }

    // Composite actions
    pub fn sequence(mut self, actions: Vec<ActionEffectPrototype>) -> Self {
        self.action = Some(ActionEffectPrototype::Sequence(actions));
        self
    }

    pub fn conditional(
        mut self,
        condition: Condition,
        then_action: ActionEffectPrototype,
        else_action: Option<ActionEffectPrototype>,
    ) -> Self {
        self.action = Some(ActionEffectPrototype::Conditional {
            condition,
            then_action: Box::new(then_action),
            else_action: else_action.map(Box::new),
        });
        self
    }

    // Action properties
    pub fn with_timing(mut self, timing: ActionTiming) -> Self {
        self.timing = timing;
        self
    }

    pub fn immediate(mut self) -> Self {
        self.timing = ActionTiming::Immediate;
        self
    }

    pub fn delayed(mut self, turns: u32, phase: Phase) -> Self {
        self.timing = ActionTiming::Delayed { turns, phase };
        self
    }

    pub fn at_trigger(mut self, event: Event) -> Self {
        self.timing = ActionTiming::AtTrigger { trigger: event };
        self
    }

    pub fn with_speed(mut self, speed: SpellSpeed) -> Self {
        self.speed = speed;
        self
    }

    pub fn slow_speed(mut self) -> Self {
        self.speed = SpellSpeed::Slow;
        self
    }

    pub fn fast_speed(mut self) -> Self {
        self.speed = SpellSpeed::Fast;
        self
    }

    pub fn instant_speed(mut self) -> Self {
        self.speed = SpellSpeed::Instant;
        self
    }

    pub fn uncounterable(mut self) -> Self {
        self.can_be_countered = false;
        self
    }

    pub fn counterable(mut self) -> Self {
        self.can_be_countered = true;
        self
    }

    pub fn play_command_speed(mut self) -> Self {
        self.can_be_countered = true;
        self.speed = SpellSpeed::Slow;
        self.timing = ActionTiming::Immediate;
        self
    }

    // Build the final Action
    pub fn build(self) -> Result<ActionPrototype, ActionBuilderError> {
        let action = self.action.ok_or(ActionBuilderError::NoActionSet)?;

        Ok(ActionPrototype {
            action,
            timing: self.timing,
            speed: self.speed,
            can_be_countered: self.can_be_countered,
        })
    }
}

impl Default for ActionPrototypeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum ActionBuilderError {
    NoActionSet,
    MissingRequiredField(&'static str),
    InvalidConfiguration(String),
    InvalidTargeting,
}

impl std::fmt::Display for ActionBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionBuilderError::NoActionSet => write!(f, "No action was set on the builder"),
            ActionBuilderError::MissingRequiredField(field) => {
                write!(f, "Missing required field: {}", field)
            }
            ActionBuilderError::InvalidConfiguration(msg) => {
                write!(f, "Invalid configuration: {}", msg)
            }
            ActionBuilderError::InvalidTargeting => write!(f, "Invalid targeting"),
        }
    }
}

impl Action {
    // NEW: Create ActionBuilder from prototype + context
    pub fn from_prototype(
        proto: ActionPrototype,
        ctx: ActionContext,
    ) -> Result<Action, ActionBuilderError> {
        let effect = Self::finalize_prototype_effect(proto.action, &ctx)?;

        // Set player from context if available
        let Some(player_id) = ctx.player_id else {
            return Err(ActionBuilderError::MissingRequiredField("player id"));
        };

        Ok(Action {
            action: effect,
            player: player_id,
            speed: proto.speed,
            timing: proto.timing,
            priority: ctx.priority,
            can_be_countered: proto.can_be_countered,
        })
    }

    // Convert ActionEffectPrototype to ActionEffect using context
    fn finalize_prototype_effect(
        proto: ActionEffectPrototype,
        ctx: &ActionContext,
    ) -> Result<ActionEffect, ActionBuilderError> {
        // Helper macro to extract required fields
        macro_rules! req {
            ($field:ident) => {
                ctx.$field
                    .ok_or(ActionBuilderError::MissingRequiredField(stringify!($field)))?
            };
            ($field:ident.clone()) => {
                ctx.$field
                    .clone()
                    .ok_or(ActionBuilderError::MissingRequiredField(stringify!($field)))?
            };

            ($field:ident.clone().verify()) => {
                ctx.$field
                    .clone()
                    .ok_or(ActionBuilderError::MissingRequiredField(stringify!($field)))?
            };
        }

        use ActionEffect as E;
        use ActionEffectPrototype as P;

        Ok(match proto {
            // Actions requiring source + targets
            P::DealDamage {
                targeting_type,
                amount,
            } => {
                let (source, target) = (
                    req!(source),
                    verify_targets!(targeting_type, req!(targets.clone())),
                );

                E::DealDamage {
                    target,
                    amount,
                    source,
                }
            }
            P::HealCreature {
                targeting_type,
                amount,
            } => E::HealCreature {
                target: verify_targets!(targeting_type, req!(targets.clone())),
                amount,
                source: req!(source),
            },
            P::DestroyCreature { targeting_type } => E::DestroyCreature {
                targets: verify_targets!(targeting_type, req!(targets.clone())),
            },
            P::ApplyEffect {
                effect,
                targeting_type,
                duration,
            } => E::ApplyEffect {
                effect,
                targets: verify_targets!(targeting_type, req!(targets.clone())),
                duration,
            },

            // Actions requiring player + card_index + position
            P::PlaceCreature {} => E::PlaceCreature {
                card_index: req!(card_index),
                position: req!(position),
                player_id: req!(player_id),
            },
            P::PlaceTrap => E::PlaceTrap {
                card_index: req!(card_index),
                position: req!(position),
                player_id: req!(player_id),
            },

            // Actions requiring player + card_index
            P::CastSpell => E::CastSpell {
                card_index: req!(card_index),
                player_id: req!(player_id),
            },

            // Actions requiring player only
            P::DrawCards { count } => E::DrawCards {
                player_id: req!(player_id),
                count,
            },
            P::AddGold { amount } => E::AddGold {
                player_id: req!(player_id),
                amount,
            },

            // Actions requiring position + owner
            P::SummonCreature { creature_id } => E::SummonCreature {
                creature_id,
                position: req!(position),
                owner: req!(player_id),
            },

            // Actions requiring from/to positions
            P::MoveCreature => E::MoveCreature {
                from: req!(from),
                to: req!(to),
                player_id: req!(player_id),
            },

            // No context required
            P::EndTurn => E::EndTurn,

            // Recursive cases
            P::Sequence(protos) => E::Sequence(
                protos
                    .into_iter()
                    .map(|p| Self::finalize_prototype_effect(p, ctx))
                    .collect::<Result<Vec<_>, _>>()?,
            ),
            P::Conditional {
                condition,
                then_action,
                else_action,
            } => E::Conditional {
                condition,
                then_action: Box::new(Self::finalize_prototype_effect(*then_action, ctx)?),
                else_action: else_action
                    .map(|e| Self::finalize_prototype_effect(*e, ctx))
                    .transpose()?
                    .map(Box::new),
            },

            P::Custom {
                action,
                targeting_type,
            } => {
                let target = verify_targets!(targeting_type, req!(targets.clone()));
                E::Custom { action, target }
            }
        })
    }
}

impl std::error::Error for ActionBuilderError {}

// Convenience trait for converting ActionEffect to Action
impl From<ActionEffectPrototype> for ActionPrototypeBuilder {
    fn from(effect: ActionEffectPrototype) -> Self {
        Self::new().with_action_effect(effect)
    }
}

impl ActionPrototypeBuilder {
    fn with_action_effect(mut self, action: ActionEffectPrototype) -> Self {
        self.action = Some(action);
        self
    }
}
