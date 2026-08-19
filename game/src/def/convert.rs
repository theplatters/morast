//! Conversion from data-level `Def` IR to runtime ECS types.

use bevy::math::I16Vec2;

use crate::{
    actions::{
        conditions::{Condition, CreatureCondition, PlayerCondition},
        targeting::{
            AnyTargetSelector, AutoHand, AutoMultiCreature, AutoMultiTile, AutoPlayerMulti,
            AutoPlayerSingle, AutoSingleCreature, CreatureTarget, HandTarget, ManualCreature,
            ManualHand, ManualPlayer, ManualTile, MultiTarget, PlayerTarget, SelectionMethod,
            SingleTarget, TargetSelector, TileTarget,
            filters::{
                CreatureExtraRules, CreatureFilters, HandExtraRules, HandFilters,
                PlayerExtraRules, PlayerFilters, RulesWithExtras, TileExtraRules, TileFilters,
            },
        },
        value_source::ValueSource,
    },
};

use super::{
    condition::{ConditionDef, CreatureConditionDef, PlayerConditionDef},
    effect::StatModifierDef,
    selector::{CardinalityDef, FilterDef, SelectionDef, SelectorDef, SelectorKindDef},
    value::ValueDef,
    value_expr::parse_value_expr,
};

/// Errors that can occur while converting a data definition into a runtime type.
#[derive(Debug, Clone, PartialEq)]
pub enum DefError {
    InvalidValue(String),
    InvalidCondition(String),
    InvalidSelector(String),
    ValueExpr(super::value_expr::ValueExprError),
}

impl std::fmt::Display for DefError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DefError::InvalidValue(s) => write!(f, "invalid value: {}", s),
            DefError::InvalidCondition(s) => write!(f, "invalid condition: {}", s),
            DefError::InvalidSelector(s) => write!(f, "invalid selector: {}", s),
            DefError::ValueExpr(e) => write!(f, "value expression error: {}", e),
        }
    }
}

impl std::error::Error for DefError {}

impl From<DefError> for crate::error::GameError {
    fn from(value: DefError) -> Self {
        crate::error::GameError::DefError(value.to_string())
    }
}

// ============================================================================
// ValueDef -> ValueSource
// ============================================================================

impl TryFrom<&ValueDef> for ValueSource {
    type Error = DefError;

    fn try_from(value: &ValueDef) -> Result<Self, Self::Error> {
        match value {
            ValueDef::Constant(v) => Ok(ValueSource::Constant(*v)),
            ValueDef::Count(sel) => {
                let selector: AnyTargetSelector = sel.as_ref().try_into()?;
                match selector {
                    AnyTargetSelector::CreatureMulti(s) => {
                        Ok(ValueSource::count(crate::actions::targeting::MultiTargetSelector::Creature(s)))
                    }
                    AnyTargetSelector::TileMulti(s) => {
                        Ok(ValueSource::count(crate::actions::targeting::MultiTargetSelector::Tile(s)))
                    }
                    AnyTargetSelector::PlayerMulti(s) => {
                        Ok(ValueSource::count(crate::actions::targeting::MultiTargetSelector::Player(s)))
                    }
                    AnyTargetSelector::HandMulti(s) => {
                        Ok(ValueSource::count(crate::actions::targeting::MultiTargetSelector::Hand(s)))
                    }
                    _ => Err(DefError::InvalidValue(
                        "Count selector must be multi-target".into(),
                    )),
                }
            }
            ValueDef::Random { min, max } => Ok(ValueSource::Random {
                min: Box::new(min.as_ref().try_into()?),
                max: Box::new(max.as_ref().try_into()?),
            }),
            ValueDef::CreatureStat { selector, stat } => {
                let any: AnyTargetSelector = selector.as_ref().try_into()?;
                match any {
                    AnyTargetSelector::CreatureSingle(s) => Ok(ValueSource::CreatureStat {
                        selector: Box::new(s),
                        stat: *stat,
                    }),
                    _ => Err(DefError::InvalidValue(
                        "CreatureStat selector must be a single creature".into(),
                    )),
                }
            }
            ValueDef::Add(a, b) => Ok(ValueSource::Add(
                Box::new(a.as_ref().try_into()?),
                Box::new(b.as_ref().try_into()?),
            )),
            ValueDef::Sub(a, b) => Ok(ValueSource::Sub(
                Box::new(a.as_ref().try_into()?),
                Box::new(b.as_ref().try_into()?),
            )),
            ValueDef::Multiply(a, b) => Ok(ValueSource::Multiply(
                Box::new(a.as_ref().try_into()?),
                Box::new(b.as_ref().try_into()?),
            )),
            ValueDef::Divide(a, b) => Ok(ValueSource::Divide(
                Box::new(a.as_ref().try_into()?),
                Box::new(b.as_ref().try_into()?),
            )),
            ValueDef::Min(a, b) => Ok(ValueSource::Min(
                Box::new(a.as_ref().try_into()?),
                Box::new(b.as_ref().try_into()?),
            )),
            ValueDef::Max(a, b) => Ok(ValueSource::Max(
                Box::new(a.as_ref().try_into()?),
                Box::new(b.as_ref().try_into()?),
            )),
            ValueDef::Expr(s) => {
                let parsed = parse_value_expr(s).map_err(DefError::ValueExpr)?;
                Self::try_from(&parsed)
            }
        }
    }
}

