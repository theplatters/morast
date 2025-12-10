use crate::engine::janet_handler::{api::*, controller::Environment};

use super::types::cfunction::JanetRawCFunction;

pub struct CoreFunction {
    pub name: &'static str,
    pub cfun: JanetRawCFunction,
    pub docs: &'static str,
}

pub const CORE_FUNCTIONS: &[CoreFunction] = &[
    CoreFunction {
        name: "draw",
        cfun: cfun_draw,
        docs: "Draws a card for the current player",
    },
    CoreFunction {
        name: "discard",
        cfun: cfun_discard,
        docs: "Discards a card from the hand",
    },
    CoreFunction {
        name: "get-gold",
        cfun: cfun_add_gold_to_player,
        docs: "Get's the amount of gold",
    },
    CoreFunction {
        name: "turn-player",
        cfun: cfun_turn_player,
        docs: "Get's the current player",
    },
    CoreFunction {
        name: "other-player",
        cfun: cfun_other_player,
        docs: "Get's the other player",
    },
    CoreFunction {
        name: "plus",
        cfun: cfun_plus,
        docs: "Generates a Plus of size n",
    },
    CoreFunction {
        name: "cross",
        cfun: cfun_cross,
        docs: "Generates a Cross of size n",
    },
    CoreFunction {
        name: "player-gold",
        cfun: cfun_gold_amount,
        docs: "Get's the amount of gold a player has",
    },
    CoreFunction {
        name: "shuffle",
        cfun: cfun_shuffle_deck,
        docs: "Shuffles the deck of the player",
    },
    CoreFunction {
        name: "owner",
        cfun: cfun_card_owner,
        docs: "Returns the owner of the card",
    },
    CoreFunction {
        name: "apply-effect",
        cfun: cfun_apply_effect,
        docs: "Applies an Effect to the given offset",
    },
    CoreFunction {
        name: "current-index",
        cfun: cfun_get_current_index,
        docs: "Get's the current index of the card",
    },
    CoreFunction {
        name: "from-current-position",
        cfun: cfun_from_current_position,
        docs: "Maps an array relative to a position",
    },
    CoreFunction {
        name: "is-owners-turn?",
        cfun: cfun_is_owners_turn,
        docs: "Returns true if the turn player is the owner of the card",
    },
];

impl Environment {
    pub fn register_core_functions(&self) {
        for func in CORE_FUNCTIONS {
            self.register(func.name, func.cfun, func.docs, Some("std"))
                .unwrap_or_else(|_| panic!("Could not register {} function", func.name));
        }
    }
}
