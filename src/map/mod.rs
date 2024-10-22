//! Generates and interacts with the map

use bevy::{
    app::App,
    asset::{Asset, Handle},
    ecs::world::Command,
    math::Vec3,
    prelude::{
        default, BuildWorldChildren, Component, Entity, Image, Mut, Res, Resource, Transform, World,
    },
    reflect::{Reflect, TypePath},
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
use leafwing_manifest::{identifier::Id, manifest::Manifest};
use manifest::RoomDefinitionManifest;
use map_navigation::FocusedRoom;
use serde::{Deserialize, Serialize};
use toa_animator::ArtCollection;

use crate::AppLoadingState;

pub use generate_map::GenerateMap;
pub use map_navigation::{ChangeRoom, MapRoomIndex};

mod cleanup_map;
mod generate_map;
mod manifest;
mod map_navigation;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        generate_map::plugin,
        cleanup_map::plugin,
        map_navigation::plugin,
        manifest::plugin,
    ));
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
    pub room_def_id: Id<RoomDefinition>,
}

/// The directions that a room can connect in
#[derive(Reflect, Deserialize, Serialize, Hash, PartialEq, Eq, Clone, Debug)]
pub enum RoomConnectionDirection {
    North,
    East,
    South,
    West,
}

#[derive(Asset, Debug, TypePath)]
pub struct RoomDefinition {
    pub room_name: String,
    pub allowed_directions: Vec<RoomConnectionDirection>,
    pub art_collection: ArtCollection,
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
        world.resource_scope(
            |world: &mut World, room_assets: Mut<RoomDefinitionManifest>| {
                let room = world
                    .spawn((
                        self.room_id,
                        Room {
                            connections: self.room_connections,
                            room_def_id: Id::from_name(&self.room_def_id),
                        },
                        SpriteBundle {
                            transform: Transform::from_translation(Vec3::splat(
                                (self.room_id.0 as f32 + 1.0) * 1000.0,
                            )),
                            texture: room_assets
                                .get(Id::from_name(&self.room_def_id))
                                .unwrap()
                                .art_collection
                                .textures
                                .get("idle")
                                .unwrap()
                                .texture_handle()
                                .clone(),
                            ..default()
                        },
                    ))
                    .id();
                if let Some(mut house) = world.entity_mut(self.house_entity).get_mut::<House>() {
                    house.rooms.insert(self.room_id, room);
                }
                if self.room_id == RoomId(0) {
                    world
                        .entity_mut(room)
                        .insert((FocusedRoom, Transform::from_translation(Vec3::splat(0.0))));
                }
                world.entity_mut(self.house_entity).push_children(&[room]);
            },
        );
    }
}
