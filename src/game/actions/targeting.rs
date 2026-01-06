use bevy::ecs::{component::Component, query::QueryFilter};

// The main selector - now with phantom types to track state
#[derive(Component, Debug, Clone, PartialEq, Eq)]

pub struct SingleTarget;
pub struct MultiTarget;

#[derive(Clone, Debug)]
pub struct CreatureTarget;
#[derive(Clone, Debug)]
pub struct TileTarget;
#[derive(Clone, Debug)]
pub struct PlayerTarget;

pub trait RulesFor<K>: Clone + std::fmt::Debug + Send + Sync + 'static {}

pub trait TargetKind: 'static {
    type FilterBase: Clone + std::fmt::Debug + Send + Sync + 'static;
    type FilterExtra: Clone + std::fmt::Debug + Send + Sync + 'static;

    type Filter: Clone + std::fmt::Debug + Send + Sync + 'static;
}

// generic composition type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RulesWithExtras<Base, Extra> {
    pub base: Base,
    pub extras: Vec<Extra>,
}

impl<Base: Default, Extra> Default for RulesWithExtras<Base, Extra> {
    fn default() -> Self {
        Self {
            base: Base::default(),
            extras: Vec::new(),
        }
    }
}

#[derive(Component, Clone, Debug)]
pub struct TargetSelector<K, C>
where
    K: TargetKind,
{
    pub(crate) selection: SelectionMethod,
    pub(crate) validation: K::Filter,
    _kind: std::marker::PhantomData<(K, C)>,
}

// Internal enums stay the same but are now private construction
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectionMethod {
    Auto(AutoSelector),
    Manual(ManualSelector),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AutoSelector {
    AllEnemyCreatures,
    AllFriendlyCreatures,
    RandomCreatures { count: u8 },
    Caster,
    TurnPlayer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManualSelector {
    ChooseCreatures { min: u8, max: u8 },
    ChooseTiles { amount: u8 },
    ChooseArea { radius: u8 },
    ChooseLine { length: u8 },
}

pub struct CreatureTargetBuilder {
    selection: AutoSelector,
    filters: CreatureFilters,
}

pub struct ManualCreatureTargetBuilder {
    min: u8,
    max: u8,
    filters: CreatureFilters,
}

pub struct TileTargetBuilder<C> {
    selector: ManualSelector,
    filters: TileFilters,
    _card: std::marker::PhantomData<C>,
}

pub struct PlayerTargetBuilder {
    selector: AutoSelector,
    filters: PlayerFilters,
}

impl<K> TargetSelector<K, ()>
where
    K: TargetKind,
{
    // creature auto
    pub fn all_enemy_creatures() -> CreatureTargetBuilder {
        CreatureTargetBuilder {
            selection: AutoSelector::AllEnemyCreatures,
            filters: CreatureFilters::default(),
        }
    }

    // creature manual
    pub fn choose_creatures(min: u8, max: u8) -> ManualCreatureTargetBuilder {
        ManualCreatureTargetBuilder {
            min,
            max,
            filters: CreatureFilters::default(),
        }
    }

    // tiles
    pub fn choose_tile() -> TileTargetBuilder<SingleTarget> {
        TileTargetBuilder {
            selector: ManualSelector::ChooseTiles { amount: 1 },
            filters: TileFilters::default(),
            _card: std::marker::PhantomData,
        }
    }

    pub fn choose_tiles(amount: u8) -> TileTargetBuilder<MultiTarget> {
        TileTargetBuilder {
            selector: ManualSelector::ChooseTiles { amount },
            filters: TileFilters::default(),
            _card: std::marker::PhantomData,
        }
    }

    pub fn choose_area(radius: u8) -> TileTargetBuilder<MultiTarget> {
        TileTargetBuilder {
            selector: ManualSelector::ChooseArea { radius },
            filters: TileFilters::default(),
            _card: std::marker::PhantomData,
        }
    }

    pub fn choose_line(length: u8) -> TileTargetBuilder<MultiTarget> {
        TileTargetBuilder {
            selector: ManualSelector::ChooseLine { length },
            filters: TileFilters::default(),
            _card: std::marker::PhantomData,
        }
    }

    // players
    pub fn turn_player() -> PlayerTargetBuilder {
        PlayerTargetBuilder {
            selector: AutoSelector::TurnPlayer,
            filters: PlayerFilters::default(),
        }
    }
}

// --- build() produces *typed* TargetSelector<Kind, Cardinality> ---

impl CreatureTargetBuilder {
    pub fn damaged_only(mut self) -> Self {
        self.filters.damaged_only = true;
        self
    }
    pub fn build(self) -> TargetSelector<CreatureTarget, MultiTarget, CreatureFilters> {
        TargetSelector {
            selection: SelectionMethod::Auto(self.selection),
            validation: ValidationRules {
                creature: Some(self.filters),
                tile: None,
                player: None,
            },
            _kind: std::marker::PhantomData,
        }
    }
}

impl ManualCreatureTargetBuilder {
    pub fn build(self) -> TargetSelector<CreatureTarget, MultiTarget, CreatureFilters> {
        TargetSelector {
            selection: SelectionMethod::Manual(ManualSelector::ChooseCreatures {
                min: self.min,
                max: self.max,
            }),
            validation: ValidationRules {
                creature: Some(self.filters),
                tile: None,
                player: None,
            },
            _kind: std::marker::PhantomData,
        }
    }
}

impl<C> TileTargetBuilder<C> {
    pub fn empty_only(mut self) -> Self {
        self.filters.empty_only = true;
        self
    }
    pub fn in_range_of_caster(mut self, range: u8) -> Self {
        self.filters.in_range_of_caster = Some(range);
        self
    }

    pub fn build(self) -> TargetSelector<TileTarget, C> {
        TargetSelector {
            selection: SelectionMethod::Manual(self.selector),
            validation: ValidationRules {
                creature: None,
                tile: Some(self.filters),
                player: None,
            },
            _kind: std::marker::PhantomData,
        }
    }
}

impl PlayerTargetBuilder {
    pub fn build(self) -> TargetSelector<PlayerTarget, SingleTarget> {
        TargetSelector {
            selection: SelectionMethod::Auto(self.selector),
            validation: ValidationRules {
                creature: None,
                tile: None,
                player: Some(self.filters),
            },
            _kind: std::marker::PhantomData,
        }
    }
}

// Filter structs with defaults
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CreatureFilters {
    pub min_health: Option<u32>,
    pub max_health: Option<u32>,
    pub health_percent: Option<(u8, u8)>,
    pub damaged_only: bool,
    pub min_attack: Option<u32>,
    pub can_attack: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreatureExtraRules {}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TileFilters {
    pub empty_only: bool,
    pub occupied_only: bool,
    pub in_range_of_caster: Option<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PlayerFilters {
    pub min_mana: Option<u32>,
    pub has_cards_in_hand: Option<u8>,
}

impl TargetKind for CreatureTarget {
    type FilterBase = CreatureFilters;
    type FilterExtra = CreatureExtraRules;

    type Filter = RulesWithExtras<Self::FilterBase, Self::FilterExtra>;
}
impl TargetKind for TileTarget {
    type FilterBase = TileFilters;
    type FilterExtra = TileExtraRules;

    type Filter = RulesWithExtras<Self::FilterBase, Self::FilterExtra>;
}
impl TargetKind for PlayerTarget {
    type FilterBase = PlayerTarget;
    type FilterExtra = PlayerExtraRules;

    type Filter = RulesWithExtras<Self::FilterBase, Self::FilterExtra>;
}
