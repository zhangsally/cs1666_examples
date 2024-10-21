use bevy::{asset::LoadState, prelude::*};

use crate::{
    loading::{despawn_with, LoadingAssets},
    GameState,
};

#[derive(Resource, Deref, DerefMut)]
pub struct BackgroundMusic(Handle<AudioSource>);

#[derive(Component)]
pub struct Music;

pub struct BackgroundMusicPlugin;
impl Plugin for BackgroundMusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_background_music)
            .add_systems(OnEnter(GameState::Playing), start_background_music)
            .add_systems(OnExit(GameState::Playing), despawn_with::<Music>);
    }
}

fn load_background_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading_assets: ResMut<LoadingAssets>,
) {
    let bg_music_handle = asset_server.load("bg_music.ogg");
    loading_assets.push((bg_music_handle.clone().untyped(), LoadState::NotLoaded));
    commands.insert_resource(BackgroundMusic(bg_music_handle));
}

fn start_background_music(mut commands: Commands, background_music: Res<BackgroundMusic>) {
    commands.spawn((
        AudioBundle {
            source: background_music.clone(),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Loop,
                ..default()
            },
        },
        Music,
    ));
}
