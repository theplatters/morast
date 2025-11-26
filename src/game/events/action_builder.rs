use crate::game::{
    board::effect::Effect,
    card::{card_id::CardID, in_play_id::InPlayID},
    events::{
        action::{Action, ActionTiming, SpellSpeed},
        action_effect::{ActionEffect, Condition, CreatureFilter, TargetingType},
        event::Event,
    },
    phases::Phase,
    player::PlayerID,
};
use macroquad::math::I16Vec2;

#[derive(Debug, Clone)]
pub struct ActionBuilder {
    action: Option<ActionEffect>,
    timing: ActionTiming,
    speed: SpellSpeed,
    priority: u32,
    source: Option<InPlayID>,
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
            source: None,
            player: None,
            can_be_countered: true,
        }
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

    pub fn deal_damage(mut self, target: TargetingType, amount: u16, source: InPlayID) -> Self {
        self.action = Some(ActionEffect::DealDamage {
            target,
            amount,
            source,
        });
        self.source = Some(source);
        self
    }

    pub fn heal_creature(mut self, target: TargetingType, amount: u16, source: InPlayID) -> Self {
        self.action = Some(ActionEffect::HealCreature {
            target,
            amount,
            source,
        });
        self.source = Some(source);
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

    pub fn apply_effect(mut self, effect: Effect, tiles: TargetingType, duration: u32) -> Self {
        self.action = Some(ActionEffect::ApplyEffect {
            effect,
            tiles,
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

    pub fn destroy_creature(mut self, target: TargetingType) -> Self {
        self.action = Some(ActionEffect::DestroyCreature { target });
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

    pub fn for_each_in_area(mut self, center: I16Vec2, radius: u8, action: ActionEffect) -> Self {
        self.action = Some(ActionEffect::ForEachInArea {
            center,
            radius,
            action: Box::new(action),
        });
        self
    }

    pub fn for_each_creature(mut self, filter: CreatureFilter, action: ActionEffect) -> Self {
        self.action = Some(ActionEffect::ForEachCreature {
            filter,
            action: Box::new(action),
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

    pub fn with_source(mut self, source: InPlayID) -> Self {
        self.source = Some(source);
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

    // Build the final Action
    pub fn build(self) -> Result<Action, ActionBuilderError> {
        let action = self.action.ok_or(ActionBuilderError::NoActionSet)?;
        let source = self
            .source
            .ok_or(ActionBuilderError::MissingRequiredField("source"))?;
        let player = self
            .player
            .ok_or(ActionBuilderError::MissingRequiredField("source"))?;

        Ok(Action {
            action,
            timing: self.timing,
            speed: self.speed,
            priority: self.priority,
            source,
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
            source: self.source.unwrap(),
            player: self.player.unwrap(),
            can_be_countered: self.can_be_countered,
        }
    }
}

impl Default for ActionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// Convenience constructors for common action types
impl ActionBuilder {
    pub fn damage(target: TargetingType, amount: u16, source: InPlayID) -> Self {
        Self::new().deal_damage(target, amount, source)
    }

    pub fn heal(target: TargetingType, amount: u16, source: InPlayID) -> Self {
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

    pub fn destroy(target: TargetingType) -> Self {
        Self::new().destroy_creature(target)
    }

    // Area effect builders
    pub fn area_damage(radius: u8, amount: u16, source: InPlayID) -> Self {
        let damage_action = ActionEffect::DealDamage {
            target: TargetingType::Area { radius }, // Will be replaced by for_each
            amount,
            source,
        };

        Self::new().with_action(damage_action).with_source(source)
    }

    pub fn area_heal(radius: u8, amount: u16, source: InPlayID) -> Self {
        let heal_action = ActionEffect::HealCreature {
            target: TargetingType::Area { radius },
            amount,
            source,
        };

        Self::new()
            .with_action(heal_action)
            .with_source(source)
            .with_source(source)
    }
}

#[derive(Debug, Clone)]
pub enum ActionBuilderError {
    NoActionSet,
    MissingRequiredField(&'static str),
    InvalidConfiguration(String),
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
        }
    }
}

impl std::error::Error for ActionBuilderError {}

// Convenience trait for converting ActionEffect to Action
impl From<ActionEffect> for ActionBuilder {
    fn from(effect: ActionEffect) -> Self {
        Self::new().with_action(effect)
    }
}

impl ActionBuilder {
    fn with_action(mut self, action: ActionEffect) -> Self {
        self.action = Some(action);
        self
    }
}
