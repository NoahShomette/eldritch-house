//! The screen state for the main gameplay.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{
        config::{ConfigureLoadingState, LoadingStateConfig},
        LoadingStateAppExt,
    },
};

use crate::{audio::Music, screens::Screen, AppLoadingState};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_level);
    app.configure_loading_state(
        LoadingStateConfig::new(AppLoadingState::Loading).load_collection::<GameplayMusic>(),
    );
    app.add_systems(OnEnter(Screen::Gameplay), play_gameplay_music);
    app.add_systems(OnExit(Screen::Gameplay), stop_music);

    app.add_systems(
        Update,
        return_to_title_screen
            .run_if(in_state(Screen::Gameplay).and_then(input_just_pressed(KeyCode::Escape))),
    );
}

fn spawn_level(mut commands: Commands) {}

#[derive(Resource, AssetCollection, Reflect, Clone)]
pub struct GameplayMusic {
    #[asset(path = "audio/music/Fluffing A Duck.ogg")]
    handle: Handle<AudioSource>,
    entity: Option<Entity>,
}

fn play_gameplay_music(mut commands: Commands, mut music: ResMut<GameplayMusic>) {
    music.entity = Some(
        commands
            .spawn((
                AudioBundle {
                    source: music.handle.clone(),
                    settings: PlaybackSettings::LOOP,
                },
                Music,
            ))
            .id(),
    );
}

fn stop_music(mut commands: Commands, mut music: ResMut<GameplayMusic>) {
    if let Some(entity) = music.entity.take() {
        commands.entity(entity).despawn_recursive();
    }
}

fn return_to_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
