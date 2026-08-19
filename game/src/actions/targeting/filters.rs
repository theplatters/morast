use bevy::ecs::{entity::Entity, query::With, system::Query};
use bevy::log::warn;

use crate::{
    actions::{
        targeting::{
            CreatureTarget, HandTarget, PlayerTarget, TargetFilter, TileTarget,
            systems::{CreatureQuery, HandQuery, PlayerQuery, TileQuery},
        },
        value_source::{ValueEvalParams, ValueSource},
    },
    board::{effect::EffectType, tile::Tile},

};

pub trait IsFilter {
    fn validate(
        &self,
        context: &mut ValueEvalParams,
        caster: Entity,
        candidate: Entity,
    ) -> bool;
}

// generic composition type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RulesWithExtras<Base: IsFilter, Extra: IsFilter> {
    pub base: Base,
    pub extras: Vec<Extra>,
}

impl<B: IsFilter, E: IsFilter> IsFilter for RulesWithExtras<B, E> {
    fn validate(
        &self,
        context: &mut ValueEvalParams,
        caster: Entity,
        candidate: Entity,
    ) -> bool {
        self.base.validate(context, caster, candidate)
            && self.extras.validate(context, caster, candidate)
    }
}

impl<Base: IsFilter, Extra: IsFilter> RulesWithExtras<Base, Extra> {
    pub fn from_base(base: Base) -> Self {
        Self {
            base,
            extras: Vec::new(),
        }
    }
}

impl<Base: Default + IsFilter, Extra: IsFilter> Default for RulesWithExtras<Base, Extra> {
    fn default() -> Self {
        Self {
            base: Base::default(),
            extras: Vec::new(),
        }
    }
}

impl<T: IsFilter> IsFilter for Vec<T> {
    fn validate(
        &self,
        context: &mut ValueEvalParams,
        caster: Entity,
        candidate: Entity,
    ) -> bool {
        self.iter().all(|l| l.validate(context, caster, candidate))
    }
}

// Filter structs with defaults
#[derive(Debug, Clone, Default)]
pub struct CreatureFilters {
    pub min_health: Option<ValueSource>,
    pub max_health: Option<ValueSource>,
    pub health_percent: Option<(ValueSource, ValueSource)>,
    pub damaged_only: bool,
    pub min_attack: Option<ValueSource>,
    pub can_attack: Option<bool>,
}

impl IsFilter for CreatureFilters {
    fn validate(
        &self,
        context: &mut ValueEvalParams,
        caster: Entity,
        candidate: Entity,
    ) -> bool {
        let Ok(creature) = context.creatures.get(candidate) else {
            return false;
        };

        if let Some(min) = &self.min_health {
            if creature.health.value() < min.eval(context, caster) {
                return false;
            }
        }

        if let Some(max) = &self.max_health {
            if creature.health.value() > max.eval(context, caster) {
                return false;
            }
        }

        if let Some((min_pct, max_pct)) = &self.health_percent {
            let max = creature.health.value().max(1);
            let current = creature.current_defense.0;
            let pct = (current * 100) / max;
            let min_val = min_pct.eval(context, caster);
            let max_val = max_pct.eval(context, caster);
            if pct < min_val || pct > max_val {
                return false;
            }
        }

        if self.damaged_only && creature.current_defense.0 >= creature.health.value() {
            return false;
        }

        if let Some(min) = &self.min_attack {
            if creature.current_atttack.0 < min.eval(context, caster) {
                return false;
            }
        }

        if let Some(can_attack) = self.can_attack {
            // There is no dedicated CanAttack component yet. A missing flag is
            // treated as "can attack", so only a positive filter passes.
            if !can_attack {
                return false;
            }
        }

        true
    }
}

#[derive(Debug, Clone)]
pub enum CreatureExtraRules {}
impl IsFilter for CreatureExtraRules {
    fn validate(
        &self,
        _context: &mut ValueEvalParams,
        _caster: Entity,
        _candidate: Entity,
    ) -> bool {
        true
    }
}

#[derive(Debug, Clone, Default)]
pub struct TileFilters {
    pub empty_only: bool,
    pub occupied_only: bool,
    pub in_range_of_caster: Option<ValueSource>,
}

