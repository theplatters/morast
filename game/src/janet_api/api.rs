use bevy::math::I16Vec2;

use janet_bindings::{
    bindings::{
        Janet, JanetArray, janet_array, janet_array_push, janet_fixarity, janet_getarray,
        janet_getinteger64, janet_wrap_array, janet_wrap_integer,
    },
    types::janetenum::JanetEnum,
};

pub fn vec_to_janet_array(coords: &[I16Vec2]) -> *mut JanetArray {
    unsafe {
        let arr = janet_array(coords.len() as i32);
        for coord in coords {
            let sub = janet_array(2);
            janet_array_push(sub, janet_wrap_integer(coord.x as i32));
            janet_array_push(sub, janet_wrap_integer(coord.y as i32));
            janet_array_push(arr, janet_wrap_array(sub));
        }

        arr
    }
}

pub unsafe fn ptr_to_i16_vec(arr_ptr: *mut JanetArray) -> Option<Vec<I16Vec2>> {
    unsafe {
        if arr_ptr.is_null() {
            return None;
        }
        // Safety: rely on JanetArray layout from bindings; treat data as pointer to Janet elements.
        let count = (*arr_ptr).count as usize;
        let mut out = Vec::with_capacity(count);
        for i in 0..count {
            // pointer to the i-th Janet value in the outer array
            let elem_janet_ptr = (*arr_ptr).data.add(i);
            // obtain inner array pointer from that element (index 0 of a one-element argv)
            let sub_arr = janet_getarray(elem_janet_ptr, 0);
            if sub_arr.is_null() {
                return None;
            }
            // read integers from sub array's data (first and second element)
            let x = janet_getinteger64((*sub_arr).data, 0) as i16;
            let y = janet_getinteger64((*sub_arr).data, 1) as i16;
            out.push(I16Vec2::new(x, y));
        }
        Some(out)
    }
}
pub fn to_i16_vec(item: JanetEnum) -> Option<Vec<I16Vec2>> {
    let JanetEnum::Array(arr) = item else {
        return None;
    };

    let mut result = Vec::new();
    for item in arr {
        // Ensure the item is am `JanetEnum::_Array`
        if let JanetEnum::Array(inner_vec) = item {
            // Ensure the inner array has exactly two elements
            if inner_vec.len() != 2 {
                return None;
            }
            // Extract the two values
            let x = match inner_vec[..] {
                [JanetEnum::Int(value_x), JanetEnum::Int(value_y)] => {
                    [value_x as i16, value_y as i16]
                }
                _ => return None,
            };

            result.push(I16Vec2::from_array(x));
        } else {
            return None;
        }
    }
    Some(result)
}

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