// ============================================================================
// ConditionDef -> Condition
// ============================================================================

impl TryFrom<&ConditionDef> for Condition {
    type Error = DefError;

    fn try_from(value: &ConditionDef) -> Result<Self, Self::Error> {
        match value {
            ConditionDef::Always => Ok(Condition::Always),
            ConditionDef::Never => Ok(Condition::Never),
            ConditionDef::Compare { left, op, right } => Ok(Condition::Compare {
                left: left.try_into()?,
                op: *op,
                right: right.try_into()?,
            }),
            ConditionDef::HasEffect { selector, effect } => {
                let any: AnyTargetSelector = selector.try_into()?;
                match any {
                    AnyTargetSelector::TileSingle(s) => Ok(Condition::HasEffect {
                        selector: s,
                        effect: *effect,
                    }),
                    _ => Err(DefError::InvalidCondition(
                        "HasEffect requires a single tile selector".into(),
                    )),
                }
            }
            ConditionDef::Player(pc) => Ok(Condition::PlayerCondition(pc.try_into()?)),
            ConditionDef::Creature(cc) => Ok(Condition::CreatureCondition(cc.try_into()?)),
            ConditionDef::And(a, b) => Ok(Condition::And(
                Box::new(a.as_ref().try_into()?),
                Box::new(b.as_ref().try_into()?),
            )),
            ConditionDef::Or(a, b) => Ok(Condition::Or(
                Box::new(a.as_ref().try_into()?),
                Box::new(b.as_ref().try_into()?),
            )),
            ConditionDef::Not(c) => Ok(Condition::Not(Box::new(c.as_ref().try_into()?))),
        }
    }
}

impl TryFrom<&PlayerConditionDef> for PlayerCondition {
    type Error = DefError;

