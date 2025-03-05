use std::collections::HashMap;

use log::debug;

use crate::{
    engine::{
        asset_loader::AssetLoader,
        janet_handler::{
            bindings::Janet,
            controller::Environment,
            types::{
                function::Function,
                janetenum::{convert_to_u16_vec, JanetEnum},
            },
        },
    },
    game::{error::Error, game_action::GameAction},
};

use super::Card;

fn destructure_action(action: JanetEnum) -> Result<Vec<GameAction>, Error> {
    if let JanetEnum::_Array(elements) = action {
        let mut result = Vec::new();
        for element in elements {
            if let JanetEnum::_HashMap(map) = element {
                let Some(JanetEnum::_Function(func)) = map.get("action") else {
                    return Err(Error::CastError);
                };
                let Some(JanetEnum::_UInt(timing)) = map.get("timing") else {
                    return Err(Error::CastError);
                };
                result.push(GameAction::new(func.to_owned(), *timing as u32));
            } else {
                return Err(Error::CastError);
            }
        }
        Ok(result)
    } else {
        return Err(Error::CastError);
    }
}

pub async fn read_card(
    env: &Environment,
    name: &str,
    asset_loader: &mut AssetLoader,
) -> Result<Card, Error> {
    let draw_action = match JanetEnum::get(env, "on-draw", Some(name)) {
        Some(value) => destructure_action(value)?,
        None => return Err(Error::NotFound),
    };

    let play_action = match JanetEnum::get(env, "on-play", Some(name)) {
        Some(value) => destructure_action(value)?,
        None => return Err(Error::NotFound),
    };

    let turn_begin_action = match JanetEnum::get(env, "on-turn-begin", Some(name)) {
        Some(value) => destructure_action(value)?,
        None => return Err(Error::NotFound),
    };

    let turn_end_action = match JanetEnum::get(env, "on-turn-end", Some(name)) {
        Some(value) => destructure_action(value)?,
        None => return Err(Error::NotFound),
    };

    let discard_action = match JanetEnum::get(env, "on-discard", Some(name)) {
        Some(value) => destructure_action(value)?,
        None => return Err(Error::NotFound),
    };

    let attack = convert_to_u16_vec(env, "attack", name).ok_or(Error::NotFound)?;
    let movement = convert_to_u16_vec(env, "movement", name).ok_or(Error::NotFound)?;
    let JanetEnum::_String(asset_string) =
        JanetEnum::get(env, "card-image", Some(name)).ok_or(Error::NotFound)?
    else {
        return Err(Error::CastError);
    };
    debug!("Reading in card{:?} {:?}", asset_string, name);
    asset_loader
        .load_texture(asset_string.as_str(), name)
        .await
        .map_err(|_| Error::CastError)?;

    let Some(JanetEnum::_Int(attack_strength)) = JanetEnum::get(env, "attack-strength", Some(name))
    else {
        return Err(Error::CastError);
    };

    let Some(JanetEnum::_Int(defense)) = JanetEnum::get(env, "defense", Some(name)) else {
        return Err(Error::CastError);
    };

    Ok(Card {
        draw_action,
        play_action,
        turn_begin_action,
        turn_end_action,
        discard_action,
        name: name.to_string(),
        attack,
        attack_strength: attack_strength as u16,
        defense: defense as u16,
        movement,
    })
}

pub fn get_card_list(env: &Environment) -> Option<Vec<String>> {
    let Some(JanetEnum::_Array(cards)) = JanetEnum::get(env, "cards", Some("cards")) else {
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
