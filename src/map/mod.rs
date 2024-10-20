//! Generates and interacts with the map

use bevy::{
    app::App,
    asset::{Asset, Handle},
    ecs::world::Command,
    math::Vec3,
    prelude::{
        default, BuildWorldChildren, Component, Entity, Image, Mut, Res, Resource, Transform, World,
    },
    reflect::Reflect,
    sprite::SpriteBundle,
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

pub use generate_map::GenerateMap;
pub use map_navigation::{ChangeRoom, MapRoomIndex};

mod cleanup_map;
mod generate_map;
mod map_navigation;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        generate_map::plugin,
        cleanup_map::plugin,
        map_navigation::plugin,
    ));
    app.add_plugins(JsonAssetPlugin::<RoomDefinition>::new(&["room.json"]));
    app.configure_loading_state(
        LoadingStateConfig::new(AppLoadingState::Loading)
            .with_dynamic_assets_file::<StandardDynamicAssetCollection>("rooms.assets.ron")
            .load_collection::<RoomAssets>(),
    );
}

/// Unique identifier of a room
#[derive(Component, Hash, PartialEq, Eq, Clone, Copy, Debug)]
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
#[derive(Reflect, Deserialize, Serialize, Hash, PartialEq, Eq, Clone, Debug)]
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
    #[asset(path = "images/entrance.png")]
    pub texture: Handle<Image>,
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
    pub room_def_id: String,
}

impl Command for SpawnRoom {
    fn apply(self, world: &mut bevy::prelude::World) {
        world.resource_scope(|world: &mut World, room_assets: Mut<RoomAssets>| {
            let room = world
                .spawn((
                    self.room_id,
                    Room {
                        connections: self.room_connections,
                    },
                    SpriteBundle {
                        transform: Transform::from_translation(Vec3::splat(
                            self.room_id.0 as f32 * 1000.0,
                        )),
                        texture: room_assets.texture.clone(),
                        ..default()
                    },
                ))
                .id();
            if let Some(mut house) = world.entity_mut(self.house_entity).get_mut::<House>() {
                house.rooms.insert(self.room_id, room);
            }
            world.entity_mut(self.house_entity).push_children(&[room]);
        });
    }
}
