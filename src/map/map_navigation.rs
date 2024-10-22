use bevy::prelude::*;

use crate::screens::Screen;

use super::{Room, RoomId};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(MapRoomIndex(RoomId(0)));
    app.add_systems(OnEnter(Screen::Gameplay), |mut commands: Commands| {
        commands.insert_resource(MapRoomIndex(RoomId(0)));
    });

    app.add_systems(
        Update,
        (change_room_index, move_room_to_camera)
            .chain()
            .run_if(on_event::<ChangeRoom>()),
    );
    app.add_event::<ChangeRoom>();
}

#[derive(Resource)]
pub struct MapRoomIndex(pub RoomId);

#[derive(Event)]
pub struct ChangeRoom {
    pub new_room_id: RoomId,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct FocusedRoom;

fn change_room_index(mut events: EventReader<ChangeRoom>, mut room_index: ResMut<MapRoomIndex>) {
    for event in events.read() {
        println!("room changed to: {:?}", event.new_room_id);
        room_index.0 = event.new_room_id;
    }
}

fn move_room_to_camera(
    mut events: EventReader<ChangeRoom>,
    mut focused: Query<(Entity, &mut Transform, &RoomId), With<FocusedRoom>>,
    mut rooms: Query<(Entity, &mut Transform, &RoomId, &Room), Without<FocusedRoom>>,
    mut commands: Commands,
) {
    for event in events.read() {
        let Ok((old_entity, mut focused_transform, room_id)) = focused.get_single_mut() else {
            return;
        };
        println!(
            "moved room to: {:?}",
            Transform::from_translation(Vec3::splat((room_id.0 as f32 + 1.0 as f32) * 1000.0))
        );
        *focused_transform =
            Transform::from_translation(Vec3::splat((room_id.0 as f32 + 1.0 as f32) * 1000.0));
        commands.entity(old_entity).remove::<FocusedRoom>();

        for (new_entity, mut new_transform, room_id, room) in rooms.iter_mut() {
            if room_id == &event.new_room_id {
                println!(
                    "moved room to: {:?}",
                    Transform::from_translation(Vec3::splat(0.0))
                );
                println!("room_def_id: {:?}", room.room_def_id);
                *new_transform = Transform::from_translation(Vec3::splat(0.0));
                commands.entity(new_entity).insert(FocusedRoom);
            }
        }
    }
}