    fn try_from(value: &PlayerConditionDef) -> Result<Self, Self::Error> {
        match value {
            PlayerConditionDef::HasMinGold { player, amount } => {
                let any: AnyTargetSelector = player.try_into()?;
                match any {
                    AnyTargetSelector::PlayerSingle(s) => Ok(PlayerCondition::HasMinGold {
                        player: s,
                        amount: *amount,
                    }),
                    _ => Err(DefError::InvalidCondition(
                        "HasMinGold requires a single player selector".into(),
                    )),
                }
            }
            PlayerConditionDef::HasMaxGold { player, amount } => {
                let any: AnyTargetSelector = player.try_into()?;
                match any {
                    AnyTargetSelector::PlayerSingle(s) => Ok(PlayerCondition::HasMaxGold {
                        player: s,
                        amount: *amount,
                    }),
                    _ => Err(DefError::InvalidCondition(
                        "HasMaxGold requires a single player selector".into(),
                    )),
                }
            }
            PlayerConditionDef::HasMinHealth { player, amount } => {
                let any: AnyTargetSelector = player.try_into()?;
                match any {
                    AnyTargetSelector::PlayerSingle(s) => Ok(PlayerCondition::HasMinHealt {
                        player: s,
                        amount: *amount,
                    }),
                    _ => Err(DefError::InvalidCondition(
                        "HasMinHealth requires a single player selector".into(),
                    )),
                }
            }
            PlayerConditionDef::HasMaxHealth { player, amount } => {
                let any: AnyTargetSelector = player.try_into()?;
                match any {
                    AnyTargetSelector::PlayerSingle(s) => Ok(PlayerCondition::HasMaxHealth {
                        player: s,
                        amount: *amount,
                    }),
                    _ => Err(DefError::InvalidCondition(
                        "HasMaxHealth requires a single player selector".into(),
                    )),
                }
            }
            PlayerConditionDef::DeckHasCards { player, count } => {
                let any: AnyTargetSelector = player.try_into()?;
                match any {
                    AnyTargetSelector::PlayerSingle(s) => Ok(PlayerCondition::DeckHasCards {
                        player: s,
                        count: *count,
                    }),
                    _ => Err(DefError::InvalidCondition(
                        "DeckHasCards requires a single player selector".into(),
                    )),
                }
            }
            PlayerConditionDef::SelectorHasCount { selector, count } => {
                let any: AnyTargetSelector = selector.try_into()?;
                match any {
                    AnyTargetSelector::PlayerSingleMulti(s) => Ok(PlayerCondition::SelectorHasCount {
                        selector: s,
                        count: *count,
                    }),
                    AnyTargetSelector::PlayerMulti(s) => Ok(PlayerCondition::SelectorHasCount {
                        selector: s.into(),
                        count: *count,
                    }),
                    AnyTargetSelector::PlayerSingle(s) => Ok(PlayerCondition::SelectorHasCount {
                        selector: s.into(),
                        count: *count,
                    }),
                    _ => Err(DefError::InvalidCondition(
                        "SelectorHasCount requires a player selector".into(),
                    )),
                }
            }
        }
    }
}

impl TryFrom<&CreatureConditionDef> for CreatureCondition {
    type Error = DefError;

    fn try_from(value: &CreatureConditionDef) -> Result<Self, Self::Error> {
        match value {
            CreatureConditionDef::NotMoved { creature } => {
                let any: AnyTargetSelector = creature.try_into()?;
                match any {
                    AnyTargetSelector::CreatureSingleMulti(s) => Ok(CreatureCondition::NotMoved {
                        creature: s,
                    }),
                    AnyTargetSelector::CreatureMulti(s) => Ok(CreatureCondition::NotMoved {
                        creature: s.into(),
                    }),
                    AnyTargetSelector::CreatureSingle(s) => Ok(CreatureCondition::NotMoved {
                        creature: s.into(),
                    }),
                    _ => Err(DefError::InvalidCondition(
                        "NotMoved requires a creature selector".into(),
                    )),
                }
            }
            CreatureConditionDef::FullHealth { creature } => {
                let any: AnyTargetSelector = creature.try_into()?;
                match any {
                    AnyTargetSelector::CreatureSingleMulti(s) => Ok(CreatureCondition::FullHealth {
                        creature: s,
                    }),
                    AnyTargetSelector::CreatureMulti(s) => Ok(CreatureCondition::FullHealth {
                        creature: s.into(),
                    }),
                    AnyTargetSelector::CreatureSingle(s) => Ok(CreatureCondition::FullHealth {
                        creature: s.into(),
                    }),
                    _ => Err(DefError::InvalidCondition(
                        "FullHealth requires a creature selector".into(),
                    )),
                }
            }
            CreatureConditionDef::SelectorHasCount { selector, count } => {
                let any: AnyTargetSelector = selector.try_into()?;
                match any {
                    AnyTargetSelector::CreatureSingleMulti(s) => {
                        Ok(CreatureCondition::SelectorHasCount {
                            selector: s,
                            count: *count,
                        })
                    }
                    AnyTargetSelector::CreatureMulti(s) => {
                        Ok(CreatureCondition::SelectorHasCount {
                            selector: s.into(),
                            count: *count,
                        })
                    }
                    AnyTargetSelector::CreatureSingle(s) => {
                        Ok(CreatureCondition::SelectorHasCount {
                            selector: s.into(),
                            count: *count,
                        })
                    }
                    _ => Err(DefError::InvalidCondition(
                        "SelectorHasCount requires a creature selector".into(),
                    )),
                }
            }
        }
    }
}

