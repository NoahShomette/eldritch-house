use std::ops::Add;

use bevy::{ecs::world::Command, prelude::*, utils::hashbrown::HashMap};
use leafwing_manifest::{identifier::Id, manifest::Manifest};
use rand::{seq::IteratorRandom, thread_rng};

use crate::screens::Screen;

use super::{
    manifest::RoomDefinitionManifest, House, RoomConnectionDirection, RoomDefinition, RoomId,
    SpawnRoom,
};

pub(super) fn plugin(app: &mut App) {}

pub struct GenerateMap {
    pub room_count: u8,
}

impl Command for GenerateMap {
    fn apply(self, world: &mut World) {
        let mut map: HashMap<
            IVec2,
            (
                IVec2,
                RoomId,
                HashMap<RoomConnectionDirection, RoomId>,
                String,
            ),
        > = HashMap::new();

        let house = world
            .spawn((
                House {
                    rooms: HashMap::new(),
                },
                VisibilityBundle::default(),
                TransformBundle::from_transform(Transform::from_translation(Vec3::splat(0.0))),
                StateScoped(Screen::Gameplay),
            ))
            .id();

        let mut room_connections: HashMap<IVec2, HashMap<RoomConnectionDirection, RoomId>> =
            HashMap::new();
        // Randomly spawn rooms assigning ids. We basically just need to select a random room, check each neighbor for a free space, and then spawn a room there.
        // Then after we have enough rooms iterate through each room and check its neighbors, filling in that rooms connections.
        // After iterating every room we will have all the rooms connections and can actually spawn them
        world.resource_scope(
            |_world: &mut World, room_assets: Mut<RoomDefinitionManifest>| {
                map.insert(
                    IVec2::new(0, 0),
                    (
                        IVec2::new(0, 0),
                        RoomId(0),
                        HashMap::new(),
                        "entrance".to_string(),
                    ),
                );

                let mut rng = thread_rng();

                let mut room_count = 1;

                // Generate all the initial rooms connected to each other
                'new_room: while room_count != self.room_count {
                    let new_room_definition = room_assets.items.values().choose(&mut rng).unwrap();
                    //

                    println!("room count: {}", map.len());
                    for i in map.keys() {
                        let Some(maybe_origin_room) = map.get(i) else {
                            continue;
                        };
                        let origin_def = room_assets
                            .get(Id::from_name(&maybe_origin_room.3))
                            .unwrap();

                        for allowed_direction in origin_def.allowed_directions.iter() {
                            if (maybe_origin_room.0.y as i32)
                                .add(connection_direction_dif(allowed_direction).y)
                                >= 0
                                && map
                                    .get(
                                        &maybe_origin_room
                                            .0
                                            .add(connection_direction_dif(allowed_direction)),
                                    )
                                    .is_none()
                            {
                                let opposite = get_opposite_direction(allowed_direction);
                                if new_room_definition.allowed_directions.contains(&opposite) {
                                    room_count += 1;
                                    map.insert(
                                        maybe_origin_room
                                            .0
                                            .add(connection_direction_dif(allowed_direction)),
                                        (
                                            maybe_origin_room
                                                .0
                                                .add(connection_direction_dif(allowed_direction)),
                                            RoomId(room_count),
                                            HashMap::new(),
                                            String::from(&new_room_definition.room_name),
                                        ),
                                    );
                                }

                                continue 'new_room;
                            } else {
                                println!("invalid room pos");
                            }
                        }
                    }
                }

                // Tie all the rooms together through their connections. For each room get

                for (_pos, (pos, _, _, room_definition_id)) in &map {
                    let origin_def = room_assets.get(Id::from_name(&room_definition_id)).unwrap();
                    for allowed_direction in origin_def.allowed_directions.iter() {
                        if map
                            .get(&pos.add(connection_direction_dif(allowed_direction)))
                            .is_some()
                        {
                            let other_room = map
                                .get(&pos.add(connection_direction_dif(allowed_direction)))
                                .unwrap();

                            let other_def = room_assets.get(Id::from_name(&other_room.3)).unwrap();

                            let opposite = get_opposite_direction(allowed_direction);
                            if other_def.allowed_directions.contains(&opposite) {
                                let room_connection =
                                    room_connections.entry(pos.clone()).or_default();
                                room_connection.insert(allowed_direction.clone(), other_room.1);
                            }
                        } else {
                            continue;
                        }
                    }
                }
            },
        );
        for (_pos, (pos, id, _connections, room_definition_id)) in map {
            println!("spawning room at pos: {}", pos);
            SpawnRoom {
                house_entity: house,
                room_id: id,
                room_connections: room_connections.get(&pos).unwrap().clone(),
                room_def_id: room_definition_id,
            }
            .apply(world);
        }
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

fn get_opposite_direction(direction: &RoomConnectionDirection) -> RoomConnectionDirection {
    match direction {
        RoomConnectionDirection::North => RoomConnectionDirection::South,
        RoomConnectionDirection::East => RoomConnectionDirection::West,
        RoomConnectionDirection::South => RoomConnectionDirection::North,
        RoomConnectionDirection::West => RoomConnectionDirection::East,
    }
}
