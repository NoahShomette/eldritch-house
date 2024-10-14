use bevy::{ecs::world::Command, prelude::*, utils::hashbrown::HashMap};

use super::{House, RoomConnectionDirections, RoomId, SpawnRoom};

pub(super) fn plugin(app: &mut App) {}

pub struct GenerateMap {
    pub room_count: u8,
}

impl Command for GenerateMap {
    fn apply(self, world: &mut World) {
        // Randomly spawn rooms assigning ids. We basically just need to select a random room, check each neighbor for a free space, and then spawn a room there. 
        // Then after we have enough rooms iterate through each room and check its neighbors, filling in that rooms connections.
        // After iterating every room we will have all the rooms connections and can actually spawn them
        let mut map: HashMap<UVec2, (RoomId, HashMap<RoomConnectionDirections, RoomId>)> =
            HashMap::new();

        let house = world
            .spawn(House {
                rooms: HashMap::new(),
            })
            .id();

        SpawnRoom {
            house_entity: house,
            room_id: RoomId(0),
            room_connections: HashMap::new(),
        }
        .apply(world);
    }
}
