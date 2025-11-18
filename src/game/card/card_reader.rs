use std::str::FromStr;

use crate::{
    engine::{
        asset_loader::AssetLoader,
        janet_handler::{
            controller::Environment,
            types::janetenum::{convert_to_i16_vec, JanetEnum},
        },
    },
    game::{
        card::{abilities::Abilities, card_builder::CardBuilder, Card},
        error::Error,
        game_action::{GameAction, Timing},
    },
};
/// Parse timing information from Janet array
fn parse_timing(arr: &[JanetEnum]) -> Result<Timing, Error> {
    match arr {
        [JanetEnum::_Int(turns), JanetEnum::_String(timing_str), ..] => {
            let turns_u32 = u32::try_from(*turns)
                .map_err(|_| Error::Cast("Turn count out of u32 range".into()))?;

            match timing_str.as_str() {
                "end" => Ok(Timing::End(turns_u32)),
                "start" => Ok(Timing::Start(turns_u32)),
                _ => Err(Error::Cast(format!(
                    "Invalid timing variant: {}",
                    timing_str
                ))),
            }
        }
        [JanetEnum::_String(timing_str)] => match timing_str.as_str() {
            "now" => Ok(Timing::Now),
            _ => Err(Error::Cast(format!(
                "Invalid timing variant: {}",
                timing_str
            ))),
        },
        _ => Err(Error::Cast(
            "Timing must be either [int, \"end|start\"] or [\"now\"]".into(),
        )),
    }
}

/// Convert Janet action data to GameAction vector
fn destructure_action(action: JanetEnum) -> Result<Vec<GameAction>, Error> {
    let JanetEnum::_Array(elements) = action else {
        return Err(Error::Cast("Value is not an array".into()));
    };

    let mut result = Vec::new();

    for element in elements {
        let JanetEnum::_Table(map) = element else {
            return Err(Error::Cast("Element is not a table".into()));
        };

        let Some(JanetEnum::_Function(func)) = map.get("action") else {
            return Err(Error::Cast("Action not found in table".into()));
        };

        let Some(JanetEnum::_Array(timing_arr)) = map.get("timing") else {
            return Err(Error::Cast("Timing not found in table".into()));
        };
        let Some(JanetEnum::_Table(targeting_type)) = map.get("targeting") else {
            return Err(Error::Cast("Targeting Type not found".into()));
        };

        let timing = parse_timing(timing_arr.as_slice())?;
        result.push(GameAction::new(func.to_owned(), timing));
    }

    Ok(result)
}

pub fn read_common_data(
    env: &Environment,
    name: &str,
    asset_loader: &mut AssetLoader,
) -> Result<CardBuilder, Error> {
    let JanetEnum::_String(description) = JanetEnum::get(env, "description", Some(name))
        .ok_or(Error::NotFound("Description".into()))?
    else {
        return Err(Error::NotFound("Description".into()));
    };

    let Some(JanetEnum::_Int(cost)) = JanetEnum::get(env, "cost", Some(name)) else {
        return Err(Error::NotFound("Attack strength".into()));
    };

    let JanetEnum::_String(display_image_asset_string) =
        JanetEnum::get(env, "display-image-asset-string", Some(name))
            .ok_or(Error::NotFound("Display image".into()))?
    else {
        return Err(Error::NotFound("Display image".into()));
    };

    Ok(Card::builder()
        .name(name)
        .cost(cost as u16)
        .description(description)
        .display_image_asset_string(display_image_asset_string))
}

