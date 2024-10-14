//! Generates and interacts with the map

use bevy::{
    app::App,
    ecs::world::Command,
    prelude::{BuildWorldChildren, Component, Entity},
    utils::HashMap,
};

mod cleanup_map;
mod generate_map;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((generate_map::plugin, cleanup_map::plugin));
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
    pub connections: HashMap<RoomConnectionDirections, RoomId>,
}

/// The directions that a room can connect in
pub enum RoomConnectionDirections {
    North,
    East,
    South,
    West,
}

/// Spawn a new room
pub struct SpawnRoom {
    pub house_entity: Entity,
    pub room_id: RoomId,
    pub room_connections: HashMap<RoomConnectionDirections, RoomId>,
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
