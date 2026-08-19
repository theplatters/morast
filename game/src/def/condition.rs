use serde::{Deserialize, Serialize};

use crate::{actions::conditions::CompareOp, board::effect::EffectType};

use super::{selector::SelectorDef, value::ValueDef};

/// Data-level mirror of the runtime `Condition`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConditionDef {
    /// Always true.
    Always,

    /// Always false.
    Never,

    /// Compare two dynamically evaluated values.
    Compare {
        left: ValueDef,
        op: CompareOp,
        right: ValueDef,
    },

    /// Check if a tile has an effect.
    HasEffect {
        selector: SelectorDef,
        effect: EffectType,
    },

    Player(PlayerConditionDef),
    Creature(CreatureConditionDef),

    /// Logical operations.
    And(Box<ConditionDef>, Box<ConditionDef>),
    Or(Box<ConditionDef>, Box<ConditionDef>),
    Not(Box<ConditionDef>),
}

impl Default for ConditionDef {
    fn default() -> Self {
        Self::Always
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PlayerConditionDef {
    HasMinGold { player: SelectorDef, amount: u16 },
    HasMaxGold { player: SelectorDef, amount: u16 },

    HasMinHealth { player: SelectorDef, amount: u16 },
    HasMaxHealth { player: SelectorDef, amount: u16 },

    DeckHasCards { player: SelectorDef, count: u16 },
    SelectorHasCount { selector: SelectorDef, count: u16 },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CreatureConditionDef {
    NotMoved { creature: SelectorDef },
    FullHealth { creature: SelectorDef },
    SelectorHasCount { selector: SelectorDef, count: u16 },
}
