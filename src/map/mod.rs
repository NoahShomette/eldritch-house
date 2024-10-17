//! Generates and interacts with the map

use bevy::{
    app::App,
    asset::{Asset, Handle},
    ecs::world::Command,
    prelude::{BuildWorldChildren, Component, Entity, Res, Resource},
    reflect::Reflect,
    utils::HashMap,
};
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{
        config::{ConfigureLoadingState, LoadingStateConfig},
        LoadingStateAppExt,
    },
    standard_dynamic_asset::StandardDynamicAssetCollection,
};
use bevy_common_assets::json::JsonAssetPlugin;
use serde::{Deserialize, Serialize};

use crate::AppLoadingState;

mod cleanup_map;
mod generate_map;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((generate_map::plugin, cleanup_map::plugin));
    app.add_plugins(JsonAssetPlugin::<RoomDefinition>::new(&["room.json"]));
    app.configure_loading_state(
        LoadingStateConfig::new(AppLoadingState::Loading)
            .with_dynamic_assets_file::<StandardDynamicAssetCollection>("rooms.assets.ron")
            .load_collection::<RoomAssets>(),
    );
}

/// Unique identifier of a room
#[derive(Component, Hash, PartialEq, Eq, Clone, Copy)]
pub struct RoomId(pub u8);

/// Information on the house
#[derive(Component)]
pub struct House {
    /// All the rooms in the house
    pub rooms: HashMap<RoomId, Entity>,
}

/// A room in the house
#[derive(Component)]
pub struct Room {
    /// The connections this room has to other rooms
    pub connections: HashMap<RoomConnectionDirection, RoomId>,
}

/// The directions that a room can connect in
#[derive(Reflect, Deserialize, Serialize, Hash, PartialEq, Eq, Clone)]
pub enum RoomConnectionDirection {
    North,
    East,
    South,
    West,
}

#[derive(Resource, AssetCollection, Reflect, Clone)]
pub struct RoomAssets {
    #[asset(key = "room_assets", collection(typed, mapped))]
    pub room_definitions: HashMap<String, Handle<RoomDefinition>>,
}

#[derive(Reflect, Asset, Deserialize, Serialize)]
struct RoomDefinition {
    pub room_name: String,
    pub allowed_directions: Vec<RoomConnectionDirection>,
}

/// Spawn a new room
pub struct SpawnRoom {
    pub house_entity: Entity,
    pub room_id: RoomId,
    pub room_connections: HashMap<RoomConnectionDirection, RoomId>,
}

impl Command for SpawnRoom {
    fn apply(self, world: &mut bevy::prelude::World) {
        let room = world
            .spawn((
                self.room_id,
                Room {
                    connections: self.room_connections,
                },
            ))
            .id();
        if let Some(mut house) = world.entity_mut(self.house_entity).get_mut::<House>() {
            house.rooms.insert(self.room_id, room);
        }
        world.entity_mut(self.house_entity).push_children(&[room]);
    }
}
