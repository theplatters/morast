use std::{ffi::CStr, str::FromStr};

use log::debug;

use crate::{
    engine::janet_handler::bindings::janet_getsymbol,
    game::{
        board::effect::{Effect, EffectType},
        error::Error,
        events::event_scheduler::GameScheduler,
        game_context::GameContext,
        player::PlayerID,
        Game,
    },
};

use super::{
    bindings::{
        janet_array, janet_array_push, janet_fixarity, janet_getarray, janet_getinteger64,
        janet_getpointer, janet_getuinteger16, janet_getuinteger64, janet_wrap_array,
        janet_wrap_boolean, janet_wrap_integer, janet_wrap_nil, janet_wrap_u64, Janet,
    },
    types::janetenum::{to_u16_vec, JanetEnum},
};

pub unsafe extern "C" fn cfun_draw(argc: i32, argv: *mut Janet) -> Janet {
    janet_fixarity(argc, 4);
    let context = (janet_getpointer(argv, 0) as *mut GameContext)
        .as_mut()
        .expect("Couldn't cast reference");

    let num_cards = janet_getuinteger16(argv, 1);
    let player_id = janet_getuinteger16(argv, 2);
    context.draw_cards(PlayerID::new(player_id), num_cards);
    janet_wrap_nil()
}

pub unsafe extern "C" fn cfun_discard(argc: i32, argv: *mut Janet) -> Janet {
    janet_fixarity(argc, 3);
    let context = (janet_getpointer(argv, 0) as *mut GameContext)
        .as_mut()
        .expect("Couldn't cast reference");

    let num_cards = janet_getuinteger16(argv, 1);
    let player_id = janet_getuinteger16(argv, 2);

    context.discard_cards(PlayerID::new(player_id), num_cards);
    janet_wrap_nil()
}

pub unsafe extern "C" fn cfun_add_gold_to_player(argc: i32, argv: *mut Janet) -> Janet {
    debug!("Called into cfun_add_gold_to_player");
    janet_fixarity(argc, 3);

    let context = (janet_getpointer(argv, 0) as *mut GameContext)
        .as_mut()
        .expect("Couldn't cast reference");
    let amount = janet_getinteger64(argv, 1);
    let player_id = janet_getinteger64(argv, 2) as u16;

    context.add_gold(PlayerID::new(player_id), amount);

    janet_wrap_nil()
}

pub unsafe extern "C" fn cfun_turn_player(argc: i32, argv: *mut Janet) -> Janet {
    janet_fixarity(argc, 1);
    (janet_getpointer(argv, 0) as *mut GameContext)
        .as_mut()
        .map_or(janet_wrap_nil(), |g| {
            janet_wrap_u64(g.turn_player_id().get() as u64)
        })
}

pub unsafe extern "C" fn cfun_other_player(argc: i32, argv: *mut Janet) -> Janet {
    janet_fixarity(argc, 1);
    (janet_getpointer(argv, 0) as *mut GameContext)
        .as_mut()
        .map_or(janet_wrap_nil(), |g| {
            janet_wrap_u64(g.other_player_id().get() as u64)
        })
}

pub unsafe extern "C" fn cfun_plus(argc: i32, argv: *mut Janet) -> Janet {
    janet_fixarity(argc, 1);
    let size = janet_getinteger64(argv, 0) as i32;
    let plus: [[i32; 2]; 4] = [[-size, 0], [0, -size], [0, size], [size, 0]];
    let arr = janet_array(4);
    plus.iter().for_each(|el| {
        let r = janet_array(2);
        janet_array_push(r, janet_wrap_integer(el[0]));
        janet_array_push(r, janet_wrap_integer(el[1]));
        janet_array_push(arr, janet_wrap_array(r));
    });
    janet_wrap_array(arr)
}

