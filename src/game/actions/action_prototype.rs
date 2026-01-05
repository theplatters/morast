// ============================================================================
// File: action_prototype.rs - Refined action system with conditions
// ============================================================================
use bevy::{
    ecs::{bundle::Bundle, component::Component},
    math::I16Vec2,
};

use crate::{
    engine::janet_handler::types::{janetenum::JanetEnum, table::Table},
    game::{
        actions::{
            action_builder::ActionPrototypeBuilder, spell_speed::SpellSpeed,
            targeting::TargetSelector, timing::ActionTiming,
        },
        board::effect::EffectType,
        card::card_id::CardID,
        error::GameError,
        janet_action::JanetAction,
    },
};

#[derive(Component, Debug, Clone, Copy)]
pub struct Pending;

#[derive(Component, Debug, Clone, Copy)]
pub struct NeedsTargeting;

#[derive(Component, Debug, Clone, Copy)]
pub struct ReadyToExecute;

#[derive(Component, Debug, Clone)]
pub struct Counterable;

#[derive(Bundle, Debug, Clone)]
pub struct GameAction {
    pub action: UnitAction,
    pub timing: ActionTiming,
    pub speed: SpellSpeed,
}

// ============================================================================
// Core Action Types
// ============================================================================

/// Main action effect that can be executed
#[derive(Component, Debug, Clone)]
pub enum UnitAction {
    // Basic game actions
    PlaceCreature,
    CastSpell,
    PlaceTrap,
    MoveCreature {
        direction: I16Vec2,
    },
    EndTurn,

    // Atomic effects
    DealDamage {
        targeting_type: TargetSelector,
        amount: ValueSource,
    },
    HealCreature {
        targeting_type: TargetSelector,
        amount: ValueSource,
    },
    DrawCards {
        count: ValueSource,
    },
    AddGold {
        amount: ValueSource,
    },
    ApplyEffect {
        effect: EffectType,
        duration: ValueSource,
        targeting_type: TargetSelector,
    },
    SummonCreature {
        creature_id: CardID,
        position: Option<I16Vec2>,
    },
    DestroyCreature {
        targeting_type: TargetSelector,
    },
    ModifyStats {
        targeting_type: TargetSelector,
        stat_modifier: StatModifier,
    },
    DiscardCards {
        count: ValueSource,
        random: bool,
    },
    ReturnToHand {
        targeting_type: TargetSelector,
    },
    Mill {
        count: ValueSource,
    },

    // Composite actions with better control flow
    Sequence(Vec<UnitAction>),
    Parallel(Vec<UnitAction>),
    Choice {
        options: Vec<UnitAction>,
        chooser: ChoiceSource,
    },
    Repeat {
        action: Box<UnitAction>,
        count: ValueSource,
    },

    // Conditional actions
    Conditional {
        condition: Condition,
        on_true: Box<UnitAction>,
        on_false: Option<Box<UnitAction>>,
    },

    // Advanced patterns
    ForEach {
        selector: TargetSelector,
        action: Box<UnitAction>,
    },

    // Custom scripted actions
    Custom {
        action: Box<JanetAction>,
        targeting_type: TargetSelector,
    },
}

// ============================================================================
// Value Sources - Dynamic value resolution
// ============================================================================

/// Represents where a numeric value comes from
#[derive(Debug, Clone, PartialEq)]
pub enum ValueSource {
    /// Static constant value
    Constant(u16),

    /// Count of entities matching a selector
    Count(TargetSelector),

    /// Random value in range [min, max]
    Random {
        min: u16,
        max: u16,
    },

    /// Value from creature stats
    CreatureStat {
        selector: TargetSelector,
        stat: StatType,
    },

    /// Mathematical operations
    Add(Box<ValueSource>, Box<ValueSource>),
    Multiply(Box<ValueSource>, Box<ValueSource>),
    Divide(Box<ValueSource>, Box<ValueSource>),
    Min(Box<ValueSource>, Box<ValueSource>),
    Max(Box<ValueSource>, Box<ValueSource>),
}

impl ValueSource {
    pub fn constant(value: u16) -> Self {
        Self::Constant(value)
    }

    pub fn count(selector: TargetSelector) -> Self {
        Self::Count(selector)
    }
}

