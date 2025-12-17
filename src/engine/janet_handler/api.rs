use bevy::math::I16Vec2;

use super::{
    bindings::{janet_fixarity, janet_getinteger64, janet_wrap_array, Janet},
    types::janetenum::vec_to_janet_array,
};

pub unsafe extern "C" fn cfun_draw(argc: i32, argv: *mut Janet) -> Janet {
    todo!()
}

pub unsafe extern "C" fn cfun_discard(argc: i32, argv: *mut Janet) -> Janet {
    todo!()
}

pub unsafe extern "C" fn cfun_add_gold_to_player(argc: i32, argv: *mut Janet) -> Janet {
    todo!()
}

pub unsafe extern "C" fn cfun_turn_player(argc: i32, argv: *mut Janet) -> Janet {
    todo!()
}

pub unsafe extern "C" fn cfun_other_player(argc: i32, argv: *mut Janet) -> Janet {
    todo!()
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
    todo!()
}

pub unsafe extern "C" fn cfun_shuffle_deck(argc: i32, argv: *mut Janet) -> Janet {
    todo!()
}

pub unsafe extern "C" fn cfun_card_owner(argc: i32, argv: *mut Janet) -> Janet {
    todo!()
}

pub unsafe extern "C" fn cfun_get_current_index(argc: i32, argv: *mut Janet) -> Janet {
    todo!()
}

pub unsafe extern "C" fn cfun_apply_effect(argc: i32, argv: *mut Janet) -> Janet {
    todo!()
}

pub unsafe extern "C" fn cfun_from_current_position(argc: i32, argv: *mut Janet) -> Janet {
    todo!()
}

pub unsafe extern "C" fn cfun_is_owners_turn(argc: i32, argv: *mut Janet) -> Janet {
    todo!()
}
