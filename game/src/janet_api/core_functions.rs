macro_rules! core_fns {
    ($( $name:literal => $cfun:path ; $docs:literal ),* $(,)?) => {
        &[
            $(
                CoreFunction { name: $name, cfun: $cfun, docs: $docs },
            )*
        ] as &[CoreFunction]
    };
}
// 2) Your existing list can become:
pub const CORE_FUNCTIONS: &[CoreFunction] = core_fns![
    "draw" => cfun_draw; "Draws a card for the current player",
    "discard" => cfun_discard; "Discards a card from the hand",
    "get-gold" => cfun_add_gold_to_player; "Get's the amount of gold",
    "plus" => cfun_plus; "Generates a Plus of size n",
    "cross" => cfun_cross; "Generates a Cross of size n",
    "player-gold" => cfun_gold_amount; "Get's the amount of gold a player has",
    "shuffle" => cfun_shuffle_deck; "Shuffles the deck of the player",
    "owner" => cfun_card_owner; "Returns the owner of the card",
    "apply-effect" => cfun_apply_effect; "Applies an Effect to the given offset",
    "current-index" => cfun_get_current_index; "Get's the current index of the card",
    "from-current-position" => cfun_from_current_position; "Maps an array relative to a position",
    "is-owners-turn?" => cfun_is_owners_turn; "Returns true if the turn player is the owner of the card",
];

impl Environment {
    pub fn register_core_functions(&self) {
        for func in CORE_FUNCTIONS {
            self.register(func.name, func.cfun, func.docs, Some("std"))
                .unwrap_or_else(|_| panic!("Could not register {} function", func.name));
        }
    }
}
