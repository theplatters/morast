use crate::game::{player::PlayerID, Game};

use super::bindings::{
    janet_array, janet_array_push, janet_fixarity, janet_getinteger64, janet_getpointer,
    janet_getuinteger16, janet_wrap_array, janet_wrap_integer, janet_wrap_nil, janet_wrap_u64,
    Janet,
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

pub unsafe extern "C" fn cfun_getgold(argc: i32, argv: *mut Janet) -> Janet {
    janet_fixarity(argc, 3);
    let game = (janet_getpointer(argv, 0) as *mut Game)
        .as_mut()
        .expect("Couldn't cast reference");
    let amount = janet_getinteger64(argv, 1);
    let player_id = janet_getuinteger16(argv, 2);

    game.scheduler.schedule_now(
        move |context| context.get_gold(PlayerID::new(player_id), amount),
        1,
    );
    janet_wrap_nil()
}

pub unsafe extern "C" fn cfun_turn_player(argc: i32, argv: *mut Janet) -> Janet {
    janet_fixarity(argc, 1);
    let game = (janet_getpointer(argv, 0) as *mut Game)
        .as_mut()
        .expect("Couldn't cast reference");
    janet_wrap_u64(game.turn_player_id().get() as u64)
}

pub unsafe extern "C" fn cfun_other_player(argc: i32, argv: *mut Janet) -> Janet {
    janet_fixarity(argc, 1);
    let game = (janet_getpointer(argv, 0) as *mut Game)
        .as_mut()
        .expect("Couldn't cast reference");
    janet_wrap_u64(game.other_player_id().get() as u64)
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
