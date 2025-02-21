use log::debug;
use macroquad::math::I16Vec2;

use crate::{
    engine::{
        asset_loader::AssetLoader,
        janet_handler::{
            controller::Environment,
            types::{
                function::Function,
                janetenum::{convert_to_u16_vec, JanetEnum},
            },
        },
    },
    game::game_context::GameContext,
};

use super::Card;

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
    let turn_begin_action =
        Function::get_method(env, "on-turn-begin", Some(name)).ok_or("on-turn-begin not found")?;

    let turn_end_action =
        Function::get_method(env, "on-turn-end", Some(name)).ok_or("on-turn-end not found")?;

    let attack = convert_to_u16_vec(env, "attack", name).ok_or("attack not found")?;
    let movement = convert_to_u16_vec(env, "movement", name).ok_or("movement not found")?;
    let JanetEnum::_String(asset_string) =
        JanetEnum::get::<GameContext>(env, "card-image", Some(name))
            .ok_or("Asset path not found")?
    else {
        return Err("Asset path is not a String");
    };
    debug!("Reading in card{:?} {:?}", asset_string, name);
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
        turn_begin_action,
        turn_end_action,
        attack,
        attack_strength: attack_strength as u16,
        defense: defense as u16,
        movement,
    })
}

pub fn get_card_list(env: &Environment) -> Option<Vec<String>> {
    let Some(JanetEnum::_Array(cards)) = JanetEnum::get::<GameContext>(env, "cards", Some("cards"))
    else {
        return None;
    };

    cards
        .into_iter()
        .map(|x| match x {
            JanetEnum::_String(s) => Some(s),
            _ => None,
        })
        .collect()
}
