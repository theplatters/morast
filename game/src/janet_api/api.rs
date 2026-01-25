use bevy::math::I16Vec2;

use janet_bindings::{bindings::Janet, error::JanetError, janet_cfun, types::janetenum::JanetEnum};

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

pub unsafe extern "C" fn cfun_draw(_argc: i32, _argv: *mut Janet) -> Janet {
    todo!()
}

pub unsafe extern "C" fn cfun_discard(_argc: i32, _argv: *mut Janet) -> Janet {
    todo!()
}

pub unsafe extern "C" fn cfun_add_gold_to_player(_argc: i32, _argv: *mut Janet) -> Janet {
    todo!()
}

pub unsafe extern "C" fn cfun_turn_player(_argc: i32, _argv: *mut Janet) -> Janet {
    todo!()
}

pub unsafe extern "C" fn cfun_other_player(_argc: i32, _argv: *mut Janet) -> Janet {
    todo!()
}

fn plus(size: &[JanetEnum]) -> Result<JanetEnum, JanetError> {
    if size.len() != 1  {
        return Err(JanetError::OutOfBounds);
    }

    let size = size[0].as_int().ok_or(JanetError::Cast(format!(
        "could not cast {} to int",
        size[0]
    )))?;
    let plus: [[i32; 2]; 4] = [[-size, 0], [0, -size], [0, size], [size, 0]];

    let plus_enum = JanetEnum::Array(
        plus.into_iter()
            .map(Into::into) // each [i32; 2] -> JanetEnum
            .collect(),
    );
    Ok(plus_enum)
}

pub fn cross(argv: &[JanetEnum]) -> Result<JanetEnum, JanetError> {
    if argv.len() != 1  {
        return Err(JanetError::OutOfBounds);
    }

    let size = argv[0].as_int().ok_or(JanetError::Cast(format!(
        "could not cast {} to int",
        argv[0]
    )))?;

    let cross: [[i32; 2]; 4] = [[-size, -size], [size, -size], [-size, size], [size, size]];

    let cross_enum = JanetEnum::Array(
        cross
            .into_iter()
            .map(Into::into) // each [i32; 2] -> JanetEnum
            .collect(),
    );
    Ok(cross_enum)
}

janet_cfun!(cfun_cross, cross);
janet_cfun!(cfun_plus, plus);

pub unsafe extern "C" fn cfun_gold_amount(_argc: i32, _argv: *mut Janet) -> Janet {
    todo!()
}

pub unsafe extern "C" fn cfun_shuffle_deck(_argc: i32, _argv: *mut Janet) -> Janet {
    todo!()
}

pub unsafe extern "C" fn cfun_card_owner(_argc: i32, _argv: *mut Janet) -> Janet {
    todo!()
}

pub unsafe extern "C" fn cfun_get_current_index(_argc: i32, _argv: *mut Janet) -> Janet {
    todo!()
}

pub unsafe extern "C" fn cfun_apply_effect(_argc: i32, _argv: *mut Janet) -> Janet {
    todo!()
}

pub unsafe extern "C" fn cfun_from_current_position(_argc: i32, _argv: *mut Janet) -> Janet {
    todo!()
}

pub unsafe extern "C" fn cfun_is_owners_turn(_argc: i32, _argv: *mut Janet) -> Janet {
    todo!()
}
