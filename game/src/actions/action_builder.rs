use bevy::math::I16Vec2;

use super::{
    spell_speed::SpellSpeed,
    targeting::{CreatureSel, MultiTarget, PlayerSel, SingleTarget, TileSel},
    timing::ActionTiming,
    {GameAction, UnitAction, ValueSource},
};
use crate::board::effect::EffectType;
use crate::card::card_id::CardID;
use crate::phases::Phase;

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
    action: Option<UnitAction>,
    timing: ActionTiming,
    speed: SpellSpeed,
    can_be_countered: bool,
    optional: bool,
}

impl ActionPrototypeBuilder {
    pub fn new() -> Self {
        Self {
            action: None,
            timing: ActionTiming::Immediate,
            speed: SpellSpeed::Slow,
            can_be_countered: true,
            optional: false,
        }
    }

    pub fn with_action(mut self, action: UnitAction) -> Self {
        self.action = Some(action);
        self
    }
    // Core action setters
    pub fn place_creature(mut self) -> Self {
        self.action = Some(UnitAction::PlaceCreature);
        self
    }

    pub fn place_trap(mut self) -> Self {
        self.action = Some(UnitAction::PlaceTrap);
        self
    }

    pub fn end_turn(mut self) -> Self {
        self.action = Some(UnitAction::EndTurn);
        self
    }

    pub fn cast_spell(mut self) -> Self {
        self.action = Some(UnitAction::CastSpell);
        self
    }

    pub fn move_creature(
        mut self,
        direction_x: ValueSource,
        direction_y: ValueSource,
        absolute: bool,
        target: CreatureSel<SingleTarget>,
    ) -> Self {
        self.action = Some(UnitAction::MoveCreature {
            direction_x,
            direction_y,
            absolute,
            target,
        });
        self
    }

    pub fn deal_damage<S>(mut self, targeting_type: S, amount: ValueSource) -> Self
    where
        S: Into<CreatureSel<super::targeting::Or<SingleTarget, MultiTarget>>>,
    {
        self.action = Some(UnitAction::DealDamage {
            amount,
            target_selector: targeting_type.into(),
        });
        self
    }

    pub fn heal_creature<S>(mut self, targeting_type: S, amount: ValueSource) -> Self
    where
        S: Into<CreatureSel<super::targeting::Or<SingleTarget, MultiTarget>>>,
    {
        self.action = Some(UnitAction::HealCreature {
            amount,
            target_selector: targeting_type.into(),
        });
        self
    }

    pub fn draw_cards<S>(mut self, count: ValueSource, players_selector: S) -> Self
    where
        S: Into<PlayerSel<super::targeting::Or<SingleTarget, MultiTarget>>>,
    {
        self.action = Some(UnitAction::DrawCards {
            count,
            player_selector: players_selector.into(),
        });
        self
    }

    pub fn add_gold<S>(mut self, amount: ValueSource, players_selector: S) -> Self
    where
        S: Into<PlayerSel<super::targeting::Or<SingleTarget, MultiTarget>>>,
    {
        self.action = Some(UnitAction::AddGold {
            amount,
            player_selector: players_selector.into(),
        });
        self
    }

    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }

    pub fn apply_effect<S>(
        mut self,
        effect: EffectType,
        duration: ValueSource,
        targeting_type: S,
    ) -> Self
    where
        S: Into<TileSel<super::targeting::Or<SingleTarget, MultiTarget>>>,
    {
        self.action = Some(UnitAction::ApplyEffect {
            effect,
            duration,
            targeting_type: targeting_type.into(),
        });
        self
    }

    pub fn summon_creature(mut self, creature_id: CardID, position: TileSel<SingleTarget>) -> Self {
        self.action = Some(UnitAction::SummonCreature {
            creature_id,
            position,
        });
        self
    }

    pub fn destroy_creature<S>(mut self, targeting_type: S) -> Self
    where
        S: Into<CreatureSel<super::targeting::Or<SingleTarget, MultiTarget>>>,
    {
        self.action = Some(UnitAction::DestroyCreature {
            targeting_type: targeting_type.into(),
        });
        self
    }

    // Composite actions
    pub fn sequence(mut self, actions: Vec<UnitAction>) -> Self {
        self.action = Some(UnitAction::Sequence(actions));
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
    pub fn build(self) -> Result<GameAction, ActionBuilderError> {
        let action = self.action.ok_or(ActionBuilderError::NoActionSet)?;

        Ok(GameAction {
            action,
            timing: self.timing,
            speed: self.speed,
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

impl std::error::Error for ActionBuilderError {}

// Convenience trait for converting ActionEffect to Action
impl From<UnitAction> for ActionPrototypeBuilder {
    fn from(effect: UnitAction) -> Self {
        Self::new().with_action_effect(effect)
    }
}

impl ActionPrototypeBuilder {
    fn with_action_effect(mut self, action: UnitAction) -> Self {
        self.action = Some(action);
        self
    }
}