// ============================================================================
// SelectorDef -> AnyTargetSelector
// ============================================================================

impl TryFrom<&SelectorDef> for AnyTargetSelector {
    type Error = DefError;

    fn try_from(value: &SelectorDef) -> Result<Self, Self::Error> {
        match value.kind {
            SelectorKindDef::Creature => build_creature_selector(value),
            SelectorKindDef::Tile => build_tile_selector(value),
            SelectorKindDef::Player => build_player_selector(value),
            SelectorKindDef::Hand => build_hand_selector(value),
        }
    }
}

fn build_creature_selector(sel: &SelectorDef) -> Result<AnyTargetSelector, DefError> {
    let filter = build_creature_filter(&sel.filters)?;

    match sel.cardinality {
        CardinalityDef::Single => match &sel.selection {
            SelectionDef::Strongest => {
                let sel: TargetSelector<CreatureTarget, SingleTarget> = TargetSelector::new(
                    SelectionMethod::from(AutoSingleCreature::Strongest),
                    filter,
                );
                Ok(sel.into())
            }
            SelectionDef::Caster => {
                let sel: TargetSelector<CreatureTarget, SingleTarget> = TargetSelector::new(
                    SelectionMethod::from(AutoSingleCreature::Caster),
                    filter,
                );
                Ok(sel.into())
            }
            SelectionDef::CurrentTarget => {
                let sel: TargetSelector<CreatureTarget, SingleTarget> = TargetSelector::new(
                    SelectionMethod::from(AutoSingleCreature::CurrentTarget),
                    filter,
                );
                Ok(sel.into())
            }
            SelectionDef::ChooseCreatures { .. }
            | SelectionDef::MaxNFriendly { .. }
            | SelectionDef::ExactlyNFriendly { .. } => {
                build_manual_creature(&sel.selection, filter, false)
            }
            _ => Err(DefError::InvalidSelector(format!(
                "{:?} is not a valid single-creature selection",
                sel.selection
            ))),
        },
        CardinalityDef::Multi | CardinalityDef::Any => match &sel.selection {
            SelectionDef::AllEnemy => {
                let sel: TargetSelector<CreatureTarget, MultiTarget> = TargetSelector::new(
                    SelectionMethod::from(AutoMultiCreature::AllEnemy),
                    filter,
                );
                Ok(sel.into())
            }
            SelectionDef::AllFriendly => {
                let sel: TargetSelector<CreatureTarget, MultiTarget> = TargetSelector::new(
                    SelectionMethod::from(AutoMultiCreature::AllFriendly),
                    filter,
                );
                Ok(sel.into())
            }
            SelectionDef::RandomCreatures { count } => {
                let sel: TargetSelector<CreatureTarget, MultiTarget> = TargetSelector::new(
                    SelectionMethod::from(AutoMultiCreature::Random {
                        count: count.try_into()?,
                    }),
                    filter,
                );
                Ok(sel.into())
            }
            SelectionDef::ChooseCreatures { .. }
            | SelectionDef::MaxNFriendly { .. }
            | SelectionDef::ExactlyNFriendly { .. } => {
                build_manual_creature(&sel.selection, filter, true)
            }
            SelectionDef::Caster | SelectionDef::CurrentTarget | SelectionDef::Strongest => {
                // Treat single-only selections as single even when cardinality is Any.
                let single_sel = SelectorDef {
                    kind: SelectorKindDef::Creature,
                    cardinality: CardinalityDef::Single,
                    selection: sel.selection.clone(),
                    filters: sel.filters.clone(),
                };
                build_creature_selector(&single_sel)
            }
            _ => Err(DefError::InvalidSelector(format!(
                "{:?} is not a valid multi-creature selection",
                sel.selection
            ))),
        },
    }
}