pub unsafe extern "C" fn cfun_cross(argc: i32, argv: *mut Janet) -> Janet {
    janet_fixarity(argc, 1);
    let size = janet_getinteger64(argv, 0) as i32;
    let cross: [[i32; 2]; 4] = [[-size, -size], [size, -size], [-size, size], [size, size]];
    let arr = janet_array(4);
    cross.iter().for_each(|el| {
        let r = janet_array(2);
        janet_array_push(r, janet_wrap_integer(el[0]));
        janet_array_push(r, janet_wrap_integer(el[1]));
        janet_array_push(arr, janet_wrap_array(r));
    });
    janet_wrap_array(arr)
}

pub unsafe extern "C" fn cfun_gold_amout(argc: i32, argv: *mut Janet) -> Janet {
    janet_fixarity(argc, 2);
    let game = (janet_getpointer(argv, 0) as *mut GameContext)
        .as_mut()
        .expect("Couldn't cast reference");

    let player_id = PlayerID::new(janet_getinteger64(argv, 1) as u16);
    game.get_player_gold(player_id)
        .map_or(janet_wrap_nil(), |r| janet_wrap_integer(r as i32))
}

pub unsafe extern "C" fn cfun_turn_count(argc: i32, argv: *mut Janet) -> Janet {
    janet_fixarity(argc, 1);
    (janet_getpointer(argv, 0) as *mut GameScheduler)
        .as_mut()
        .map_or(janet_wrap_nil(), |scheduler| {
            janet_wrap_u64(scheduler.get_turn_count() as u64)
        })
}

pub unsafe extern "C" fn cfun_shuffle_deck(argc: i32, argv: *mut Janet) -> Janet {
    janet_fixarity(argc, 1);
    let player_id = PlayerID::new(janet_getuinteger16(argv, 0));
    (janet_getpointer(argv, 2) as *mut GameContext)
        .as_mut()
        .map_or(janet_wrap_nil(), |game| match game.shuffe_deck(player_id) {
            Some(_) => janet_wrap_boolean(1),
            None => janet_wrap_boolean(0),
        })
}

pub unsafe extern "C" fn cfun_card_owner(argc: i32, argv: *mut Janet) -> Janet {
    janet_fixarity(argc, 1);

    (janet_getpointer(argv, 0) as *mut GameContext)
        .as_mut()
        .map_or(janet_wrap_nil(), |game| match game.current_selected_card {
            Some(card_id) => {
                println!("{:?}", card_id);
                janet_wrap_u64(card_id.player_id.get() as u64)
            }
            None => panic!("Selected card not found"),
        })
}

pub unsafe extern "C" fn cfun_get_current_index(argc: i32, argv: *mut Janet) -> Janet {
    janet_fixarity(argc, 1);
    (janet_getpointer(argv, 0) as *mut GameContext)
        .as_mut()
        .map_or(janet_wrap_nil(), |game| match game.current_selected_index {
            Some(index) => {
                let arr = janet_array(2);
                janet_array_push(arr, janet_wrap_integer(index.x as i32));
                janet_array_push(arr, janet_wrap_integer(index.y as i32));
                janet_wrap_array(arr)
            }
            None => panic!("Selected card not found"),
        })
}

pub unsafe extern "C" fn cfun_apply_effect(argc: i32, argv: *mut Janet) -> Janet {
    janet_fixarity(argc, 3);

    let context = (janet_getpointer(argv, 0) as *mut GameContext)
        .as_mut()
        .expect("Couldn't cast reference");

    let effect_type = EffectType::from_str(
        CStr::from_ptr(janet_getsymbol(argv, 1) as *const i8)
            .to_str()
            .expect("Couldn't read effect as string"),
    )
    .expect("Effect not found");
    let duration = janet_getuinteger16(argv, 2);
    let effect = Effect::new(effect_type, duration);

    //TODO: Rewrite this, this is horrible and a desaster waiting to happen
    let tiles = to_u16_vec(JanetEnum::_Array(
        JanetEnum::unwrap_array(*janet_getarray(argv, 4)).expect("Could not cast array"),
    ))
    .expect("Could not cast array");

    context.add_effects(effect, &tiles);
    // Iterate over elements
    janet_wrap_nil()
}
