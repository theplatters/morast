use serde::{Deserialize, Serialize};

use crate::actions::value_source::StatType;

use super::selector::SelectorDef;

/// Data-level mirror of the runtime `ValueSource`, plus an `Expr` sugar that
/// is parsed into the structured variants by [`super::value_expr`] at
/// load/convert time.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ValueDef {
    /// Static constant value.
    Constant(u16),

    /// Count of entities matching a selector.
    Count(Box<SelectorDef>),

    /// Random value in the inclusive range [min, max].
    Random {
        min: Box<ValueDef>,
        max: Box<ValueDef>,
    },

    /// Value read from a creature's stats.
    CreatureStat {
        selector: Box<SelectorDef>,
        stat: StatType,
    },

    /// Mathematical operations (evaluated with saturating/checked semantics;
    /// division by zero evaluates to 0).
    Add(Box<ValueDef>, Box<ValueDef>),
    Sub(Box<ValueDef>, Box<ValueDef>),
    Multiply(Box<ValueDef>, Box<ValueDef>),
    Divide(Box<ValueDef>, Box<ValueDef>),
    Min(Box<ValueDef>, Box<ValueDef>),
    Max(Box<ValueDef>, Box<ValueDef>),

    /// Sugar: arithmetic expression such as `attack(caster) * 2 + 1`.
    /// Parsed into the structured variants above by
    /// [`super::value_expr::parse_value_expr`].
    Expr(String),
}