fn build_manual_creature(
    selection: &SelectionDef,
    filter: RulesWithExtras<CreatureFilters, CreatureExtraRules>,
    multi: bool,
) -> Result<AnyTargetSelector, DefError> {
    let (min, max) = match selection {
        SelectionDef::ChooseCreatures { min, max } => {
            (min.try_into()?, max.try_into()?)
        }
        SelectionDef::MaxNFriendly { count: min } => {
            (min.try_into()?, ValueSource::Constant(u16::MAX))
        }
        SelectionDef::ExactlyNFriendly { count: min } => {
            let min: ValueSource = min.try_into()?;
            (min.clone(), min)
        }
        _ => {
            return Err(DefError::InvalidSelector(
                "expected a manual creature selection".into(),
            ))
        }
    };
    if multi {
        let sel: TargetSelector<CreatureTarget, MultiTarget> = TargetSelector::new(
            SelectionMethod::from(ManualCreature::Choose { min, max }),
            filter,
        );
        Ok(sel.into())
    } else {
        let sel: TargetSelector<CreatureTarget, SingleTarget> = TargetSelector::new(
            SelectionMethod::from(ManualCreature::Choose { min, max }),
            filter,
        );
        Ok(sel.into())
    }
}

fn build_tile_selector(sel: &SelectorDef) -> Result<AnyTargetSelector, DefError> {
    let filter = build_tile_filter(&sel.filters)?;

    match sel.cardinality {
        CardinalityDef::Multi | CardinalityDef::Any => match &sel.selection {
            SelectionDef::AllTiles => {
                let sel: TargetSelector<TileTarget, MultiTarget> = TargetSelector::new(
                    SelectionMethod::from(AutoMultiTile::AllTiles),
                    filter,
                );
                Ok(sel.into())
            }
            SelectionDef::RadiusAroundCaster { radius } => {
                let sel: TargetSelector<TileTarget, MultiTarget> = TargetSelector::new(
                    SelectionMethod::from(AutoMultiTile::RadiusAroundCaster { radius: *radius }),
                    filter,
                );
                Ok(sel.into())
            }
            SelectionDef::ChooseTiles { amount } => {
                let sel: TargetSelector<TileTarget, MultiTarget> = TargetSelector::new(
                    SelectionMethod::from(ManualTile::ChooseTiles {
                        amount: amount.try_into()?,
                    }),
                    filter,
                );
                Ok(sel.into())
            }
            SelectionDef::ChooseArea { radius } => {
                let sel: TargetSelector<TileTarget, MultiTarget> = TargetSelector::new(
                    SelectionMethod::from(ManualTile::ChooseArea {
                        radius: radius.try_into()?,
                    }),
                    filter,
                );
                Ok(sel.into())
            }
            _ => Err(DefError::InvalidSelector(format!(
                "{:?} is not a valid tile selection",
                sel.selection
            ))),
        },
        CardinalityDef::Single => match &sel.selection {
            SelectionDef::ChooseTiles { amount } => {
                let sel: TargetSelector<TileTarget, SingleTarget> = TargetSelector::new(
                    SelectionMethod::from(ManualTile::ChooseTiles {
                        amount: amount.try_into()?,
                    }),
                    filter,
                );
                Ok(sel.into())
            }
            SelectionDef::ChooseArea { radius } => {
                let sel: TargetSelector<TileTarget, SingleTarget> = TargetSelector::new(
                    SelectionMethod::from(ManualTile::ChooseArea {
                        radius: radius.try_into()?,
                    }),
                    filter,
                );
                Ok(sel.into())
            }
            _ => Err(DefError::InvalidSelector(format!(
                "{:?} is not a valid single-tile selection",
                sel.selection
            ))),
        },
    }
}