impl IsFilter for TileFilters {
    fn validate(
        &self,
        context: &mut ValueEvalParams,
        caster: Entity,
        candidate: Entity,
    ) -> bool {
        let Ok(tile) = context.tiles.get(candidate) else {
            return false;
        };

        if self.empty_only && tile.occupant.is_some() {
            return false;
        }
        if self.occupied_only && tile.occupant.is_none() {
            return false;
        }

        if let Some(range_source) = &self.in_range_of_caster {
            let range = range_source.eval(context, caster);

            let Ok(caster_creature) = context.creatures.get(caster) else {
                return false;
            };
            let Ok(caster_tile) = context.tiles.get(caster_creature.position.position) else {
                return false;
            };

            let caster_pos = caster_tile.position.0;
            let candidate_pos = tile.position.0;
            // Manhattan distance; could be switched to Chebyshev if the design doc prefers it.
            let distance = ((caster_pos.x as i32) - (candidate_pos.x as i32)).abs()
                + ((caster_pos.y as i32) - (candidate_pos.y as i32)).abs();

            if distance > range as i32 {
                return false;
            }
        }

        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TileExtraRules {}

impl IsFilter for TileExtraRules {
    fn validate(
        &self,
        _context: &mut ValueEvalParams,
        _caster: Entity,
        _candidate: Entity,
    ) -> bool {
        true
    }
}

#[derive(Debug, Clone, Default)]
pub struct PlayerFilters {
    pub min_gold: Option<ValueSource>,
    pub max_gold: Option<ValueSource>,
    pub has_cards_in_hand: Option<ValueSource>,
    pub min_health: Option<ValueSource>,
    pub max_health: Option<ValueSource>,
}

impl IsFilter for PlayerFilters {
    fn validate(
        &self,
        context: &mut ValueEvalParams,
        caster: Entity,
        candidate: Entity,
    ) -> bool {
        let Ok(player) = context.player.get(candidate) else {
            return false;
        };

        if let Some(min) = &self.min_gold {
            if player.resources.gold < min.eval(context, caster) {
                return false;
            }
        }

        if let Some(max) = &self.max_gold {
            if player.resources.gold > max.eval(context, caster) {
                return false;
            }
        }

        if let Some(min_cards) = &self.has_cards_in_hand {
            let count = context
                .hand
                .iter()
                .filter(|card| card.in_hand.parent == candidate)
                .count() as u16;
            if count < min_cards.eval(context, caster) {
                return false;
            }
        }

        if let Some(min) = &self.min_health {
            if player.resources.health < min.eval(context, caster) {
                return false;
            }
        }

        if let Some(max) = &self.max_health {
            if player.resources.health > max.eval(context, caster) {
                return false;
            }
        }

        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerExtraRules {
    TookDamageLastRound,
    PlayedCardThisTurn,
}

impl IsFilter for PlayerExtraRules {
    fn validate(
        &self,
        _context: &mut ValueEvalParams,
        _caster: Entity,
        _candidate: Entity,
    ) -> bool {
        warn!("PlayerExtraRules::{:?} not yet tracked; treating as false", self);
        false
    }
}

#[derive(Clone, Debug, Default)]
pub struct HandFilters {
    pub min_cost: Option<ValueSource>,
    pub max_cost: Option<ValueSource>,
}

impl IsFilter for HandFilters {
    fn validate(
        &self,
        context: &mut ValueEvalParams,
        caster: Entity,
        candidate: Entity,
    ) -> bool {
        let Ok(card) = context.hand.get(candidate) else {
            return false;
        };

        let cost = card.cost.map(|c| c.value).unwrap_or(0);

        if let Some(min) = &self.min_cost {
            if cost < min.eval(context, caster) {
                return false;
            }
        }

        if let Some(max) = &self.max_cost {
            if cost > max.eval(context, caster) {
                return false;
            }
        }

        true
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HandExtraRules {
    ExludeCreatures,
    ExcludeSpells,
    ExcludeTraps,
}

impl IsFilter for HandExtraRules {
    fn validate(
        &self,
        context: &mut ValueEvalParams,
        _caster: Entity,
        candidate: Entity,
    ) -> bool {
        let Ok(card) = context.hand.get(candidate) else {
            return false;
        };

        match self {
            // A filter returns false when the candidate matches the excluded type.
            HandExtraRules::ExludeCreatures => card.creature.is_none(),
            HandExtraRules::ExcludeSpells => card.spell.is_none(),
            HandExtraRules::ExcludeTraps => card.trap.is_none(),
        }
    }
}

impl TargetFilter for CreatureTarget {
    type FilterBase = CreatureFilters;
    type FilterExtra = CreatureExtraRules;
    type Filter = RulesWithExtras<Self::FilterBase, Self::FilterExtra>;
}

impl TargetFilter for TileTarget {
    type FilterBase = TileFilters;
    type FilterExtra = TileExtraRules;
    type Filter = RulesWithExtras<Self::FilterBase, Self::FilterExtra>;
}
impl TargetFilter for PlayerTarget {
    type FilterBase = PlayerFilters;
    type FilterExtra = PlayerExtraRules;
    type Filter = RulesWithExtras<Self::FilterBase, Self::FilterExtra>;
}

impl TargetFilter for HandTarget {
    type FilterBase = HandFilters;
    type FilterExtra = HandExtraRules;
    type Filter = RulesWithExtras<Self::FilterBase, Self::FilterExtra>;
}

#[derive(bevy::ecs::system::SystemParam)]
pub struct FilterParams<'w, 's> {
    pub creatures: Query<'w, 's, CreatureQuery>,
    pub tiles: Query<'w, 's, TileQuery, With<Tile>>,
    pub hand: Query<'w, 's, HandQuery>,
    pub player: Query<'w, 's, PlayerQuery>,
    pub effects: Query<'w, 's, &'static EffectType>,
}
