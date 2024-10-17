use bevy::{ecs::world::Command, prelude::*, utils::hashbrown::HashMap};
use rand::{seq::IteratorRandom, thread_rng};

use super::{House, RoomAssets, RoomConnectionDirection, RoomDefinition, RoomId, SpawnRoom};

pub(super) fn plugin(app: &mut App) {}

pub struct GenerateMap {
    pub room_count: u8,
}

impl Command for GenerateMap {
    fn apply(self, world: &mut World) {
        // Randomly spawn rooms assigning ids. We basically just need to select a random room, check each neighbor for a free space, and then spawn a room there.
        // Then after we have enough rooms iterate through each room and check its neighbors, filling in that rooms connections.
        // After iterating every room we will have all the rooms connections and can actually spawn them
        world.resource_scope(|world: &mut World, room_assets: Mut<RoomAssets>| {
            world.resource_scope(
                |world: &mut World, definition_assets: Mut<Assets<RoomDefinition>>| {
                    let mut map: HashMap<
                        UVec2,
                        (
                            UVec2,
                            RoomId,
                            HashMap<RoomConnectionDirection, RoomId>,
                            String,
                        ),
                    > = HashMap::new();

                    let house = world
                        .spawn(House {
                            rooms: HashMap::new(),
                        })
                        .id();

                    map.insert(
                        UVec2::new(0, 0),
                        (
                            UVec2::new(0, 0),
                            RoomId(0),
                            HashMap::new(),
                            "entrance".to_string(),
                        ),
                    );

                    let mut rng = thread_rng();

                    for i in 1..self.room_count {
                        let new_room_definition = if i == 0 {
                            definition_assets
                                .get(room_assets.room_definitions.get("entrance").unwrap())
                        } else {
                            definition_assets.get(
                                room_assets
                                    .room_definitions
                                    .values()
                                    .choose(&mut rng)
                                    .unwrap(),
                            )
                        };

                        let new_room_definition = new_room_definition.unwrap();
                        //
                        let mut selected_room = false;
                        while !selected_room {
                            let maybe_origin_room = map.values().choose(&mut rng).cloned().unwrap();
                            let origin_def = definition_assets
                                .get(
                                    room_assets
                                        .room_definitions
                                        .get(&maybe_origin_room.3)
                                        .unwrap(),
                                )
                                .unwrap();

                            for allowed_direction in origin_def.allowed_directions.iter() {
                                if maybe_origin_room.2.get(allowed_direction).is_some()
                                    && (maybe_origin_room.0.y as i32).saturating_sub(
                                        connection_direction_dif(allowed_direction).y,
                                    ) >= 0
                                {
                                    map.insert(
                                        maybe_origin_room.0.saturating_add_signed(
                                            connection_direction_dif(allowed_direction),
                                        ),
                                        (
                                            maybe_origin_room.0.saturating_add_signed(
                                                connection_direction_dif(allowed_direction),
                                            ),
                                            RoomId(i),
                                            HashMap::new(),
                                            new_room_definition.room_name.clone(),
                                        ),
                                    );
                                    selected_room = true;
                                } else {
                                    continue;
                                }
                            }
                        }
                    }

                    for (_pos, (pos, id, connections, room_definition_id)) in map {
                        SpawnRoom {
                            house_entity: house,
                            room_id: id,
                            room_connections: connections,
                        }
                        .apply(world);
                    }
                },
            );
        });
    }
}

fn connection_direction_dif(direction: &RoomConnectionDirection) -> IVec2 {
    match direction {
        RoomConnectionDirection::North => IVec2::new(0, 1),
        RoomConnectionDirection::East => IVec2::new(1, 0),
        RoomConnectionDirection::South => IVec2::new(0, -1),
        RoomConnectionDirection::West => IVec2::new(-1, 0),
    }
}
