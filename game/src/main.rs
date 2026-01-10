use bevy::{math::I16Vec2, prelude::*};
use janet_bindings::{
    bindings::{
        JanetArray, janet_array, janet_array_push, janet_getarray, janet_getinteger64,
        janet_wrap_array, janet_wrap_integer,
    },
    controller::Environment,
    error::JanetError,
    types::janetenum::JanetEnum,
};

mod actions;
mod board;
mod card;
mod components;
mod error;
mod events;
mod janet_api;
mod phases;
mod player;
mod renderer;
mod turn_controller;

use crate::{
    actions::action_systems::ActionPlugin,
    board::BoardPlugin,
    card::{
        add_cards,
        card_registry::{CardRegistry, init_card_registry},
    },
    events::GameMessagesPlugin,
    janet_api::janet_systems::read_card_list,
    player::{add_player, draw_starting_cards},
    renderer::RendererPlugin,
    turn_controller::TurnControllerPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(CardRegistry::new())
        .init_non_send_resource::<Environment>()
        .add_systems(
            Startup,
            (
                init_card_registry,
                add_player,
                add_cards,
                draw_starting_cards,
            )
                .chain(),
        )
        .add_plugins((
            GameMessagesPlugin,
            BoardPlugin,
            TurnControllerPlugin,
            RendererPlugin,
            ActionPlugin,
        ))
        .run();
}