// ============================================================================
// Stat Modifiers
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StatModifier {
    Attack(i16),
    Health(i16),
    MaxHealth(i16),
    Speed(i16),
    Both { attack: i16, health: i16 },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StatType {
    Attack,
    Health,
    MaxHealth,
    Speed,
}

// ============================================================================
// Choice Sources
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChoiceSource {
    /// Active player chooses
    ActivePlayer,

    /// Owner of the card chooses
    Owner,

    /// Opponent chooses
    Opponent,

    /// Random choice
    Random,
}

// ============================================================================
// Implementation
// ============================================================================

impl UnitAction {
    /// Returns true if this action effect requires targeting from the player
    pub fn requires_selection(&self) -> bool {
        match self {
            Self::DealDamage { targeting_type, .. }
            | Self::HealCreature { targeting_type, .. }
            | Self::ApplyEffect { targeting_type, .. }
            | Self::DestroyCreature { targeting_type }
            | Self::ModifyStats { targeting_type, .. }
            | Self::ReturnToHand { targeting_type } => targeting_type.requires_selection(),

            Self::PlaceCreature
            | Self::PlaceTrap
            | Self::CastSpell
            | Self::MoveCreature { .. }
            | Self::DrawCards { .. }
            | Self::AddGold { .. }
            | Self::DiscardCards { .. }
            | Self::Mill { .. }
            | Self::SummonCreature { .. } => false,

            Self::Sequence(actions) | Self::Parallel(actions) => {
                actions.iter().any(|action| action.requires_selection())
            }

            Self::Choice { options, .. } => {
                options.iter().any(|action| action.requires_selection())
            }

            Self::Repeat { action, .. } => action.requires_selection(),

            Self::Conditional {
                on_true, on_false, ..
            } => {
                on_true.requires_selection()
                    || on_false.as_ref().map_or(false, |a| a.requires_selection())
            }

            Self::ForEach { action, .. } => action.requires_selection(),

            Self::Custom { targeting_type, .. } => targeting_type.requires_selection(),

            Self::EndTurn => false,
        }
    }

    /// Returns the targeting type if this action requires targeting
    pub fn targeting_type(&self) -> TargetSelector {
        match self {
            Self::DealDamage { targeting_type, .. }
            | Self::HealCreature { targeting_type, .. }
            | Self::ApplyEffect { targeting_type, .. }
            | Self::Custom { targeting_type, .. }
            | Self::ModifyStats { targeting_type, .. }
            | Self::ReturnToHand { targeting_type }
            | Self::DestroyCreature { targeting_type } => *targeting_type,

            Self::MoveCreature { .. } => TargetSelector::SingleTile,

            Self::Sequence(actions) | Self::Parallel(actions) => actions
                .iter()
                .find_map(|a| {
                    if a.requires_selection() {
                        Some(a.targeting_type())
                    } else {
                        None
                    }
                })
                .unwrap_or(TargetSelector::None),

            _ => TargetSelector::None,
        }
    }

    /// Collects all targeting types needed for this action
    pub fn all_targeting_types(&self) -> Vec<TargetSelector> {
        let mut types = Vec::new();
        self.collect_targeting_types(&mut types);
        types
    }

    fn collect_targeting_types(&self, types: &mut Vec<TargetSelector>) {
        match self {
            Self::DealDamage { targeting_type, .. }
            | Self::HealCreature { targeting_type, .. }
            | Self::ApplyEffect { targeting_type, .. }
            | Self::DestroyCreature { targeting_type }
            | Self::ModifyStats { targeting_type, .. }
            | Self::ReturnToHand { targeting_type }
            | Self::Custom { targeting_type, .. } => {
                if targeting_type.requires_selection() {
                    types.push(*targeting_type);
                }
            }

            Self::Sequence(actions) | Self::Parallel(actions) => {
                for action in actions {
                    action.collect_targeting_types(types);
                }
            }

            Self::Choice { options, .. } => {
                for action in options {
                    action.collect_targeting_types(types);
                }
            }

            Self::Repeat { action, .. } | Self::ForEach { action, .. } => {
                action.collect_targeting_types(types);
            }

            Self::Conditional {
                on_true, on_false, ..
            } => {
                on_true.collect_targeting_types(types);
                if let Some(on_false) = on_false {
                    on_false.collect_targeting_types(types);
                }
            }

            _ => {}
        }
    }
}

impl GameAction {
    pub fn builder() -> ActionPrototypeBuilder {
        ActionPrototypeBuilder::new()
    }
}

// ============================================================================
// Builder helpers for common patterns
// ============================================================================

impl UnitAction {
    /// Deal damage to a target
    pub fn deal_damage(amount: u16, targeting: TargetSelector) -> Self {
        Self::DealDamage {
            targeting_type: targeting,
            amount: ValueSource::Constant(amount),
        }
    }

    /// Deal damage equal to a creature's attack
    pub fn deal_damage_equal_to_attack(targeting: TargetSelector, source: TargetSelector) -> Self {
        Self::DealDamage {
            targeting_type: targeting,
            amount: ValueSource::CreatureStat {
                selector: source,
                stat: StatType::Attack,
            },
        }
    }

    /// Conditional effect: "If you have 3+ creatures, draw a card"
    pub fn if_has_creatures_draw(count: u16, cards_to_draw: u16) -> Self {
        Self::Conditional {
            condition: Condition::Compare {
                left: ValueSource::Count(TargetSelector::FriendlyCreatures),
                op: CompareOp::GreaterOrEqual,
                right: ValueSource::Constant(count),
            },
            on_true: Box::new(Self::DrawCards {
                count: ValueSource::Constant(cards_to_draw),
            }),
            on_false: None,
        }
    }

    /// "Deal 1 damage to all enemy creatures"
    pub fn damage_all_enemies(amount: u16) -> Self {
        Self::ForEach {
            selector: TargetSelector::EnemyCreatures,
            action: Box::new(Self::DealDamage {
                targeting_type: TargetSelector::None,
                amount: ValueSource::Constant(amount),
            }),
        }
    }

    /// "Choose one: Draw 2 cards OR Gain 3 gold"
    pub fn choose_draw_or_gold() -> Self {
        Self::Choice {
            options: vec![
                Self::DrawCards {
                    count: ValueSource::Constant(2),
                },
                Self::AddGold {
                    amount: ValueSource::Constant(3),
                },
            ],
            chooser: ChoiceSource::ActivePlayer,
        }
    }

    /// "Deal damage equal to the number of friendly creatures"
    pub fn damage_equal_to_creature_count(targeting: TargetSelector) -> Self {
        Self::DealDamage {
            targeting_type: targeting,
            amount: ValueSource::Count(TargetSelector::FriendlyCreatures),
        }
    }
}

// ============================================================================
// Conversion from Janet
// ============================================================================

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

impl TryFrom<Table> for UnitAction {
    type Error = GameError;

    fn try_from(value: Table) -> Result<Self, Self::Error> {
        let Some(JanetEnum::String(action_type)) = value.get("type") else {
            return Err(GameError::Incomplete("Type not found"));
        };

        match action_type.as_str() {
            "damage" => parse_damage_action(&value),
            "heal" => parse_heal_action(&value),
            "apply-effect" => parse_apply_effect_action(&value),
            "get-gold" => parse_gold_action(&value),
            "draw" => parse_draw_action(&value),
            "destroy" => parse_destroy_action(&value),
            "move-creature" => parse_move_action(&value),
            "modify-stats" => parse_modify_stats_action(&value),
            "sequence" => parse_sequence_action(&value),
            "conditional" => parse_conditional_action(&value),
            "for-each" => parse_for_each_action(&value),
            "choice" => parse_choice_action(&value),
            _ => Err(GameError::Cast(format!(
                "Unknown action type: {}",
                action_type
            ))),
        }
    }
}

// Helper parsing functions
fn parse_damage_action(value: &Table) -> Result<UnitAction, GameError> {
    let targeting_type = value
        .get("targeting")
        .ok_or(GameError::Incomplete("Targeting Type not found"))?
        .try_into()?;

    let amount = parse_value_source(value, "amount")?;

    Ok(UnitAction::DealDamage {
        targeting_type,
        amount,
    })
}

fn parse_heal_action(value: &Table) -> Result<UnitAction, GameError> {
    let targeting_type = value
        .get("targeting")
        .ok_or(GameError::Incomplete("Targeting Type not found"))?
        .try_into()?;

    let amount = parse_value_source(value, "amount")?;

    Ok(UnitAction::HealCreature {
        targeting_type,
        amount,
    })
}

fn parse_apply_effect_action(value: &Table) -> Result<UnitAction, GameError> {
    let effect = value
        .get("effect")
        .ok_or(GameError::Incomplete("Effect not found"))?
        .try_into()?;

    let duration = parse_value_source(value, "duration")?;

    let targeting_type = value
        .get("targeting")
        .ok_or(GameError::Incomplete("Targeting Type not found"))?
        .try_into()?;

    Ok(UnitAction::ApplyEffect {
        effect,
        duration,
        targeting_type,
    })
}

fn parse_gold_action(value: &Table) -> Result<UnitAction, GameError> {
    let amount = parse_value_source(value, "amount")?;
    Ok(UnitAction::AddGold { amount })
}

fn parse_draw_action(value: &Table) -> Result<UnitAction, GameError> {
    let count = parse_value_source(value, "count")?;
    Ok(UnitAction::DrawCards { count })
}

fn parse_destroy_action(value: &Table) -> Result<UnitAction, GameError> {
    let targeting_type = value
        .get("targeting")
        .ok_or(GameError::Incomplete("Targeting Type not found"))?
        .try_into()?;

    Ok(UnitAction::DestroyCreature { targeting_type })
}

fn parse_move_action(value: &Table) -> Result<UnitAction, GameError> {
    let direction = value
        .get("direction")
        .ok_or(GameError::Incomplete("Direction not found"))?
        .try_into()
        .map_err(GameError::EngineError)?;

    Ok(UnitAction::MoveCreature { direction })
}

fn parse_modify_stats_action(value: &Table) -> Result<UnitAction, GameError> {
    let targeting_type = value
        .get("targeting")
        .ok_or(GameError::Incomplete("Targeting Type not found"))?
        .try_into()?;

    // Parse stat modifier from table
    let stat_modifier = value
        .get("modifier")
        .ok_or(GameError::Incomplete("Modifier not found"))?
        .try_into()?;

    Ok(UnitAction::ModifyStats {
        targeting_type,
        stat_modifier,
    })
}

fn parse_sequence_action(value: &Table) -> Result<UnitAction, GameError> {
    let actions_array = value
        .get("actions")
        .ok_or(GameError::Incomplete("Actions array not found"))?;

    // Parse array of actions
    let actions = parse_action_array(&actions_array)?;

    Ok(UnitAction::Sequence(actions))
}

fn parse_conditional_action(value: &Table) -> Result<UnitAction, GameError> {
    let condition = value
        .get("condition")
        .ok_or(GameError::Incomplete("Condition not found"))?
        .try_into()?;

    let on_true = value
        .get_table("on_true")
        .ok_or(GameError::Incomplete("on_true not found"))?
        .try_into()?;

    let on_false = value
        .get_table("on_false")
        .map(|t| t.try_into())
        .transpose()?;

    Ok(UnitAction::Conditional {
        condition,
        on_true: Box::new(on_true),
        on_false: on_false.map(Box::new),
    })
}

fn parse_for_each_action(value: &Table) -> Result<UnitAction, GameError> {
    let selector = value
        .get("selector")
        .ok_or(GameError::Incomplete("Selector not found"))?
        .try_into()?;

    let action = value
        .get_table("action")
        .ok_or(GameError::Incomplete("Action not found"))?
        .try_into()?;

    Ok(UnitAction::ForEach {
        selector,
        action: Box::new(action),
    })
}

fn parse_choice_action(value: &Table) -> Result<UnitAction, GameError> {
    let options_array = value
        .get("options")
        .ok_or(GameError::Incomplete("Options array not found"))?;

    let options = parse_action_array(&options_array)?;

    let chooser = value
        .get("chooser")
        .map(|c| c.try_into())
        .transpose()?
        .unwrap_or(ChoiceSource::ActivePlayer);

    Ok(UnitAction::Choice { options, chooser })
}

fn parse_value_source(table: &Table, key: &str) -> Result<ValueSource, GameError> {
    let value = table
        .get(key)
        .ok_or(GameError::Incomplete(&format!("{} not found", key)))?;

    // If it's a simple number, return constant
    if let Ok(num) = TryInto::<u16>::try_into(value) {
        return Ok(ValueSource::Constant(num));
    }

    // Otherwise, try to parse as a complex value source
    value.try_into()
}

fn parse_action_array(value: &JanetEnum) -> Result<Vec<UnitAction>, GameError> {
    // Implementation depends on your Janet array handling
    todo!("Implement action array parsing")
}
