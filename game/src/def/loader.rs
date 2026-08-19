//! Asset loading for `CardDef` RON files and the card registry setup plugin.

use bevy::asset::{
    io::Reader, AssetLoader, AssetServer, Assets, Handle, LoadContext, LoadedFolder,
};
use bevy::prelude::*;
use bevy::tasks::ConditionalSendFuture;

use crate::card::{card_id::CardID, card_registry::CardRegistry};

use super::card::CardDef;

/// Loading state for card assets.
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum LoadState {
    #[default]
    Loading,
    Ready,
}

/// Handle to the loaded `cards/` folder.
#[derive(Resource)]
pub struct LoadedCards(pub Handle<LoadedFolder>);

/// Custom asset loader for `.ron` card definitions.
#[derive(Default, TypePath)]
pub struct RonCardLoader;

/// Errors that can occur while loading a RON card.
#[derive(Debug)]
pub enum RonCardLoaderError {
    Io(std::io::Error),
    Ron(ron::error::SpannedError),
}

impl std::fmt::Display for RonCardLoaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RonCardLoaderError::Io(e) => write!(f, "io error: {}", e),
            RonCardLoaderError::Ron(e) => write!(f, "ron error: {}", e),
        }
    }
}

impl std::error::Error for RonCardLoaderError {}

impl From<std::io::Error> for RonCardLoaderError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<ron::error::SpannedError> for RonCardLoaderError {
    fn from(value: ron::error::SpannedError) -> Self {
        Self::Ron(value)
    }
}

impl AssetLoader for RonCardLoader {
    type Asset = CardDef;
    type Settings = ();
    type Error = RonCardLoaderError;

    fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext,
    ) -> impl ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let def = ron::de::from_bytes(&bytes)?;
            Ok(def)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}

fn startup_load_cards(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.insert_resource(LoadedCards(asset_server.load_folder("cards")));
}

fn build_card_registry(
    asset_server: Res<AssetServer>,
    folder: Res<LoadedCards>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    card_assets: Res<Assets<CardDef>>,
    mut registry: ResMut<CardRegistry>,
    mut next_state: ResMut<NextState<LoadState>>,
) {
    if !asset_server.is_loaded_with_dependencies(&folder.0) {
        return;
    }

    let Some(folder) = loaded_folders.get(&folder.0) else {
        return;
    };

    let mut entries: Vec<(String, CardDef)> = Vec::new();
    for untyped in &folder.handles {
        let handle = untyped.clone().typed::<CardDef>();
        let Some(path) = handle.path() else {
            continue;
        };
        let Some(def) = card_assets.get(&handle) else {
            continue;
        };
        let key = path.path().to_string_lossy().to_string();
        entries.push((key, def.clone()));
    }

    entries.sort_by(|a, b| a.0.cmp(&b.0));

    let mut new_registry = CardRegistry::new();
    for (index, (_path, def)) in entries.into_iter().enumerate() {
        new_registry.insert(CardID::new(index as u32), def);
    }

    *registry = new_registry;
    next_state.set(LoadState::Ready);
}

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<CardDef>()
            .register_asset_loader(RonCardLoader)
            .init_state::<LoadState>()
            .add_systems(Startup, startup_load_cards)
            .add_systems(
                Update,
                build_card_registry.run_if(in_state(LoadState::Loading)),
            );
    }
}
