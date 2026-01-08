use bevy::prelude::*;

use crate::{
    actions::action_systems::ActionPlugin,
    board::BoardPlugin,
    card::{
        add_cards,
        card_registry::{CardRegistry, init_card_registry},
    },
    events::GameMessagesPlugin,
    player::{add_player, draw_starting_cards},
    turn_controller::TurnControllerPlugin,
};

pub unsafe fn vec_to_janet_array(coords: &[I16Vec2]) -> *mut JanetArray {
    let arr = janet_array(coords.len() as i32);
    for coord in coords {
        let sub = janet_array(2);
        janet_array_push(sub, janet_wrap_integer(coord.x as i32));
        janet_array_push(sub, janet_wrap_integer(coord.y as i32));
        janet_array_push(arr, janet_wrap_array(sub));
    }
    arr
}

pub unsafe fn ptr_to_i16_vec(arr_ptr: *mut JanetArray) -> Option<Vec<I16Vec2>> {
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

impl TryFrom<JanetEnum> for I16Vec2 {
    type Error = JanetError;

    fn try_from(value: JanetEnum) -> Result<Self, Self::Error> {
        match value {
            JanetEnum::Array(a) if a.len() == 2 => {
                let x = a.get(0).expect("Fail");
                let y = a.get(1).expect("Fail");

                Ok(I16Vec2::new(x.try_into()?, y.try_into()?))
            }
            JanetEnum::Tuple(t) => {
                let x = t.get(0).expect("Fail");
                let y = t.get(1).expect("Fail");

                Ok(I16Vec2::new(x.try_into()?, y.try_into()?))
            }
            _ => Err(JanetError::Cast(format!(
                "Janet type not supported expected array or tuple, got {}",
                value
            ))),
        }
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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(CardRegistry::new())
        .insert_non_send_resource(Environment::new("scripts/loader.janet"))
        .add_systems(
            Startup,
            (
                init_card_registry,
                add_player,
                add_cards,
                draw_starting_cards,
            )
                .chain(),
        )
        .add_plugins((
            GameMessagesPlugin,
            BoardPlugin,
            TurnControllerPlugin,
            RendererPlugin,
            ActionPlugin,
        ))
        .run();
}
