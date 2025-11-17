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
        card::{abilities::Abilities, creature::Creature, Card},
        error::Error,
        game_action::{GameAction, Timing},
    },
};

fn destructure_action(action: JanetEnum) -> Result<Vec<GameAction>, Error> {
    if let JanetEnum::_Array(elements) = action {
        let mut result = Vec::new();
        for element in elements {
            if let JanetEnum::_Table(map) = element {
                let Some(JanetEnum::_Function(func)) = map.get("action") else {
                    return Err(Error::Cast("Action not found".into()));
                };

                if let Some(JanetEnum::_Array(arr)) = map.get("timing") {
                    // Inside your function processing the JanetEnum array
                    let timing = match arr.as_slice() {
                        [JanetEnum::_Int(turns), JanetEnum::_String(timing_str), ..] => {
                            // Convert signed integer to u32 safely
                            let turns_u32 = u32::try_from(*turns)
                                .map_err(|_| Error::Cast("Turn count out of u32 range".into()))?;

                            match timing_str.as_str() {
                                "end" => Timing::End(turns_u32),
                                "start" => Timing::Start(turns_u32),
                                _ => {
                                    return Err(Error::Cast(format!(
                                        "Invalid timing variant: {}",
                                        timing_str
                                    )))
                                }
                            }
                        }
                        [JanetEnum::_String(timing_str)] => match timing_str.as_str() {
                            "now" => Timing::Now,
                            _ => {
                                return Err(Error::Cast(format!(
                                    "Invalid timing variant: {}",
                                    timing_str
                                )))
                            }
                        },
                        _ => {
                            return Err(Error::Cast(
                                "Timing must be either [int, \"end|start\"] or [\"now\"]".into(),
                            ))
                        }
                    };

                    result.push(GameAction::new(func.to_owned(), timing));
                } else {
                    return Err(Error::Cast("Timing not found".into()));
                }
            } else {
                return Err(Error::Cast("Result is not a table".into()));
            }
        }
        Ok(result)
    } else {
        Err(Error::Cast("Value is not an array".into()))
    }
}

pub async fn read_card(
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

    let JanetEnum::_String(description) = JanetEnum::get(env, "description", Some(name))
        .ok_or(Error::NotFound("card-image".into()))?
    else {
        return Err(Error::Cast("Asset is not a string".into()));
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

    let Some(JanetEnum::_Int(cost)) = JanetEnum::get(env, "cost", Some(name)) else {
        return Err(Error::Cast("Cost not found".into()));
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

    Ok(Card::Creature(Creature {
        draw_action,
        place_action: play_action,
        turn_begin_action,
        turn_end_action,
        discard_action,
        name: name.to_string(),
        attack,
        attack_strength: attack_strength as u16,
        defense: defense as u16,
        movement_points: movement_points as u16,
        cost: cost as u16,
        movement,
        abilities,
        description,
    }))
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
