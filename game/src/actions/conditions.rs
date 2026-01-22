use super::{
    targeting::{SingleTarget, TargetSelector, TileTarget},
    value_source::ValueSource,
};

use crate::{
    actions::targeting::{CreatureTarget, MultiTarget, Or, PlayerTarget},
    board::effect::EffectType,
};
use janet_bindings::{bindings::JanetAbstractType, types::janetabstract::IsAbstract};

#[derive(Debug, Clone)]
pub enum Condition {
    /// Always true
    Always,

    /// Always false
    Never,

    /// Check if a value comparison is true
    Compare {
        left: ValueSource,
        op: CompareOp,
        right: ValueSource,
    },

    /// Check if a tile has an effect
    HasEffect {
        selector: TargetSelector<TileTarget, SingleTarget>,
        effect: EffectType,
    },

    PlayerCondition(PlayerCondition),
    CreatureCondition(CreatureCondition),

    /// Logical operations
    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
    Not(Box<Condition>),
}

impl IsAbstract for Condition {
    fn type_info() -> &'static janet_bindings::bindings::JanetAbstractType {
        const CONDITION_ATYPE: JanetAbstractType =
            JanetAbstractType::new(c"taget/condition", Condition::gc);
        &CONDITION_ATYPE
    }
}

#[derive(Debug, Clone)]
pub enum PlayerCondition {
    /// Check player resources
    HasMinGold {
        player: TargetSelector<PlayerTarget, SingleTarget>,
        amount: u16,
    },
    HasMaxGold {
        player: TargetSelector<PlayerTarget, SingleTarget>,
        amount: u16,
    },

    HasMinHealt {
        player: TargetSelector<PlayerTarget, SingleTarget>,
        amount: u16,
    },
    HasMaxHealth {
        player: TargetSelector<PlayerTarget, SingleTarget>,
        amount: u16,
    },

    /// Check deck/hand state
    DeckHasCards {
        player: TargetSelector<PlayerTarget, SingleTarget>,
        count: u16,
    },
    SelectorHasCount {
        selector: TargetSelector<PlayerTarget, Or<SingleTarget, MultiTarget>>,
        count: u16,
    },
}

#[derive(Debug, Clone)]
pub enum CreatureCondition {
    NotMoved {
        creature: TargetSelector<PlayerTarget, Or<SingleTarget, MultiTarget>>,
    },
    FullHealth {
        creature: TargetSelector<PlayerTarget, Or<SingleTarget, MultiTarget>>,
    },
    SelectorHasCount {
        selector: TargetSelector<CreatureTarget, Or<SingleTarget, MultiTarget>>,
        count: u16,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompareOp {
    Equal,
    NotEqual,
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
}
