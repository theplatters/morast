use bevy::{
    app::{Plugin, Startup},
    ecs::{error::Result, schedule::IntoScheduleConfigs, system::NonSend},
};
use janet_bindings::{controller::Environment, types::janetabstract::JanetAbstract};

use crate::{
    actions::targeting::target_builder::janet::{AnyTargetBuilder, BUILDER_FUNCTIONS},
    janet_api::{core_constants::CORE_CONSTANTS, core_functions::CORE_FUNCTIONS},
};

pub fn register_api(env: NonSend<Environment>) -> Result {
    for core_function in CORE_FUNCTIONS {
        env.register_function(core_function, Some("std"));
    }

    for core_constant in CORE_CONSTANTS {
        env.register_constant(core_constant)?;
    }

    for fun in BUILDER_FUNCTIONS {
        env.register_function(fun, Some("target"));
    }
    JanetAbstract::register::<AnyTargetBuilder>();
    Ok(())
}

pub fn read_card_list(env: NonSend<Environment>) {
    env.read_script("game/scripts/loader.janet").unwrap();
}

pub fn register_builder_types(_env: NonSend<Environment>) {}

pub struct JanetSystem;

impl Plugin for JanetSystem {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_non_send_resource(Environment::new())
            .add_systems(Startup, (register_api, read_card_list).chain());
    }
}
