use serde::{Deserialize, Serialize};

use super::value::ValueDef;

/// Which kind of entity a selector picks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelectorKindDef {
    Creature,
    Tile,
    Player,
    Hand,
}

/// How many entities a selector picks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardinalityDef {
    Single,
    Multi,
    /// Selector type accepts either single- or multi-cardinality modes.
    Any,
}

/// Data-level mirror of a `TargetSelector`: kind + cardinality + selection
/// mode + filters. Converted into an `AnyTargetSelector` in
/// [`crate::def::convert`], which rejects invalid kind/selection/filter
/// combinations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectorDef {
    pub kind: SelectorKindDef,
    pub cardinality: CardinalityDef,
    pub selection: SelectionDef,
    #[serde(default)]
    pub filters: Vec<FilterDef>,
}

/// Flat selection-mode space. Not every variant is valid for every
/// [`SelectorKindDef`]; invalid combinations are rejected at convert time.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SelectionDef {
    // --- automatic creature modes ---
    Strongest,
    Caster,
    /// The entity currently targeted by the running ability.
    CurrentTarget,
    AllEnemy,
    AllFriendly,
    RandomCreatures {
        count: ValueDef,
    },

    // --- automatic tile modes ---
    AllTiles,
    RadiusAroundCaster {
        radius: u8,
    },

    // --- automatic player modes ---
    TurnPlayer,
    NonTurnPlayer,
    AllPlayers,
    /// The player owning the ability's caster.
    Owner,

    // --- automatic hand modes ---
    AllCards,
    AllCreatures,
    AllSpells,
    AllTraps,

    // --- manual creature modes ---
    ChooseCreatures {
        min: ValueDef,
        max: ValueDef,
    },
    MaxNFriendly {
        count: ValueDef,
    },
    ExactlyNFriendly {
        count: ValueDef,
    },

    // --- manual tile modes ---
    ChooseTiles {
        amount: ValueDef,
    },
    ChooseArea {
        radius: ValueDef,
    },

    // --- manual player mode ---
    ChoosePlayer,

    // --- manual hand mode ---
    ChooseCards {
        count: ValueDef,
    },
}

/// Flat filter space. Not every variant is valid for every
/// [`SelectorKindDef`]; invalid combinations are rejected at convert time.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FilterDef {
    // --- creature base filters ---
    MinHealth(ValueDef),
    MaxHealth(ValueDef),
    HealthPercent { min: ValueDef, max: ValueDef },
    DamagedOnly,
    MinAttack(ValueDef),
    CanAttack(bool),

    // --- tile base filters ---
    EmptyOnly,
    OccupiedOnly,
    InRangeOfCaster(ValueDef),

    // --- player base filters ---
    MinGold(ValueDef),
    MaxGold(ValueDef),
    HasCardsInHand(ValueDef),
    MinPlayerHealth(ValueDef),
    MaxPlayerHealth(ValueDef),

    // --- player extra rules (no backing state yet: always reject) ---
    TookDamageLastRound,
    PlayedCardThisTurn,

    // --- hand base filters ---
    MinCost(ValueDef),
    MaxCost(ValueDef),

    // --- hand extra rules ---
    ExcludeCreatures,
    ExcludeSpells,
    ExcludeTraps,
}
