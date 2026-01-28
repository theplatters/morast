use bevy::{
    ecs::{entity::Entity, world::World},
    math::I16Vec2,
};

use janet_bindings::{
    bindings::{Janet, JanetAbstract, janet_checkabstract, janet_wrap_nil},
    error::JanetError,
    janet_cfun,
    types::{janetabstract::IsAbstract, janetenum::JanetEnum},
};

use crate::{
    actions::{DealDamage, HealCreature},
    janet_api::world_context::ScriptCtx,
};

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

fn unwrap_entity(item: &JanetEnum) -> Result<Entity, JanetError> {
    let Some(entity) = Entity::from_raw_u32(item.as_uint().ok_or(JanetError::Type(
        "Second argument is not a uint which s not supported".into(),
    ))? as u32) else {
        return Err(JanetError::Type(
            "Second argument is not a uint which s not supported".into(),
        ));
    };
    Ok(entity)
}

fn plus(size: &[JanetEnum]) -> Result<JanetEnum, JanetError> {
    if size.len() != 1 {
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

janet_cfun!(cfun_plus, plus);

pub fn cross(argv: &[JanetEnum]) -> Result<JanetEnum, JanetError> {
    if argv.len() != 1 {
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

pub fn damage(argv: &[JanetEnum]) -> Result<JanetEnum, JanetError> {
    if argv.len() != 3 {
        return Err(JanetError::OutOfBounds);
    }

    let Some(mut world) = argv[0].clone().into_abstract() else {
        return Err(JanetError::Type("First argument is not a world".into()));
    };
    let Some(script_ctx) = world.as_mut::<ScriptCtx>() else {
        return Err(JanetError::Type("First argument is not a world".into()));
    };
    let entity = unwrap_entity(&argv[1])?;

    let amount = argv[2].as_uint().ok_or(JanetError::Type(
        "Second argument is not a uint which s not supported".into(),
    ))? as u16;

    script_ctx.trigger(DealDamage { amount, entity });

    Ok(JanetEnum::Null)
}

janet_cfun!(cfun_damage, damage);

pub fn heal(argv: &[JanetEnum]) -> Result<JanetEnum, JanetError> {
    if argv.len() != 3 {
        return Err(JanetError::OutOfBounds);
    }

    let Some(mut world) = argv[0].clone().into_abstract() else {
        return Err(JanetError::Type("First argument is not a world".into()));
    };
    let Some(script_ctx) = world.as_mut::<ScriptCtx>() else {
        return Err(JanetError::Type("First argument is not a world".into()));
    };

    let entity = unwrap_entity(&argv[1])?;

    let amount = argv[2].as_uint().ok_or(JanetError::Type(
        "Second argument is not a uint which s not supported".into(),
    ))? as u16;

    script_ctx.trigger(HealCreature { amount, entity });

    Ok(JanetEnum::Null)
}

janet_cfun!(cfun_heal, heal);
