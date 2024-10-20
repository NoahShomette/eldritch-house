use bevy::prelude::*;

use crate::screens::Screen;

use super::RoomId;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(MapRoomIndex(RoomId(0)));
    app.add_systems(OnEnter(Screen::Gameplay), |mut commands: Commands| {
        commands.insert_resource(MapRoomIndex(RoomId(0)));
    });

    app.add_systems(Update, change_room_index.run_if(on_event::<ChangeRoom>()));
    app.add_event::<ChangeRoom>();
}

#[derive(Resource)]
pub struct MapRoomIndex(pub RoomId);

#[derive(Event)]
pub struct ChangeRoom {
    pub new_room_id: RoomId,
}

fn change_room_index(mut events: EventReader<ChangeRoom>, mut room_index: ResMut<MapRoomIndex>) {
    for event in events.read() {
        println!("room changed to: {:?}", event.new_room_id);
        room_index.0 = event.new_room_id;
    }
}

fn move_room_to_camera() {}
