use bevy::{asset::LoadState, prelude::*};
use std::convert::From;

use crate::{
    level::Background,
    loading::{despawn_with, LoadingAssets},
    win::Win,
    GameState, ACCEL_RATE, ANIM_TIME, LEVEL_LEN, PLAYER_SPEED, TILE_SIZE, WIN_H, WIN_W,
};

#[derive(Component)]
pub struct Player;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component, Deref, DerefMut)]
pub struct AnimationFrameCount(usize);

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec2);

#[derive(Resource)]
pub struct PlayerSheet(Handle<Image>, Handle<TextureAtlasLayout>);

impl Velocity {
    fn new() -> Self {
        Self(Vec2::splat(0.))
    }
}

impl From<Vec2> for Velocity {
    fn from(velocity: Vec2) -> Self {
        Self(velocity)
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_player_sheet)
            .add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, move_player.run_if(in_state(GameState::Playing)))
            .add_systems(
                Update,
                (animate_player, move_camera)
                    .after(move_player)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), despawn_with::<Player>);
    }
}

fn load_player_sheet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut loading_assets: ResMut<LoadingAssets>,
) {
    let player_sheet_handle = asset_server.load("walking.png");
    loading_assets.push((player_sheet_handle.clone().untyped(), LoadState::NotLoaded));

    let player_layout =
        TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE as u32), 4, 1, None, None);
    let player_layout_handle = texture_atlases.add(player_layout);

    commands.insert_resource(PlayerSheet(player_sheet_handle, player_layout_handle));
}

fn spawn_player(
    mut commands: Commands,
    texture_atlases: Res<Assets<TextureAtlasLayout>>,
    player_sheet: Res<PlayerSheet>,
) {
    let player_layout = texture_atlases.get(&player_sheet.1);
    let player_layout_len = player_layout.unwrap().len();

    commands.spawn((
        SpriteBundle {
            texture: player_sheet.0.clone(),
            transform: Transform::from_xyz(0., -(WIN_H / 2.) + (TILE_SIZE * 1.5), 900.),
            ..default()
        },
        TextureAtlas {
            layout: player_sheet.1.clone(),
            index: 0,
        },
        AnimationTimer(Timer::from_seconds(ANIM_TIME, TimerMode::Repeating)),
        AnimationFrameCount(player_layout_len),
        Velocity::new(),
        Player,
    ));
}

fn move_player(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&mut Transform, &mut Velocity), (With<Player>, Without<Background>)>,
    mut win_event: EventWriter<Win>,
) {
    let (mut transform, mut velocity) = player.single_mut();

    let mut deltav = Vec2::splat(0.);

    if input.pressed(KeyCode::KeyA) {
        deltav.x -= 1.;
    }

    if input.pressed(KeyCode::KeyD) {
        deltav.x += 1.;
    }

    let deltat = time.delta_seconds();
    let acc = ACCEL_RATE * deltat;

    **velocity = if deltav.length() > 0. {
        (**velocity + (deltav.normalize_or_zero() * acc)).clamp_length_max(PLAYER_SPEED)
    } else if velocity.length() > acc {
        **velocity + (velocity.normalize_or_zero() * -acc)
    } else {
        Vec2::splat(0.)
    };
    let change = **velocity * deltat;

    let new_pos = transform.translation + Vec3::new(change.x, 0., 0.);
    if new_pos.x >= -(WIN_W / 2.) + TILE_SIZE / 2.
        && new_pos.x <= LEVEL_LEN - (WIN_W / 2. + TILE_SIZE / 2.)
    {
        transform.translation = new_pos;
    }

    let new_pos = transform.translation + Vec3::new(0., change.y, 0.);
    if new_pos.y >= -(WIN_H / 2.) + (TILE_SIZE * 1.5) && new_pos.y <= WIN_H / 2. - TILE_SIZE / 2. {
        transform.translation = new_pos;
    }

    if new_pos.x > LEVEL_LEN - (WIN_W / 2. + TILE_SIZE) {
        // Close enough to end of level, move to WinScreen
        win_event.send(Win);
    }
}

fn animate_player(
    time: Res<Time>,
    mut player: Query<
        (
            &Velocity,
            &mut TextureAtlas,
            &mut AnimationTimer,
            &AnimationFrameCount,
        ),
        With<Player>,
    >,
) {
    let (velocity, mut texture_atlas, mut timer, frame_count) = player.single_mut();
    if velocity.cmpne(Vec2::ZERO).any() {
        timer.tick(time.delta());

        if timer.just_finished() {
            texture_atlas.index = (texture_atlas.index + 1) % **frame_count;
        }
    }
}

fn move_camera(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    let pt = player.single();
    let mut ct = camera.single_mut();

    ct.translation.x = pt.translation.x.clamp(0., LEVEL_LEN - WIN_W);
}