pub async fn read_creature(
    env: &Environment,
    name: &str,
    asset_loader: &mut AssetLoader,
) -> Result<Card, Error> {
    println!("Reading card: {}", name);

    let draw_action = match JanetEnum::get(env, "on-draw", Some(name)) {
        Some(value) => destructure_action(value)?,
        None => return Err(Error::NotFound("on-draw".into())),
    };

    let play_action = match JanetEnum::get(env, "on-play", Some(name)) {
        Some(value) => destructure_action(value)?,
        None => return Err(Error::NotFound("on-play".into())),
    };

    let turn_begin_action = match JanetEnum::get(env, "on-turn-begin", Some(name)) {
        Some(value) => destructure_action(value)?,
        None => return Err(Error::NotFound("on-turn-begin".into())),
    };

    let turn_end_action = match JanetEnum::get(env, "on-turn-end", Some(name)) {
        Some(value) => destructure_action(value)?,
        None => return Err(Error::NotFound("on-turn-end".into())),
    };

    let discard_action = match JanetEnum::get(env, "on-discard", Some(name)) {
        Some(value) => destructure_action(value)?,
        None => return Err(Error::NotFound("on-discard".into())),
    };

    let attack = convert_to_i16_vec(env, "attack", name)
        .ok_or(Error::NotFound(format!("{}, attack", name)))?;
    let movement = convert_to_i16_vec(env, "movement", name)
        .ok_or(Error::NotFound(format!("{}, attack", name)))?;

    let JanetEnum::_String(asset_string) = JanetEnum::get(env, "card-image", Some(name))
        .ok_or(Error::NotFound("card-image".into()))?
    else {
        return Err(Error::Cast("Asset is not a string".into()));
    };
    asset_loader
        .load_texture(asset_string.as_str(), name)
        .await
        .map_err(Error::EngineError)?;

    let Some(JanetEnum::_Int(attack_strength)) = JanetEnum::get(env, "attack-strength", Some(name))
    else {
        return Err(Error::Cast("Attack strength not found".into()));
    };

    let Some(JanetEnum::_Int(defense)) = JanetEnum::get(env, "defense", Some(name)) else {
        return Err(Error::Cast("Defense strength not found".into()));
    };

    let Some(JanetEnum::_Int(movement_points)) = JanetEnum::get(env, "movement-points", Some(name))
    else {
        return Err(Error::Cast("Movement Points not found".into()));
    };

    let abilities: Vec<Abilities> = match JanetEnum::get(env, "abilities", Some(name)) {
        Some(JanetEnum::_Array(abilities)) => abilities
            .iter()
            .map(|el| {
                let s: String = el.try_into().map_err(Error::EngineError)?; // try convert to String
                Abilities::from_str(&s)
            })
            .collect::<Result<_, Error>>()?,

        _ => Vec::new(),
    };

    let card_builder = read_common_data(env, name, asset_loader)?;
    card_builder
        .movement(movement)
        .movement_points(movement_points as u16)
        .attack_strength(attack_strength as u16)
        .attack_pattern(attack)
        .defense(defense as u16)
        .on_play_action(play_action)
        .turn_begin_action(turn_begin_action)
        .turn_end_action(turn_end_action)
        .draw_action(draw_action)
        .discard_action(discard_action)
        .abilities(abilities)
        .build_creature()
}

pub async fn read_spell(
    env: &Environment,
    name: &str,
    asset_loader: &mut AssetLoader,
) -> Result<Card, Error> {
    let play_action = match JanetEnum::get(env, "on-play", Some(name)) {
        Some(value) => destructure_action(value)?,
        None => return Err(Error::NotFound("on-play".into())),
    };

    let card_builder = read_common_data(env, name, asset_loader)?;
    card_builder.on_play_action(play_action).build_spell()
}

pub async fn read_trap(
    env: &Environment,
    name: &str,
    asset_loader: &mut AssetLoader,
) -> Result<Card, Error> {
    let place_action = match JanetEnum::get(env, "on-play", Some(name)) {
        Some(value) => destructure_action(value)?,
        None => return Err(Error::NotFound("on-play".into())),
    };

    let reveal_action = match JanetEnum::get(env, "on-reveal", Some(name)) {
        Some(value) => destructure_action(value)?,
        None => return Err(Error::NotFound("on-reveal".into())),
    };

    let card_builder = read_common_data(env, name, asset_loader)?;
    card_builder
        .place_action(place_action)
        .reveal_action(reveal_action)
        .build_trap()
}

pub fn get_card_list(env: &Environment) -> Option<(Vec<String>, Vec<String>, Vec<String>)> {
    let Some(JanetEnum::_Array(creatures)) = JanetEnum::get(env, "creatures", Some("cards")) else {
        return None;
    };

    let creature_names: Vec<String> = creatures
        .into_iter()
        .map(|x| match x {
            JanetEnum::_String(s) => Some(s),
            _ => None,
        })
        .collect::<Option<Vec<String>>>()?;

    let Some(JanetEnum::_Array(spells)) = JanetEnum::get(env, "spells", Some("cards")) else {
        return None;
    };

    let spell_names: Vec<String> = spells
        .into_iter()
        .map(|x| match x {
            JanetEnum::_String(s) => Some(s),
            _ => None,
        })
        .collect::<Option<Vec<String>>>()?;

    let Some(JanetEnum::_Array(traps)) = JanetEnum::get(env, "traps", Some("cards")) else {
        return None;
    };

    let trap_names: Vec<String> = traps
        .into_iter()
        .map(|x| match x {
            JanetEnum::_String(s) => Some(s),
            _ => None,
        })
        .collect::<Option<Vec<String>>>()?;

    Some((creature_names, spell_names, trap_names))
}
