use std::{ffi::CStr, str::FromStr};

use log::debug;
use macroquad::math::I16Vec2;

use crate::{
    engine::janet_handler::{bindings::janet_getsymbol, types::janetenum::ptr_to_i16_vec},
    game::{
        board::effect::{Effect, EffectType},
        events::event_scheduler::GameScheduler,
        game_context::GameContext,
        player::PlayerID,
    },
};

use super::{
    bindings::{
        janet_array, janet_array_push, janet_fixarity, janet_getarray, janet_getinteger64,
        janet_getpointer, janet_getuinteger16, janet_wrap_array, janet_wrap_boolean,
        janet_wrap_integer, janet_wrap_nil, janet_wrap_u64, Janet,
    },
    types::janetenum::vec_to_janet_array,
};

pub unsafe extern "C" fn cfun_draw(argc: i32, argv: *mut Janet) -> Janet {
    janet_fixarity(argc, 3);
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
    let size = janet_getinteger64(argv, 0) as i16;
    let plus: [I16Vec2; 4] = [
        I16Vec2::new(-size, 0),
        I16Vec2::new(0, -size),
        I16Vec2::new(0, size),
        I16Vec2::new(size, 0),
    ];
    janet_wrap_array(vec_to_janet_array(&plus))
}

pub unsafe extern "C" fn cfun_cross(argc: i32, argv: *mut Janet) -> Janet {
    janet_fixarity(argc, 1);
    let size = janet_getinteger64(argv, 0) as i16;

    let cross: [I16Vec2; 4] = [
        I16Vec2::new(-size, -size),
        I16Vec2::new(size, -size),
        I16Vec2::new(-size, size),
        I16Vec2::new(size, size),
    ];
    janet_wrap_array(vec_to_janet_array(&cross))
}

pub unsafe extern "C" fn cfun_gold_amount(argc: i32, argv: *mut Janet) -> Janet {
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
    (janet_getpointer(argv, 1) as *mut GameContext)
        .as_mut()
        .map_or(janet_wrap_nil(), |game| {
            match game.shuffle_deck(player_id) {
                Some(_) => janet_wrap_boolean(1),
                None => janet_wrap_boolean(0),
            }
        })
}

pub unsafe extern "C" fn cfun_card_owner(argc: i32, argv: *mut Janet) -> Janet {
    janet_fixarity(argc, 2);

    let id = janet_getinteger64(argv, 1);
    (janet_getpointer(argv, 0) as *mut GameContext)
        .as_mut()
        .map_or(janet_wrap_nil(), |game| {
            match game.get_card_owner(id.into()) {
                Some(owner) => janet_wrap_u64(owner.get() as u64),
                None => janet_wrap_nil(),
            }
        })
}

pub unsafe extern "C" fn cfun_get_current_index(argc: i32, argv: *mut Janet) -> Janet {
    print!("getting current index");
    janet_fixarity(argc, 2);
    let id = janet_getinteger64(argv, 1);
    (janet_getpointer(argv, 0) as *mut GameContext)
        .as_mut()
        .map_or(janet_wrap_nil(), |game| {
            match game.get_board().get_card_index(id.into()) {
                Some(index) => {
                    let arr = janet_array(2);
                    janet_array_push(arr, janet_wrap_integer(index.x as i32));
                    janet_array_push(arr, janet_wrap_integer(index.y as i32));
                    janet_wrap_array(arr)
                }
                None => janet_wrap_nil(),
            }
        })
}

pub unsafe extern "C" fn cfun_apply_effect(argc: i32, argv: *mut Janet) -> Janet {
    janet_fixarity(argc, 5);

    println!("Applying effect");
    let Some(context) = (janet_getpointer(argv, 0) as *mut GameContext).as_mut() else {
        print!("Could not get game context");
        return janet_wrap_nil();
    };

    let card_id = janet_getuinteger16(argv, 1).into();
    let effect_cstr = CStr::from_ptr(janet_getsymbol(argv, 2) as *const i8);
    let effect_str = match effect_cstr.to_str() {
        Ok(s) => s,
        Err(_) => return janet_wrap_nil(),
    };

    let effect_type = match EffectType::from_str(effect_str) {
        Ok(t) => t,
        Err(_) => return janet_wrap_nil(),
    };

    let duration = janet_getuinteger16(argv, 2);

    let owner = context
        .get_board()
        .get_card_owner(&card_id)
        .expect("Could not find card owner");

    let effect = Effect::new(effect_type, duration, owner);

    // Use centralized helper to convert nested JanetArray -> Vec<I16Vec2>
    let tiles = match ptr_to_i16_vec(janet_getarray(argv, 3)) {
        Some(v) => v,
        None => return janet_wrap_nil(),
    };

    context.add_effects(effect, &tiles);
    // Iterate over elements
    janet_wrap_nil()
}

pub unsafe extern "C" fn cfun_from_current_position(argc: i32, argv: *mut Janet) -> Janet {
    janet_fixarity(argc, 3);

    let context = (janet_getpointer(argv, 0) as *mut GameContext)
        .as_mut()
        .expect("Failed to cast reference to GameContext");

    let card_id = janet_getinteger64(argv, 1);
    let board = context.get_board();
    let Some(card_index) = board.get_card_index(card_id.into()) else {
        return janet_wrap_nil();
    };

    let tiles = match ptr_to_i16_vec(janet_getarray(argv, 2)) {
        Some(v) => v,
        None => return janet_wrap_nil(),
    };

    let remaped_tiles: Vec<I16Vec2> = tiles.iter().map(|tile| *tile + card_index).collect();
    janet_wrap_array(vec_to_janet_array(&remaped_tiles))
}

pub unsafe extern "C" fn cfun_is_owners_turn(argc: i32, argv: *mut Janet) -> Janet {
    janet_fixarity(argc, 2);

    let context = (janet_getpointer(argv, 0) as *mut GameContext)
        .as_mut()
        .expect("Failed to cast reference to GameContext");

    let card_id = janet_getinteger64(argv, 1);
    let board = context.get_board();
    let Some(card_owner) = board.get_card_owner(&card_id.into()) else {
        return janet_wrap_nil();
    };
    let is_turn = card_owner == context.turn_player_id();
    janet_wrap_boolean(is_turn.into())
}
