use std::str::FromStr;

use bevy::{ecs::error::BevyError, math::I16Vec2};
use janet_bindings::{
    controller::Environment,
    types::{janetenum::JanetEnum, table::Table},
};

use crate::{
    actions::{
        GameAction,
        action_parser::{ActionParser, ParseError},
    },
    card::{Card, abilities::Abilities},
    janet_api::api::to_i16_vec,
};

pub struct CommonData {
    pub name: String,
    pub cost: u16,
    pub description: String,
    pub display_image_asset_string: String,
}

// Centralized field extraction with better error handling
struct FieldExtractor<'a> {
    table: &'a Table,
    context: &'a str,
}

impl<'a> FieldExtractor<'a> {
    fn new(table: &'a Table, context: &'a str) -> Self {
        Self { table, context }
    }

    fn get_string(&self, field: &str) -> Result<String, ParseError> {
        match self.table.get(field) {
            Some(JanetEnum::String(value)) => Ok(value.clone()),
            _ => Err(ParseError::NotFound(format!("{}: {}", self.context, field))),
        }
    }

    fn get_int(&self, field: &str) -> Result<i32, ParseError> {
        match self.table.get(field) {
            Some(JanetEnum::Int(value)) => Ok(value),
            _ => Err(ParseError::NotFound(format!("{}: {}", self.context, field))),
        }
    }

    fn get_actions(&self) -> Vec<Result<GameAction, ParseError>> {
        let actions = self
            .table
            .get_array("actions")
            .unwrap_or_default()
            .iter()
            .map(ActionParser::parse_action)
            .collect();
        actions
    }

    fn get_required_actions(&self, field: &str) -> Result<GameAction, ParseError> {
        match self.table.get(field) {
            Some(_value) => todo!(),
            None => Err(ParseError::NotFound(format!("{}: {}", self.context, field))),
        }
    }

    fn get_i16_vec(&self, field: &str) -> Result<Vec<I16Vec2>, ParseError> {
        let value = self
            .table
            .get(field)
            .ok_or_else(|| ParseError::NotFound(format!("{}: {}", self.context, field)))?;

        to_i16_vec(value)
            .ok_or_else(|| ParseError::Cast(format!("Failed to cast {} to i16 vector", field)))
    }

    fn get_abilities(&self) -> Result<Vec<Abilities>, ParseError> {
        match self.table.get("abilities") {
            Some(JanetEnum::Array(abilities)) => abilities
                .iter()
                .map(|el| {
                    let s: String = el.try_into().map_err(ParseError::JanetError)?;
                    Abilities::from_str(&s)
                        .map_err(|e| ParseError::Cast(format!("Ability could not be casted {}", e)))
                })
                .collect(),
            _ => Ok(Vec::new()),
        }
    }
}

// Separate card data retrieval logic
struct CardDataRetriever;

impl CardDataRetriever {
    fn get_card_table(env: &Environment, name: &str) -> Result<Table, ParseError> {
        match JanetEnum::get(env, name, Some(name)) {
            Some(JanetEnum::Table(card_data)) => Ok(card_data),
            Some(_) => Err(ParseError::Cast("Card data is not in table format".into())),
            None => Err(ParseError::NotFound(format!("Card: {}", name))),
        }
    }

    fn get_action_from_env(
        _env: &Environment,
        _action_name: &str,
        _card_name: &str,
    ) -> Result<Option<GameAction>, BevyError> {
        todo!()
    }
}

pub fn read_common_data(
    card_data: &Table,
    _env: &Environment,
    name: &str,
) -> Result<CommonData, ParseError> {
    let extractor = FieldExtractor::new(card_data, name);

    Ok(CommonData {
        name: name.to_string(),
        description: extractor.get_string("description")?,
        cost: extractor.get_int("cost")? as u16,
        display_image_asset_string: extractor.get_string("display-image-asset-string")?,
    })
}

pub fn read_creature(env: &Environment, name: &str) -> Result<Card, BevyError> {
    println!("Reading card: {}", name);

    let card_data = CardDataRetriever::get_card_table(env, name)?;
    let extractor = FieldExtractor::new(&card_data, name);

    let common_data = read_common_data(&card_data, env, name)?;

    // Parse all actions

    // Parse creature-specific fields
    let attack = extractor.get_i16_vec("attack")?;
    let movement = extractor.get_i16_vec("movement")?;
    let attack_strength = extractor.get_int("attack-strength")? as u16;
    let defense = extractor.get_int("defense")? as u16;
    let movement_points = extractor.get_int("movement-points")? as u16;
    let abilities = extractor.get_abilities()?;

    Ok(Card::builder()
        .common_data(common_data)
        .creature()
        .movement(movement)
        .movement_points(movement_points)
        .attack_strength(attack_strength)
        .attack_pattern(attack)
        .defense(defense)
        .abilities(abilities)
        .build()?)
}

pub fn read_spell(env: &Environment, name: &str) -> Result<Card, BevyError> {
    println!("Reading card: {}", name);
    let card_data = CardDataRetriever::get_card_table(env, name)?;
    let extractor = FieldExtractor::new(&card_data, name);

    let common_data = read_common_data(&card_data, env, name)?;
    let play_action = extractor.get_required_actions("on-play")?;

    Ok(Card::builder()
        .common_data(common_data)
        .spell()
        .on_play_action(play_action)
        .build()?)
}

pub fn read_trap(env: &Environment, name: &str) -> Result<Card, BevyError> {
    let card_data = CardDataRetriever::get_card_table(env, name)?;
    let common_data = read_common_data(&card_data, env, name)?;

    let place_action = CardDataRetriever::get_action_from_env(env, "on-play", name)?;
    let reveal_action = CardDataRetriever::get_action_from_env(env, "on-reveal", name)?.unwrap();

    Ok(Card::builder()
        .common_data(common_data)
        .trap()
        .on_play_action(place_action)
        .reveal_action(reveal_action)
        .build()?)
}

// Extract string list parsing to a helper
fn extract_string_list(janet_array: Vec<JanetEnum>) -> Option<Vec<String>> {
    janet_array
        .into_iter()
        .map(|x| match x {
            JanetEnum::String(s) => Some(s),
            _ => None,
        })
        .collect()
}

pub fn get_card_list(env: &Environment) -> Option<(Vec<String>, Vec<String>, Vec<String>)> {
    let creatures = match JanetEnum::get(env, "creatures", Some("cards"))? {
        JanetEnum::Array(arr) => extract_string_list(arr)?,
        _ => return None,
    };

    let spells = match JanetEnum::get(env, "spells", Some("cards"))? {
        JanetEnum::Array(arr) => extract_string_list(arr)?,
        _ => return None,
    };

    let traps = match JanetEnum::get(env, "traps", Some("cards"))? {
        JanetEnum::Array(arr) => extract_string_list(arr)?,
        _ => return None,
    };

    Some((creatures, spells, traps))
}
