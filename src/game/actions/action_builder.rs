use macroquad::math::I16Vec2;

use crate::game::{
    actions::{
        action::{Action, SpellSpeed},
        action_context::ActionContext,
        action_effect::{ActionEffect, Condition},
        action_prototype::{ActionEffectPrototype, ActionPrototype},
        targeting::TargetingType,
        timing::ActionTiming,
    },
    board::effect::Effect,
    card::card_id::CardID,
    events::event::Event,
    phases::Phase,
    player::PlayerID,
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

    pub fn with_action(mut self, action: ActionEffectPrototype) -> Self {
        self.action = Some(action);
        self
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

    pub fn move_creature(mut self, direction: I16Vec2) -> Self {
        self.action = Some(ActionEffectPrototype::MoveCreature { direction });
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

    pub fn add_gold(mut self, amount: u16) -> Self {
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
            effect: effect.effect_type,
            duration: effect.duration(),
            targeting_type,
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

pub struct ActionBuilder {
    pub action: Option<ActionEffect>,
    pub timing: Option<ActionTiming>,
    pub speed: Option<SpellSpeed>,
    pub priority: Option<u32>,
    pub player: Option<PlayerID>,
    pub can_be_countered: bool,
}
impl Action {
    pub fn builder() -> ActionBuilder {
        ActionBuilder::new()
    }
}
impl ActionBuilder {
    pub fn new() -> Self {
        Self {
            action: None,
            timing: None,
            speed: None,
            priority: None,
            player: None,
            can_be_countered: true,
        }
    }

    // Core action setters
    pub fn place_creature(mut self, card_index: usize, position: I16Vec2) -> Self {
        self.action = Some(ActionEffect::PlaceCreature {
            card_index,
            position,
        });
        self
    }

    pub fn place_trap(mut self, card_index: usize, position: I16Vec2) -> Self {
        self.action = Some(ActionEffect::PlaceTrap {
            card_index,
            position,
        });
        self
    }

    pub fn end_turn(mut self) -> Self {
        self.action = Some(ActionEffect::EndTurn);
        self
    }

    pub fn cast_spell(mut self, card_index: usize) -> Self {
        self.action = Some(ActionEffect::CastSpell { card_index });
        self
    }

    pub fn move_creature(mut self, from: I16Vec2, to: I16Vec2) -> Self {
        self.action = Some(ActionEffect::MoveCreature { from, to });
        self
    }

    pub fn deal_damage(mut self, target: Vec<I16Vec2>, amount: u16) -> Self {
        self.action = Some(ActionEffect::DealDamage { amount, target });
        self
    }

    pub fn heal_creature(mut self, target: Vec<I16Vec2>, amount: u16) -> Self {
        self.action = Some(ActionEffect::HealCreature { target, amount });
        self
    }

    pub fn draw_cards(mut self, count: u16, player_id: PlayerID) -> Self {
        self.action = Some(ActionEffect::DrawCards { count, player_id });
        self
    }

    pub fn add_gold(mut self, amount: u16, player_id: PlayerID) -> Self {
        self.action = Some(ActionEffect::AddGold { amount, player_id });
        self
    }

    pub fn apply_effect(mut self, effect: Effect, targets: Vec<I16Vec2>, duration: u32) -> Self {
        self.action = Some(ActionEffect::ApplyEffect { effect, targets });
        self
    }

    pub fn summon_creature(
        mut self,
        creature_id: CardID,
        position: I16Vec2,
        owner: PlayerID,
    ) -> Self {
        self.action = Some(ActionEffect::SummonCreature {
            creature_id,
            position,
            owner,
        });
        self
    }

    pub fn destroy_creature(mut self, targets: Vec<I16Vec2>) -> Self {
        self.action = Some(ActionEffect::DestroyCreature { targets });
        self
    }

    // Composite actions
    pub fn sequence(mut self, actions: Vec<ActionEffect>) -> Self {
        self.action = Some(ActionEffect::Sequence(actions));
        self
    }

    pub fn conditional(
        mut self,
        condition: Condition,
        then_action: ActionEffect,
        else_action: Option<ActionEffect>,
    ) -> Self {
        self.action = Some(ActionEffect::Conditional {
            condition,
            then_action: Box::new(then_action),
            else_action: else_action.map(Box::new),
        });
        self
    }

    // Action properties
    pub fn with_timing(mut self, timing: ActionTiming) -> Self {
        self.timing = Some(timing);
        self
    }

    pub fn immediate(mut self) -> Self {
        self.timing = Some(ActionTiming::Immediate);
        self
    }

    pub fn delayed(mut self, turns: u32, phase: Phase) -> Self {
        self.timing = Some(ActionTiming::Delayed { turns, phase });
        self
    }

    pub fn at_trigger(mut self, event: Event) -> Self {
        self.timing = Some(ActionTiming::AtTrigger { trigger: event });
        self
    }

    pub fn with_speed(mut self, speed: SpellSpeed) -> Self {
        self.speed = Some(speed);
        self
    }

    pub fn slow_speed(mut self) -> Self {
        self.speed = Some(SpellSpeed::Slow);
        self
    }

    pub fn fast_speed(mut self) -> Self {
        self.speed = Some(SpellSpeed::Fast);
        self
    }

    pub fn instant_speed(mut self) -> Self {
        self.speed = Some(SpellSpeed::Instant);
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
        self.speed = Some(SpellSpeed::Slow);
        self.timing = Some(ActionTiming::Immediate);
        self
    }

    pub fn with_player(mut self, player_id: PlayerID) -> Self {
        self.player = Some(player_id);
        self
    }

    // Build the final Action
    pub fn build(self) -> Result<Action, ActionBuilderError> {
        let action = self.action.ok_or(ActionBuilderError::NoActionSet)?;

        Ok(Action {
            action,
            timing: self.timing.unwrap_or_default(),
            priority: self.priority.unwrap_or_default(),
            speed: self.speed.unwrap_or_default(),
            player: self
                .player
                .ok_or(ActionBuilderError::MissingRequiredField("player"))?,
            can_be_countered: self.can_be_countered,
        })
    }
}

impl Action {
    // NEW: Create ActionBuilder from prototype + context
    pub fn from_prototype(
        proto: ActionPrototype,
        ctx: &ActionContext,
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
                let target = verify_targets!(targeting_type, req!(targets.clone()));

                E::DealDamage { target, amount }
            }
            P::HealCreature {
                targeting_type,
                amount,
            } => E::HealCreature {
                target: verify_targets!(targeting_type, req!(targets.clone())),
                amount,
            },
            P::DestroyCreature { targeting_type } => E::DestroyCreature {
                targets: verify_targets!(targeting_type, req!(targets.clone())),
            },
            P::ApplyEffect {
                effect,
                targeting_type,
                duration,
            } => {
                let effect = Effect::new(effect, duration, req!(player_id));
                if matches!(
                    targeting_type,
                    TargetingType::Area { .. }
                        | TargetingType::Line { .. }
                        | TargetingType::Tiles { .. }
                ) {
                    E::ApplyEffect {
                        effect,
                        targets: verify_targets!(targeting_type, req!(targets.clone())),
                    }
                } else {
                    E::ApplyEffect {
                        effect,
                        targets: vec![req!(caster_position)],
                    }
                }
            }

            // Actions requiring player + card_index + position
            P::PlaceCreature => E::PlaceCreature {
                card_index: req!(card_index),
                position: req!(position),
            },
            P::PlaceTrap => E::PlaceTrap {
                card_index: req!(card_index),
                position: req!(position),
            },

            // Actions requiring player + card_index
            P::CastSpell => E::CastSpell {
                card_index: req!(card_index),
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
            P::MoveCreature { .. } => E::MoveCreature {
                from: req!(from),
                to: req!(to),
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
