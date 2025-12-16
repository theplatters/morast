use bevy::prelude::*;

use crate::{
    engine::{asset_loader::AssetLoader, janet_handler::controller::Environment},
    game::{
        card::{card_registry::CardRegistry, deck_builder::DeckBuilder, InDeck},
        components::Owner,
        player::*,
    },
};

pub fn add_player(mut commands: Commands) {
    commands.spawn((Player { number: 0 }, PlayerBundle::default(), TurnPlayer));
    commands.spawn((Player { number: 1 }, PlayerBundle::default()));
}

pub fn add_cards(
    card_registry: Res<CardRegistry>,
    players: Query<(&Player, &mut Deck)>,
    mut commands: Commands,
) {
    for (player_id, mut deck) in players {
        let cards: Vec<_> = DeckBuilder::standard_deck(&card_registry)
            .iter()
            .map(move |id| (*id, Owner(*player_id), InDeck))
            .collect();

        for card in cards {
            let id = commands.spawn(card).id();
            deck.0.push(id);
        }
    }
}

pub fn init_card_registry(
    mut card_registry: ResMut<CardRegistry>,
    mut environment: NonSendMut<Environment>,
    mut asset_loader: ResMut<AssetLoader>,
) {
    card_registry.init(&mut environment, &mut asset_loader);
}
