use crate::{
    engine::janet_handler::bindings::{janet_cfuns, JanetTable},
    game::context::Context,
};

use super::bindings::{janet_fixarity, janet_getnumber, janet_wrap_number, Janet, JanetReg};

pub unsafe extern "C" fn cfun_draw(argc: i32, argv: *mut Janet) -> Janet {
    unsafe {
        janet_fixarity(argc, 1);
        let num = janet_getnumber(argv, 0);
        janet_wrap_number(num * num)
    }
}

pub fn j_discard(context: &mut Context, amount: u32) {
    //TODO
}
