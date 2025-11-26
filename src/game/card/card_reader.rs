use std::str::FromStr;

use macroquad::math::I16Vec2;

use crate::{
    engine::{
        asset_loader::AssetLoader,
        janet_handler::{
            controller::Environment,
            types::{
                janetenum::{to_i16_vec, JanetEnum},
                table::Table,
            },
        },
    },
    game::{
        card::{abilities::Abilities, Card},
        error::Error,
        events::{action::Action, action_parser::ActionParser},
    },
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

    fn get_string(&self, field: &str) -> Result<String, Error> {
        match self.table.get(field) {
            Some(JanetEnum::String(value)) => Ok(value.clone()),
            _ => Err(Error::NotFound(format!("{}: {}", self.context, field))),
        }
    }

    fn get_int(&self, field: &str) -> Result<i32, Error> {
        match self.table.get(field) {
            Some(JanetEnum::Int(value)) => Ok(value),
            _ => Err(Error::NotFound(format!("{}: {}", self.context, field))),
        }
    }

    fn get_optional_actions(&self, field: &str) -> Result<Option<Action>, Error> {
        match self.table.get(field) {
            Some(value) => ActionParser::parse(&value),
            None => Ok(None),
        }
    }

    fn get_required_actions(&self, field: &str) -> Result<Action, Error> {
        match self.table.get(field) {
            Some(value) => ActionParser::parse(&value)
                .transpose()
                .ok_or(Error::Incomplete("Requires action"))?,
            None => Err(Error::NotFound(format!("{}: {}", self.context, field))),
        }
    }

    fn get_i16_vec(&self, field: &str) -> Result<Vec<I16Vec2>, Error> {
        let value = self
            .table
            .get(field)
            .ok_or_else(|| Error::NotFound(format!("{}: {}", self.context, field)))?;

        to_i16_vec(value)
            .ok_or_else(|| Error::Cast(format!("Failed to cast {} to i16 vector", field)))
    }

    fn get_abilities(&self) -> Result<Vec<Abilities>, Error> {
        match self.table.get("abilities") {
            Some(JanetEnum::Array(abilities)) => abilities
                .iter()
                .map(|el| {
                    let s: String = el.try_into().map_err(Error::EngineError)?;
                    Abilities::from_str(&s)
                        .map_err(|e| Error::Cast(format!("Ability could not be casted {}", e)))
                })
                .collect(),
            _ => Ok(Vec::new()),
        }
    }
}

// Separate card data retrieval logic
struct CardDataRetriever;

impl CardDataRetriever {
    fn get_card_table(env: &Environment, name: &str) -> Result<Table, Error> {
        match JanetEnum::get(env, name, Some(name)) {
            Some(JanetEnum::Table(card_data)) => Ok(card_data),
            Some(_) => Err(Error::Cast("Card data is not in table format".into())),
            None => Err(Error::NotFound(format!("Card: {}", name))),
        }
    }

    fn get_action_from_env(
        env: &Environment,
        action_name: &str,
        card_name: &str,
    ) -> Result<Option<Action>, Error> {
        match JanetEnum::get(env, action_name, Some(card_name)) {
            Some(value) => ActionParser::parse(&value),
            None => Err(Error::NotFound(format!("{}: {}", card_name, action_name))),
        }
    }
}

pub fn read_common_data(
    card_data: &Table,
    _env: &Environment,
    name: &str,
    _asset_loader: &mut AssetLoader,
) -> Result<CommonData, Error> {
    let extractor = FieldExtractor::new(card_data, name);

    Ok(CommonData {
        name: name.to_string(),
        description: extractor.get_string("description")?,
        cost: extractor.get_int("cost")? as u16,
        display_image_asset_string: extractor.get_string("display-image-asset-string")?,
    })
}

pub async fn read_creature(
    env: &Environment,
    name: &str,
    asset_loader: &mut AssetLoader,
) -> Result<Card, Error> {
    println!("Reading card: {}", name);

    let card_data = CardDataRetriever::get_card_table(env, name)?;
    let extractor = FieldExtractor::new(&card_data, name);

    let common_data = read_common_data(&card_data, env, name, asset_loader)?;

    // Parse all actions
    let draw_action = extractor.get_optional_actions("on-draw")?;
    let play_action = extractor.get_optional_actions("on-play")?;
    let turn_begin_action = extractor.get_optional_actions("on-turn-begin")?;
    let turn_end_action = extractor.get_optional_actions("on-turn-end")?;
    let discard_action = extractor.get_optional_actions("on-discard")?;

    // Parse creature-specific fields
    let attack = extractor.get_i16_vec("attack")?;
    let movement = extractor.get_i16_vec("movement")?;
    let attack_strength = extractor.get_int("attack-strength")? as u16;
    let defense = extractor.get_int("defense")? as u16;
    let movement_points = extractor.get_int("movement-points")? as u16;
    let abilities = extractor.get_abilities()?;

    println!("Card {} - Play action {:?}", name, play_action);

    Card::builder()
        .common_data(common_data)
        .creature()
        .movement(movement)
        .movement_points(movement_points)
        .attack_strength(attack_strength)
        .attack_pattern(attack)
        .defense(defense)
        .on_play_action_option(play_action)
        .turn_begin_action_option(turn_begin_action)
        .turn_end_action_option(turn_end_action)
        .draw_action_option(draw_action)
        .discard_action_option(discard_action)
        .abilities(abilities)
        .build()
}

pub async fn read_spell(
    env: &Environment,
    name: &str,
    asset_loader: &mut AssetLoader,
) -> Result<Card, Error> {
    println!("Reading card: {}", name);
    let card_data = CardDataRetriever::get_card_table(env, name)?;
    let extractor = FieldExtractor::new(&card_data, name);

    let common_data = read_common_data(&card_data, env, name, asset_loader)?;
    let play_action = extractor.get_required_actions("on-play")?;

    Card::builder()
        .common_data(common_data)
        .spell()
        .on_play_action(play_action)
        .build()
}

pub async fn read_trap(
    env: &Environment,
    name: &str,
    asset_loader: &mut AssetLoader,
) -> Result<Card, Error> {
    let card_data = CardDataRetriever::get_card_table(env, name)?;
    let common_data = read_common_data(&card_data, env, name, asset_loader)?;

    let place_action = CardDataRetriever::get_action_from_env(env, "on-play", name)?;
    let reveal_action = CardDataRetriever::get_action_from_env(env, "on-reveal", name)?;

    Card::builder()
        .common_data(common_data)
        .trap()
        .on_play_action(place_action)
        .reveal_action_optional(reveal_action)
        .build()
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