fn build_player_selector(sel: &SelectorDef) -> Result<AnyTargetSelector, DefError> {
    let filter = build_player_filter(&sel.filters)?;

    match sel.cardinality {
        CardinalityDef::Single | CardinalityDef::Any => match &sel.selection {
            SelectionDef::TurnPlayer => {
                let sel: TargetSelector<PlayerTarget, SingleTarget> = TargetSelector::new(
                    SelectionMethod::from(AutoPlayerSingle::TurnPlayer),
                    filter,
                );
                Ok(sel.into())
            }
            SelectionDef::NonTurnPlayer => {
                let sel: TargetSelector<PlayerTarget, SingleTarget> = TargetSelector::new(
                    SelectionMethod::from(AutoPlayerSingle::NonTurnPlayer),
                    filter,
                );
                Ok(sel.into())
            }
            SelectionDef::Owner => {
                let sel: TargetSelector<PlayerTarget, SingleTarget> = TargetSelector::new(
                    SelectionMethod::from(AutoPlayerSingle::Owner),
                    filter,
                );
                Ok(sel.into())
            }
            SelectionDef::ChoosePlayer => {
                let sel: TargetSelector<PlayerTarget, SingleTarget> = TargetSelector::new(
                    SelectionMethod::from(ManualPlayer),
                    filter,
                );
                Ok(sel.into())
            }
            SelectionDef::AllPlayers => {
                let sel: TargetSelector<PlayerTarget, MultiTarget> = TargetSelector::new(
                    SelectionMethod::from(AutoPlayerMulti),
                    filter,
                );
                Ok(sel.into())
            }
            _ => Err(DefError::InvalidSelector(format!(
                "{:?} is not a valid player selection",
                sel.selection
            ))),
        },
        CardinalityDef::Multi => match &sel.selection {
            SelectionDef::AllPlayers => {
                let sel: TargetSelector<PlayerTarget, MultiTarget> = TargetSelector::new(
                    SelectionMethod::from(AutoPlayerMulti),
                    filter,
                );
                Ok(sel.into())
            }
            _ => Err(DefError::InvalidSelector(format!(
                "{:?} is not a valid multi-player selection",
                sel.selection
            ))),
        },
    }
}

fn build_hand_selector(sel: &SelectorDef) -> Result<AnyTargetSelector, DefError> {
    let filter = build_hand_filter(&sel.filters)?;

    match sel.cardinality {
        CardinalityDef::Single | CardinalityDef::Multi | CardinalityDef::Any => match &sel.selection {
            SelectionDef::AllCards => {
                let sel: TargetSelector<HandTarget, MultiTarget> = TargetSelector::new(
                    SelectionMethod::from(AutoHand::AllCards),
                    filter,
                );
                Ok(sel.into())
            }
            SelectionDef::AllCreatures => {
                let sel: TargetSelector<HandTarget, MultiTarget> = TargetSelector::new(
                    SelectionMethod::from(AutoHand::AllCreatures),
                    filter,
                );
                Ok(sel.into())
            }
            SelectionDef::AllSpells => {
                let sel: TargetSelector<HandTarget, MultiTarget> = TargetSelector::new(
                    SelectionMethod::from(AutoHand::AllSpells),
                    filter,
                );
                Ok(sel.into())
            }
            SelectionDef::AllTraps => {
                let sel: TargetSelector<HandTarget, MultiTarget> = TargetSelector::new(
                    SelectionMethod::from(AutoHand::AllTraps),
                    filter,
                );
                Ok(sel.into())
            }
            SelectionDef::ChooseCards { count } => {
                let sel: TargetSelector<HandTarget, SingleTarget> = TargetSelector::new(
                    SelectionMethod::from(ManualHand {
                        count: count.try_into()?,
                    }),
                    filter,
                );
                Ok(sel.into())
            }
            _ => Err(DefError::InvalidSelector(format!(
                "{:?} is not a valid hand selection",
                sel.selection
            ))),
        },
    }
}

