use serde::{Deserialize, Serialize};

use crate::board::effect::EffectType;

use super::{condition::ConditionDef, selector::SelectorDef, value::ValueDef};

/// A single effect of an ability. Effects are executed in order by the
/// ability executor (`actions/execute.rs`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EffectDef {
    DealDamage {
        selector: SelectorDef,
        amount: ValueDef,
    },
    Heal {
        selector: SelectorDef,
        amount: ValueDef,
    },
    DrawCards {
        player: SelectorDef,
        amount: ValueDef,
    },
    AddGold {
        player: SelectorDef,
        amount: ValueDef,
    },
    ApplyEffect {
        selector: SelectorDef,
        effect: EffectType,
        duration: u16,
    },
    DestroyCreature {
        selector: SelectorDef,
    },
    ModifyStats {
        selector: SelectorDef,
        modifier: StatModifierDef,
    },
    /// Forced move of the creature(s) standing on the selected tile(s)
    /// by `direction` (or to an absolute board position when
    /// `absolute` is true).
    MoveCreature {
        selector: SelectorDef,
        direction: [i16; 2],
        #[serde(default)]
        absolute: bool,
    },
    DiscardCards {
        player: SelectorDef,
        amount: ValueDef,
    },
    Mill {
        player: SelectorDef,
        amount: ValueDef,
    },
    ReturnToHand {
        selector: SelectorDef,
    },

    /// Conditional branch: evaluates `condition` once when reached and
    /// splices the chosen branch into the ability's remaining effects.
    If {
        condition: ConditionDef,
        then: Vec<EffectDef>,
        #[serde(default)]
        otherwise: Vec<EffectDef>,
    },

    /// The active player picks one of the given options; the chosen
    /// option's effects are spliced into the ability's remaining effects.
    Choose { options: Vec<ChoiceOptionDef> },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChoiceOptionDef {
    pub label: String,
    #[serde(default)]
    pub effects: Vec<EffectDef>,
}

/// Data-level mirror of the runtime `StatModifier`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatModifierDef {
    Attack(i16),
    Health(i16),
    MaxHealth(i16),
    Speed(i16),
    Both { attack: i16, health: i16 },
}
