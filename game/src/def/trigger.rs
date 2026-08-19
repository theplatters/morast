use serde::{Deserialize, Serialize};

use crate::actions::{spell_speed::SpellSpeed, timing::ActionTiming};

use super::{condition::ConditionDef, effect::EffectDef};

/// When an ability fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TriggerDef {
    /// When the card is played / enters the board.
    OnPlay,
    /// At the end of the owner's turn.
    OnTurnEnd,
    /// When a trap is revealed.
    OnReveal,
}

/// A triggered ability of a card: trigger + condition + effects.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AbilityDef {
    pub trigger: TriggerDef,
    #[serde(default)]
    pub condition: ConditionDef,
    #[serde(default)]
    pub speed: SpellSpeed,
    #[serde(default)]
    pub timing: ActionTiming,
    #[serde(default)]
    pub effects: Vec<EffectDef>,
}
