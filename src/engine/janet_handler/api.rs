use crate::{
    engine::janet_handler::bindings::{janet_cfuns, JanetTable},
    game::context::Context,
};

use super::bindings::{
    janet_fixarity, janet_getinteger64, janet_getnumber, janet_getpointer, janet_wrap_nil,
    janet_wrap_number, Janet, JanetReg,
};

pub unsafe extern "C" fn cfun_draw(argc: i32, argv: *mut Janet) -> Janet {
    unsafe {
        janet_fixarity(argc, 2);
        let mut context = (janet_getpointer(argv, 0) as *mut Context)
            .as_mut()
            .expect("Couldn't cast reference");
        let num_cards = janet_getinteger64(argv, 1);
    }
    janet_wrap_nil()
}

pub fn j_discard(context: &mut Context, amount: u32) {
    //TODO
}
