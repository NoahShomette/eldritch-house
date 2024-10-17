//! A loading screen during which game assets are loaded.
//! This reduces stuttering, especially for audio on WASM.

use bevy::prelude::*;

use crate::{theme::prelude::*, AppLoadingState};

use super::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppLoadingState::Loading), spawn_loading_screen);
    app.add_systems(
        OnEnter(AppLoadingState::Loaded),
        |mut next_state: ResMut<NextState<Screen>>| next_state.set(Screen::Splash),
    );
}

fn spawn_loading_screen(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(AppLoadingState::Loading))
        .with_children(|children| {
            children.label("Loading...").insert(Style {
                justify_content: JustifyContent::Center,
                ..default()
            });
        });
}
