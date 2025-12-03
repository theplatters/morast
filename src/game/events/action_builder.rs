use crate::game::{
    board::effect::Effect,
    card::{card_id::CardID, in_play_id::InPlayID},
    events::{
        action::{Action, ActionTiming, SpellSpeed},
        action_context::ActionContext,
        action_effect::{ActionEffect, Condition, TargetingType},
        action_prototype::{ActionEffectPrototype, ActionPrototype},
        event::Event,
    },
    phases::Phase,
    player::PlayerID,
};
use macroquad::math::I16Vec2;
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
pub struct ActionBuilder {
    action: Option<ActionEffect>,
    timing: ActionTiming,
    speed: SpellSpeed,
    priority: u32,
    player: Option<PlayerID>,
    can_be_countered: bool,
}

impl ActionBuilder {
    pub fn new() -> Self {
        Self {
            action: None,
            timing: ActionTiming::Immediate,
            speed: SpellSpeed::Slow,
            priority: 0,
            player: None,
            can_be_countered: true,
        }
    }

    // NEW: Create ActionBuilder from prototype + context
    pub fn with_prototype(
        mut self,
        proto: ActionPrototype,
        ctx: ActionContext,
    ) -> Result<Self, ActionBuilderError> {
        let effect = Self::finalize_prototype_effect(proto.action, &ctx)?;

        // Set player from context if available
        let Some(player_id) = ctx.player_id else {
            return Err(ActionBuilderError::MissingRequiredField("player id"));
        };

        self.action = Some(effect);
        self.player = Some(player_id);
        self.speed = proto.speed;
        self.timing = proto.timing;

        Ok(self)
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

    // Core action setters
    pub fn place_creature(
        mut self,
        card_index: usize,
        position: I16Vec2,
        player_id: PlayerID,
    ) -> Self {
        self.action = Some(ActionEffect::PlaceCreature {
            card_index,
            position,
            player_id,
        });
        self.player = Some(player_id);
        self
    }

    pub fn place_trap(mut self, card_index: usize, position: I16Vec2, player_id: PlayerID) -> Self {
        self.action = Some(ActionEffect::PlaceTrap {
            card_index,
            position,
            player_id,
        });
        self.player = Some(player_id);
        self
    }

    pub fn end_turn(mut self) -> Self {
        self.action = Some(ActionEffect::EndTurn);
        self
    }

    pub fn cast_spell(mut self, card_index: usize, player_id: PlayerID) -> Self {
        self.action = Some(ActionEffect::CastSpell {
            card_index,
            player_id,
        });
        self.player = Some(player_id);
        self
    }

    pub fn move_creature(mut self, from: I16Vec2, to: I16Vec2, player_id: PlayerID) -> Self {
        self.action = Some(ActionEffect::MoveCreature {
            from,
            to,
            player_id,
        });
        self.player = Some(player_id);
        self
    }

    pub fn deal_damage(mut self, target: Vec<I16Vec2>, amount: u16, source: InPlayID) -> Self {
        self.action = Some(ActionEffect::DealDamage {
            target,
            amount,
            source,
        });
        self
    }

    pub fn heal_creature(mut self, target: Vec<I16Vec2>, amount: u16, source: InPlayID) -> Self {
        self.action = Some(ActionEffect::HealCreature {
            target,
            amount,
            source,
        });
        self
    }

    pub fn draw_cards(mut self, player_id: PlayerID, count: u16) -> Self {
        self.action = Some(ActionEffect::DrawCards { player_id, count });
        self.player = Some(player_id);
        self
    }

    pub fn add_gold(mut self, player_id: PlayerID, amount: i64) -> Self {
        self.action = Some(ActionEffect::AddGold { player_id, amount });
        self.player = Some(player_id);
        self
    }

    pub fn apply_effect(mut self, effect: Effect, targets: Vec<I16Vec2>, duration: u32) -> Self {
        self.action = Some(ActionEffect::ApplyEffect {
            effect,
            targets,
            duration,
        });
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
        self.player = Some(owner);
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

    pub fn with_priority(mut self, priority: u32) -> Self {
        self.priority = priority;
        self
    }

    pub fn high_priority(mut self) -> Self {
        self.priority = 100;
        self
    }

    pub fn low_priority(mut self) -> Self {
        self.priority = 1;
        self
    }

    pub fn with_player(mut self, player: PlayerID) -> Self {
        self.player = Some(player);
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
    pub fn build(self) -> Result<Action, ActionBuilderError> {
        let action = self.action.ok_or(ActionBuilderError::NoActionSet)?;
        let player = self
            .player
            .ok_or(ActionBuilderError::MissingRequiredField("source"))?;

        Ok(Action {
            action,
            timing: self.timing,
            speed: self.speed,
            priority: self.priority,
            player,
            can_be_countered: self.can_be_countered,
        })
    }

    // Convenience method for common patterns
    pub fn build_unchecked(self) -> Action {
        Action {
            action: self.action.unwrap(),
            timing: self.timing,
            speed: self.speed,
            priority: self.priority,
            player: self.player.unwrap(),
            can_be_countered: self.can_be_countered,
        }
    }

    pub(crate) fn with_targets(
        mut self,
        action: Box<Action>,
        targets: Vec<I16Vec2>,
    ) -> ActionBuilder {
        self.action = Some(ActionEffect::WithTargets { action, targets });
        self
    }
}

impl Default for ActionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// Convenience constructors for common action types
impl ActionBuilder {
    pub fn damage(target: Vec<I16Vec2>, amount: u16, source: InPlayID) -> Self {
        Self::new().deal_damage(target, amount, source)
    }

    pub fn heal(target: Vec<I16Vec2>, amount: u16, source: InPlayID) -> Self {
        Self::new().heal_creature(target, amount, source)
    }

    pub fn draw(player_id: PlayerID, count: u16) -> Self {
        Self::new().draw_cards(player_id, count)
    }

    pub fn gold(player_id: PlayerID, amount: i64) -> Self {
        Self::new().add_gold(player_id, amount)
    }

    pub fn summon(creature_id: CardID, position: I16Vec2, owner: PlayerID) -> Self {
        Self::new().summon_creature(creature_id, position, owner)
    }

    pub fn destroy(target: Vec<I16Vec2>) -> Self {
        Self::new().destroy_creature(target)
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

impl std::error::Error for ActionBuilderError {}

// Convenience trait for converting ActionEffect to Action
impl From<ActionEffect> for ActionBuilder {
    fn from(effect: ActionEffect) -> Self {
        Self::new().with_action_effect(effect)
    }
}

impl ActionBuilder {
    fn with_action_effect(mut self, action: ActionEffect) -> Self {
        self.action = Some(action);
        self
    }
}
