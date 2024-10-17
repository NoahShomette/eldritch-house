//! A credits screen that can be accessed from the title screen.

use bevy::prelude::*;
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{
        config::{ConfigureLoadingState, LoadingStateConfig},
        LoadingStateAppExt,
    },
};

use crate::{audio::Music, screens::Screen, theme::prelude::*, AppLoadingState};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Credits), spawn_credits_screen);
    app.configure_loading_state(
        LoadingStateConfig::new(AppLoadingState::Loading).load_collection::<CreditsMusic>(),
    );
    app.add_systems(OnEnter(Screen::Credits), play_credits_music);
    app.add_systems(OnExit(Screen::Credits), stop_music);
}

fn spawn_credits_screen(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Credits))
        .with_children(|children| {
            children.header("Made by");
            children.label("Joe Shmoe - Implemented aligator wrestling AI");
            children.label("Jane Doe - Made the music for the alien invasion");

            children.header("Assets");
            children.label("Bevy logo - All rights reserved by the Bevy Foundation. Permission granted for splash screen use when unmodified.");
            children.label("Ducky sprite - CC0 by Caz Creates Games");
            children.label("Button SFX - CC0 by Jaszunio15");
            children.label("Music - CC BY 3.0 by Kevin MacLeod");

            children.button("Back").observe(enter_title_screen);
        });
}

fn enter_title_screen(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

#[derive(Resource, AssetCollection, Reflect, Clone)]
pub struct CreditsMusic {
    #[asset(path = "audio/music/Monkeys Spinning Monkeys.ogg")]
    music: Handle<AudioSource>,
    entity: Option<Entity>,
}

fn play_credits_music(mut commands: Commands, mut music: ResMut<CreditsMusic>) {
    music.entity = Some(
        commands
            .spawn((
                AudioBundle {
                    source: music.music.clone(),
                    settings: PlaybackSettings::LOOP,
                },
                Music,
            ))
            .id(),
    );
}

fn stop_music(mut commands: Commands, mut music: ResMut<CreditsMusic>) {
    if let Some(entity) = music.entity.take() {
        commands.entity(entity).despawn_recursive();
    }
}
