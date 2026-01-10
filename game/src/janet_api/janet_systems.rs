use bevy::{
    app::{Plugin, Startup},
    ecs::{schedule::IntoScheduleConfigs, system::NonSend},
};
use janet_bindings::{controller::Environment, types::janetabstract::JanetAbstract};

use crate::{
    actions::target_builder::janet::{AnyTargetBuilder, BUILDER_FUNCTIONS},
    janet_api::{core_constants::CORE_CONSTANTS, core_functions::CORE_FUNCTIONS},
};

pub fn read_core_functions(env: NonSend<Environment>) {
    for core_function in CORE_FUNCTIONS {
        env.register_function(core_function, Some("std"));
    }

    for core_constant in CORE_CONSTANTS {
        env.register_constant(core_constant);
    }
}

pub fn read_card_list(env: NonSend<Environment>) {
    env.read_script("game/scripts/loader.janet").unwrap();
}

pub fn register_builder_types(env: NonSend<Environment>) {
    for fun in BUILDER_FUNCTIONS {
        env.register_function(fun, Some("target"));
    }
    JanetAbstract::register::<AnyTargetBuilder>();
}

pub struct JanetSystem;

impl Plugin for JanetSystem {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(
            Startup,
            (read_core_functions, register_builder_types, read_card_list).chain(),
        );
    }
}