// ============================================================================
// Filters
// ============================================================================

fn build_creature_filter(filters: &[FilterDef]) -> Result<RulesWithExtras<CreatureFilters, CreatureExtraRules>, DefError> {
    let mut base = CreatureFilters::default();
    let extras = Vec::new();
    for f in filters {
        match f {
            FilterDef::MinHealth(v) => base.min_health = Some(v.try_into()?),
            FilterDef::MaxHealth(v) => base.max_health = Some(v.try_into()?),
            FilterDef::HealthPercent { min, max } => {
                base.health_percent = Some((min.try_into()?, max.try_into()?))
            }
            FilterDef::DamagedOnly => base.damaged_only = true,
            FilterDef::MinAttack(v) => base.min_attack = Some(v.try_into()?),
            FilterDef::CanAttack(v) => base.can_attack = Some(*v),
            _ => return Err(DefError::InvalidSelector(format!(
                "{:?} is not a valid creature filter", f
            ))),
        }
    }
    Ok(RulesWithExtras { base, extras })
}

fn build_tile_filter(filters: &[FilterDef]) -> Result<RulesWithExtras<TileFilters, TileExtraRules>, DefError> {
    let mut base = TileFilters::default();
    let extras = Vec::new();
    for f in filters {
        match f {
            FilterDef::EmptyOnly => base.empty_only = true,
            FilterDef::OccupiedOnly => base.occupied_only = true,
            FilterDef::InRangeOfCaster(v) => base.in_range_of_caster = Some(v.try_into()?),
            _ => return Err(DefError::InvalidSelector(format!(
                "{:?} is not a valid tile filter", f
            ))),
        }
    }
    Ok(RulesWithExtras { base, extras })
}

fn build_player_filter(filters: &[FilterDef]) -> Result<RulesWithExtras<PlayerFilters, PlayerExtraRules>, DefError> {
    let mut base = PlayerFilters::default();
    let mut extras = Vec::new();
    for f in filters {
        match f {
            FilterDef::MinGold(v) => base.min_gold = Some(v.try_into()?),
            FilterDef::MaxGold(v) => base.max_gold = Some(v.try_into()?),
            FilterDef::HasCardsInHand(v) => base.has_cards_in_hand = Some(v.try_into()?),
            FilterDef::MinPlayerHealth(v) => base.min_health = Some(v.try_into()?),
            FilterDef::MaxPlayerHealth(v) => base.max_health = Some(v.try_into()?),
            FilterDef::TookDamageLastRound => extras.push(PlayerExtraRules::TookDamageLastRound),
            FilterDef::PlayedCardThisTurn => extras.push(PlayerExtraRules::PlayedCardThisTurn),
            _ => return Err(DefError::InvalidSelector(format!(
                "{:?} is not a valid player filter", f
            ))),
        }
    }
    Ok(RulesWithExtras { base, extras })
}

fn build_hand_filter(filters: &[FilterDef]) -> Result<RulesWithExtras<HandFilters, HandExtraRules>, DefError> {
    let mut base = HandFilters::default();
    let mut extras = Vec::new();
    for f in filters {
        match f {
            FilterDef::MinCost(v) => base.min_cost = Some(v.try_into()?),
            FilterDef::MaxCost(v) => base.max_cost = Some(v.try_into()?),
            FilterDef::ExcludeCreatures => extras.push(HandExtraRules::ExludeCreatures),
            FilterDef::ExcludeSpells => extras.push(HandExtraRules::ExcludeSpells),
            FilterDef::ExcludeTraps => extras.push(HandExtraRules::ExcludeTraps),
            _ => return Err(DefError::InvalidSelector(format!(
                "{:?} is not a valid hand filter", f
            ))),
        }
    }
    Ok(RulesWithExtras { base, extras })
}

