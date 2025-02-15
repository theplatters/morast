use crate::game::{player::PlayerID, Game};

use super::bindings::{
    janet_array, janet_array_push, janet_fixarity, janet_getinteger64, janet_getpointer,
    janet_getuinteger16, janet_wrap_array, janet_wrap_boolean, janet_wrap_integer, janet_wrap_nil,
    janet_wrap_u64, Janet,
};

pub unsafe extern "C" fn cfun_draw(argc: i32, argv: *mut Janet) -> Janet {
    janet_fixarity(argc, 3);
    let game = (janet_getpointer(argv, 0) as *mut Game)
        .as_mut()
        .expect("Couldn't cast reference");
    let num_cards = janet_getuinteger16(argv, 1);
    let player_id = janet_getuinteger16(argv, 2);
    game.scheduler.schedule_now(
        move |context| context.draw_cards(PlayerID::new(player_id), num_cards),
        1,
    );
    janet_wrap_nil()
}

pub unsafe extern "C" fn cfun_discard(argc: i32, argv: *mut Janet) -> Janet {
    janet_fixarity(argc, 3);
    let game = (janet_getpointer(argv, 0) as *mut Game)
        .as_mut()
        .expect("Couldn't cast reference");
    let num_cards = janet_getuinteger16(argv, 1);
    let player_id = janet_getuinteger16(argv, 2);

    game.scheduler.schedule_now(
        move |context| context.discard_cards(PlayerID::new(player_id), num_cards),
        1,
    );
    janet_wrap_nil()
}

pub unsafe extern "C" fn cfun_add_gold_to_player(argc: i32, argv: *mut Janet) -> Janet {
    janet_fixarity(argc, 3);
    let game = (janet_getpointer(argv, 0) as *mut Game)
        .as_mut()
        .expect("Couldn't cast reference");
    let amount = janet_getinteger64(argv, 1);
    let player_id = janet_getuinteger16(argv, 2);

    game.scheduler.schedule_now(
        move |context| context.set_gold(PlayerID::new(player_id), amount),
        1,
    );
    janet_wrap_nil()
}

pub unsafe extern "C" fn cfun_turn_player(argc: i32, argv: *mut Janet) -> Janet {
    janet_fixarity(argc, 1);
    (janet_getpointer(argv, 0) as *mut Game)
        .as_mut()
        .map_or(janet_wrap_nil(), |g| {
            janet_wrap_u64(g.turn_player_id().get() as u64)
        })
}

pub unsafe extern "C" fn cfun_other_player(argc: i32, argv: *mut Janet) -> Janet {
    janet_fixarity(argc, 1);
    (janet_getpointer(argv, 0) as *mut Game)
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
    let game = (janet_getpointer(argv, 0) as *mut Game)
        .as_mut()
        .expect("Couldn't cast reference");
    let player_id = PlayerID::new(janet_getuinteger16(argv, 0));
    game.get_player_gold(player_id)
        .map_or(janet_wrap_nil(), |r| janet_wrap_integer(r))
}

pub unsafe extern "C" fn cfun_turn_count(argc: i32, argv: *mut Janet) -> Janet {
    janet_fixarity(argc, 1);
    (janet_getpointer(argv, 0) as *mut Game)
        .as_mut()
        .map_or(janet_wrap_nil(), |game| {
            janet_wrap_u64(game.get_turn_count() as u64)
        })
}

pub unsafe extern "C" fn cfun_shuffle_deck(argc: i32, argv: *mut Janet) -> Janet {
    janet_fixarity(argc, 1);
    let player_id = PlayerID::new(janet_getuinteger16(argv, 1));
    (janet_getpointer(argv, 2) as *mut Game)
        .as_mut()
        .map_or(janet_wrap_nil(), |game| match game.shuffe_deck(player_id) {
            Some(_) => janet_wrap_boolean(1),
            None => janet_wrap_boolean(0),
        })
}
