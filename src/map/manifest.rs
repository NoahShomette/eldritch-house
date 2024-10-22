use bevy::{prelude::*, utils::HashMap};
use leafwing_manifest::{
    asset_state::SimpleAssetState,
    identifier::Id,
    manifest::{Manifest, ManifestFormat},
    plugin::{ManifestPlugin, RegisterManifest},
};
use serde::{Deserialize, Serialize};
use toa_animator::{Animations, ArtCollection, TextureAsset};

use super::{RoomConnectionDirection, RoomDefinition};

pub(super) fn plugin(app: &mut App) {
    app.init_state::<SimpleAssetState>()
        .add_plugins(ManifestPlugin::<SimpleAssetState>::default())
        .register_manifest::<RoomDefinitionManifest>("rooms.assets.json");
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawRoomDefinition {
    pub room_name: String,
    pub allowed_directions: Vec<RoomConnectionDirection>,
    pub textures: HashMap<String, TextureAsset>,
    pub animations: Animations,
}

#[derive(Debug, Resource)]
pub struct RoomDefinitionManifest {
    pub items: HashMap<Id<RoomDefinition>, RoomDefinition>,
}
#[derive(Debug, Resource, Serialize, Deserialize, Asset, TypePath)]
pub struct RawRoomDefinitionManifest {
    items: Vec<RawRoomDefinition>,
}
impl Manifest for RoomDefinitionManifest {
    type RawManifest = RawRoomDefinitionManifest;

    type RawItem = RawRoomDefinition;

    type Item = RoomDefinition;

    type ConversionError = std::convert::Infallible;

    const FORMAT: leafwing_manifest::manifest::ManifestFormat = ManifestFormat::Json;

    fn from_raw_manifest(
        raw_manifest: Self::RawManifest,
        world: &mut World,
    ) -> Result<Self, Self::ConversionError> {
        let mut atlases = world
            .remove_resource::<Assets<TextureAtlasLayout>>()
            .unwrap();
        let asset_server = world.resource::<AssetServer>();

        let items: HashMap<_, _> = raw_manifest
            .items
            .into_iter()
            .map(|raw_item| {
                // Load the sprite from the path provided in the raw data
                let textures: HashMap<_, _> = raw_item
                    .textures
                    .into_iter()
                    .map(|raw_item| {
                        let texture = raw_item.1.load(asset_server, &mut atlases);
                        (raw_item.0, texture)
                    })
                    .collect();
                let item = RoomDefinition {
                    room_name: raw_item.room_name,
                    allowed_directions: raw_item.allowed_directions,
                    art_collection: ArtCollection {
                        animations: raw_item.animations,
                        textures,
                    },
                };

                // Build an Id for our item, so it can be looked up later
                let id = Id::from_name(&item.room_name);

                (id, item)
            })
            .collect();

        world.insert_resource(atlases);
        Ok(RoomDefinitionManifest { items })
    }

    fn get(&self, id: Id<Self::Item>) -> Option<&Self::Item> {
        self.items.get(&id)
    }
}