// ============================================================================
// PatternDef -> Vec<I16Vec2>
// ============================================================================

impl From<&super::card::PatternDef> for Vec<I16Vec2> {
    fn from(value: &super::card::PatternDef) -> Self {
        match value {
            super::card::PatternDef::Offsets(list) => list
                .iter()
                .map(|&[x, y]| I16Vec2::new(x, y))
                .collect(),
            super::card::PatternDef::Plus(n) => {
                let mut out = Vec::new();
                for i in 1..=*n {
                    let i = i as i16;
                    out.push(I16Vec2::new(i, 0));
                    out.push(I16Vec2::new(-i, 0));
                    out.push(I16Vec2::new(0, i));
                    out.push(I16Vec2::new(0, -i));
                }
                out
            }
            super::card::PatternDef::Cross(n) => {
                let mut out = Vec::new();
                for i in 1..=*n {
                    let i = i as i16;
                    out.push(I16Vec2::new(i, i));
                    out.push(I16Vec2::new(-i, i));
                    out.push(I16Vec2::new(i, -i));
                    out.push(I16Vec2::new(-i, -i));
                }
                out
            }
            super::card::PatternDef::Union(parts) => parts
                .iter()
                .flat_map(|p| Vec::<I16Vec2>::from(p))
                .collect(),
        }
    }
}

// ============================================================================
// StatModifierDef -> StatModifier
// ============================================================================

impl From<&StatModifierDef> for crate::actions::value_source::StatModifier {
    fn from(value: &StatModifierDef) -> Self {
        match *value {
            StatModifierDef::Attack(v) => crate::actions::value_source::StatModifier::Attack(v),
            StatModifierDef::Health(v) => crate::actions::value_source::StatModifier::Health(v),
            StatModifierDef::MaxHealth(v) => crate::actions::value_source::StatModifier::MaxHealth(v),
            StatModifierDef::Speed(v) => crate::actions::value_source::StatModifier::Speed(v),
            StatModifierDef::Both { attack, health } => {
                crate::actions::value_source::StatModifier::Both { attack, health }
            }
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actions::conditions::CompareOp;

    #[test]
    fn value_expr_conversion() {
        let def = ValueDef::Expr("attack(caster) * 2 + 1".into());
        let source = ValueSource::try_from(&def).unwrap();
        assert!(matches!(source, ValueSource::Add(_, _)));
    }

    #[test]
    fn selector_creature_single_caster() {
        let sel = SelectorDef {
            kind: SelectorKindDef::Creature,
            cardinality: CardinalityDef::Single,
            selection: SelectionDef::Caster,
            filters: vec![],
        };
        let any = AnyTargetSelector::try_from(&sel).unwrap();
        assert!(matches!(any, AnyTargetSelector::CreatureSingle(_)));
    }

    #[test]
    fn selector_tile_all_tiles() {
        let sel = SelectorDef {
            kind: SelectorKindDef::Tile,
            cardinality: CardinalityDef::Multi,
            selection: SelectionDef::AllTiles,
            filters: vec![],
        };
        let any = AnyTargetSelector::try_from(&sel).unwrap();
        assert!(matches!(any, AnyTargetSelector::TileMulti(_)));
    }

    #[test]
    fn condition_compare() {
        let def = ConditionDef::Compare {
            left: ValueDef::Constant(1),
            op: CompareOp::Greater,
            right: ValueDef::Constant(0),
        };
        let cond = Condition::try_from(&def).unwrap();
        assert!(matches!(cond, Condition::Compare { .. }));
    }

    #[test]
    fn pattern_plus() {
        let pat = super::super::card::PatternDef::Plus(1);
        let offsets: Vec<I16Vec2> = (&pat).into();
        assert_eq!(offsets.len(), 4);
    }
}
