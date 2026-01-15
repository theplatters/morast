use crate::actions::{
    action_prototype::ValueSource,
    targeting::{
        CreatureTarget, HandTarget, PlayerTarget, RulesWithExtras, TargetFilter, TileTarget,
    },
};

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

#[derive(Debug, Clone)]
pub enum CreatureExtraRules {}

#[derive(Debug, Clone, Default)]
pub struct TileFilters {
    pub empty_only: bool,
    pub occupied_only: bool,
    pub in_range_of_caster: Option<ValueSource>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TileExtraRules {}

#[derive(Debug, Clone, Default)]
pub struct PlayerFilters {
    pub min_gold: Option<ValueSource>,
    pub max_gold: Option<ValueSource>,
    pub has_cards_in_hand: Option<ValueSource>,
    pub min_health: Option<ValueSource>,
    pub max_health: Option<ValueSource>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerExtraRules {
    TookDamageLastRound,
    PlayedCardThisTurn,
}

#[derive(Clone, Debug)]
pub struct HandFilters {
    pub min_cost: Option<ValueSource>,
    pub max_cost: Option<ValueSource>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HandExtraRules {
    ExludeCreatures,
    ExcludeSpells,
    ExcludeTraps,
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
