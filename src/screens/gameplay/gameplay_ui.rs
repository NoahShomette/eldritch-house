use crate::map::{ChangeRoom, MapRoomIndex, Room, RoomConnectionDirection, RoomId};
use crate::screens::Screen;
use crate::theme::prelude::OnPress;
use crate::theme::widgets::{Containers, Widgets};
use bevy::prelude::Val::Px;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), setup_gameplay_ui);
    app.add_systems(
        Update,
        enable_disable_move_room_buttons.run_if(resource_changed::<MapRoomIndex>),
    );
}

fn setup_gameplay_ui(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Gameplay))
        .with_children(|children| {
            children
                .spawn(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Px(25.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|children| {
                    children
                        .button("Move North")
                        .insert(MoveRoomButton(RoomConnectionDirection::North))
                        .observe(move_room_button);
                });

            children
                .spawn(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        left: Px(25.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|children| {
                    children
                        .button("Move West")
                        .insert(MoveRoomButton(RoomConnectionDirection::West))
                        .observe(move_room_button);
                });

            children
                .spawn(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        bottom: Px(25.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|children| {
                    children
                        .button("Move South")
                        .insert(MoveRoomButton(RoomConnectionDirection::South))
                        .observe(move_room_button);
                });
            children
                .spawn(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        right: Px(25.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|children| {
                    children
                        .button("Move East")
                        .insert(MoveRoomButton(RoomConnectionDirection::East))
                        .observe(move_room_button);
                });
        });
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct MoveRoomButton(RoomConnectionDirection);

fn move_room_button(
    trigger: Trigger<OnPress>,
    mut event_writer: EventWriter<ChangeRoom>,
    room_res: Res<MapRoomIndex>,
    buttons: Query<&MoveRoomButton>,
    rooms: Query<(&RoomId, &Room)>,
) {
    let Ok(move_room_button) = buttons.get(trigger.entity()) else {
        return;
    };

    let Some((_room_id, room)) = rooms.iter().find(|(id, _)| **id == room_res.0) else {
        return;
    };
    let Some(target_room_id) = room.connections.get(&move_room_button.0) else {
        return;
    };
    event_writer.send(ChangeRoom {
        new_room_id: target_room_id.clone(),
    });
}

fn enable_disable_move_room_buttons(
    room_res: Res<MapRoomIndex>,
    mut buttons: Query<(&MoveRoomButton, &mut Style)>,
    rooms: Query<(&RoomId, &Room)>,
) {
    let Some((_room_id, room)) = rooms.iter().find(|(id, _)| **id == room_res.0) else {
        return;
    };
    for (move_room_button, mut style) in buttons.iter_mut() {
        if !room.connections.contains_key(&move_room_button.0) {
            style.display = Display::None;
        } else {
            style.display = Display::Flex;
        }
    }
}
