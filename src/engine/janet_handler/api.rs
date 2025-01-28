use crate::game::{context::GameContext, events::event::Event};

use super::bindings::{
    janet_fixarity, janet_getinteger64, janet_getpointer, janet_wrap_nil, Janet,
};

pub unsafe extern "C" fn cfun_draw(argc: i32, argv: *mut Janet) -> Janet {
    janet_fixarity(argc, 2);
    let context = (janet_getpointer(argv, 0) as *mut GameContext)
        .as_mut()
        .expect("Couldn't cast reference");
    let num_cards = janet_getinteger64(argv, 1);
    (0..num_cards).for_each(|_| {
        context
            .event_manager
            .publish(Event::DrawCard(context.turn_player))
    });
    janet_wrap_nil()
}

pub unsafe extern "C" fn cfun_discard(argc: i32, argv: *mut Janet) -> Janet {
    janet_fixarity(argc, 2);
    let context = (janet_getpointer(argv, 0) as *mut GameContext)
        .as_mut()
        .expect("Couldn't cast reference");
    let num_cards = janet_getinteger64(argv, 1);
    (0..num_cards).for_each(|_| {
        context
            .event_manager
            .publish(Event::DiscardCard(context.turn_player))
    });
    janet_wrap_nil()
}
