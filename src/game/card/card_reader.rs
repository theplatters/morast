use macroquad::math::I16Vec2;

use crate::{
    engine::{
        asset_loader::AssetLoader,
        janet_handler::{
            bindings::Janet,
            controller::Environment,
            types::{function::Function, janetenum::JanetEnum},
        },
    },
    game::game_context::GameContext,
};

use super::Card;
fn convert_to_u16_vec(env: &Environment, attribute: &str, name: &str) -> Option<Vec<I16Vec2>> {
    let mut result = Vec::new();
    let Some(JanetEnum::_Array(vec)) = JanetEnum::get::<GameContext>(env, attribute, Some(name))
    else {
        return None;
    };

    for item in vec {
        // Ensure the item is am `JanetEnum::_Array`
        if let JanetEnum::_Array(inner_vec) = item {
            // Ensure the inner array has exactly two elements
            if inner_vec.len() != 2 {
                return None;
            }
            // Extract the two values
            let x = match inner_vec[..] {
                [JanetEnum::_Int(value_x), JanetEnum::_Int(value_y)] => {
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

pub async fn read_card(
    env: &Environment,
    name: &str,
    asset_loader: &mut AssetLoader,
) -> Result<Card, &'static str> {
    let draw_action =
        Function::get_method(env, "on-draw", Some(name)).ok_or("on-draw not found")?;
    let play_action =
        Function::get_method(env, "on-play", Some(name)).ok_or("on-play not found")?;
    let discard_action =
        Function::get_method(env, "on-discard", Some(name)).ok_or("on-discard not found")?;

    let attack = convert_to_u16_vec(env, "attack", name).ok_or("attack not found")?;
    let movement = convert_to_u16_vec(env, "movement", name).ok_or("movement not found")?;
    let JanetEnum::_String(asset_string) =
        JanetEnum::get::<GameContext>(env, "card-image", Some(name))
            .ok_or("Asset path not found")?
    else {
        return Err("Not a String");
    };
    asset_loader
        .load_texture(asset_string.as_str(), name)
        .await
        .map_err(|_| "Loading texture failed")?;
    let Some(JanetEnum::_Int(attack_strength)) =
        JanetEnum::get::<GameContext>(env, "attack-strength", Some(name))
    else {
        return Err("attack_strength not found");
    };
    let Some(JanetEnum::_Int(defense)) = JanetEnum::get::<GameContext>(env, "defense", Some(name))
    else {
        return Err("defense not found");
    };
    Ok(Card {
        name: name.to_string(),
        draw_action,
        play_action,
        discard_action,
        attack,
        attack_strength: attack_strength as u16,
        defense: defense as u16,
        movement,
    })
}
