use bevy::ecs::{component::Component, query::QueryFilter};

// The main selector - now with phantom types to track state
#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct TargetSelector {
    pub(crate) selection: SelectionMethod,
    pub(crate) validation: ValidationRules,
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

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ValidationRules {
    pub creature: Option<CreatureFilters>,
    pub tile: Option<TileFilters>,
    pub player: Option<PlayerFilters>,
}

// Type-safe builders using phantom types
pub struct CreatureTargetBuilder {
    selection: AutoSelector,
    filters: CreatureFilters,
}

pub struct ManualCreatureTargetBuilder {
    min: u8,
    max: u8,
    filters: CreatureFilters,
}

pub struct TileTargetBuilder {
    selector: ManualSelector,
    filters: TileFilters,
}

pub struct PlayerTargetBuilder {
    selector: AutoSelector,
    filters: PlayerFilters,
}

// Entry points - the only way to construct selectors
impl TargetSelector {
    // === Auto Creature Selection ===
    pub fn all_enemy_creatures() -> CreatureTargetBuilder {
        CreatureTargetBuilder {
            selection: AutoSelector::AllEnemyCreatures,
            filters: CreatureFilters::default(),
        }
    }

    pub fn all_friendly_creatures() -> CreatureTargetBuilder {
        CreatureTargetBuilder {
            selection: AutoSelector::AllFriendlyCreatures,
            filters: CreatureFilters::default(),
        }
    }

    pub fn random_creatures(count: u8) -> CreatureTargetBuilder {
        CreatureTargetBuilder {
            selection: AutoSelector::RandomCreatures { count },
            filters: CreatureFilters::default(),
        }
    }

    // === Manual Creature Selection ===
    pub fn choose_creatures(min: u8, max: u8) -> ManualCreatureTargetBuilder {
        ManualCreatureTargetBuilder {
            min,
            max,
            filters: CreatureFilters::default(),
        }
    }

    // === Tile Selection ===
    pub fn choose_tiles(amount: u8) -> TileTargetBuilder {
        TileTargetBuilder {
            selector: ManualSelector::ChooseTiles { amount },
            filters: TileFilters::default(),
        }
    }

    pub fn choose_area(radius: u8) -> TileTargetBuilder {
        TileTargetBuilder {
            selector: ManualSelector::ChooseArea { radius },
            filters: TileFilters::default(),
        }
    }

    pub fn choose_line(length: u8) -> TileTargetBuilder {
        TileTargetBuilder {
            selector: ManualSelector::ChooseLine { length },
            filters: TileFilters::default(),
        }
    }

    // === Player Selection ===
    pub fn turn_player() -> PlayerTargetBuilder {
        PlayerTargetBuilder {
            selector: AutoSelector::TurnPlayer,
            filters: PlayerFilters::default(),
        }
    }

    pub fn caster() -> PlayerTargetBuilder {
        PlayerTargetBuilder {
            selector: AutoSelector::Caster,
            filters: PlayerFilters::default(),
        }
    }

    pub fn none() -> Self {
        TargetSelector {
            selection: SelectionMethod::Auto(AutoSelector::AllEnemyCreatures), // dummy
            validation: ValidationRules::default(),
        }
    }
}

// === Creature Builder Methods ===
impl CreatureTargetBuilder {
    pub fn min_health(mut self, health: u32) -> Self {
        self.filters.min_health = Some(health);
        self
    }

    pub fn max_health(mut self, health: u32) -> Self {
        self.filters.max_health = Some(health);
        self
    }

    pub fn health_percent(mut self, min: u8, max: u8) -> Self {
        self.filters.health_percent = Some((min, max));
        self
    }

    pub fn damaged_only(mut self) -> Self {
        self.filters.damaged_only = true;
        self
    }

    pub fn min_attack(mut self, attack: u32) -> Self {
        self.filters.min_attack = Some(attack);
        self
    }

    pub fn can_attack(mut self) -> Self {
        self.filters.can_attack = Some(true);
        self
    }

    pub fn build(self) -> TargetSelector {
        TargetSelector {
            selection: SelectionMethod::Auto(self.selection),
            validation: ValidationRules {
                creature: Some(self.filters),
                tile: None,
                player: None,
            },
        }
    }
}

// === Manual Creature Builder Methods ===
impl ManualCreatureTargetBuilder {
    pub fn min_health(mut self, health: u32) -> Self {
        self.filters.min_health = Some(health);
        self
    }

    pub fn max_health(mut self, health: u32) -> Self {
        self.filters.max_health = Some(health);
        self
    }

    pub fn damaged_only(mut self) -> Self {
        self.filters.damaged_only = true;
        self
    }

    // ... other creature filters

    pub fn build(self) -> TargetSelector {
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
        }
    }
}

// === Tile Builder Methods ===
impl TileTargetBuilder {
    pub fn empty_only(mut self) -> Self {
        self.filters.empty_only = true;
        self
    }

    pub fn occupied_only(mut self) -> Self {
        self.filters.occupied_only = true;
        self
    }

    pub fn in_range_of_caster(mut self, range: u8) -> Self {
        self.filters.in_range_of_caster = Some(range);
        self
    }

    pub fn build(self) -> TargetSelector {
        TargetSelector {
            selection: SelectionMethod::Manual(self.selector),
            validation: ValidationRules {
                creature: None,
                tile: Some(self.filters),
                player: None,
            },
        }
    }
}

// === Player Builder Methods ===
impl PlayerTargetBuilder {
    pub fn min_mana(mut self, mana: u32) -> Self {
        self.filters.min_mana = Some(mana);
        self
    }

    pub fn has_cards_in_hand(mut self, count: u8) -> Self {
        self.filters.has_cards_in_hand = Some(count);
        self
    }

    pub fn build(self) -> TargetSelector {
        TargetSelector {
            selection: SelectionMethod::Auto(self.selector),
            validation: ValidationRules {
                creature: None,
                tile: None,
                player: Some(self.filters),
            },
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
