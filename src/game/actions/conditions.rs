use crate::game::{
    actions::{
        action_prototype::ValueSource,
        targeting::{SingleTarget, TargetSelector, TileFilters, TileTarget},
    },
    board::effect::EffectType,
    janet_action::JanetAction,
};

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
        selector: TargetSelector<TileTarget, SingleTarget, TileFilters>,
        effect: EffectType,
    },

    /// Logical operations
    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
    Not(Box<Condition>),

    /// Custom Janet condition
    Custom(Box<JanetAction>),
}

pub enum PlayerCondition {
    /// Check player resources
    HasGold {
        amount: u16,
    },

    /// Check deck/hand state
    DeckHasCards {
        count: u16,
    },
    HandHasCards {
        count: u16,
    },
}

pub enum CreatureCondition {
    NotMoved,
    FullHealth,
    //...
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
