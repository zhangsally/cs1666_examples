use bevy::{prelude::*, window::PresentMode};
use std::convert::From;

const TITLE: &str = "bv11 Animation";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;

const PLAYER_SPEED: f32 = 500.;
const ACCEL_RATE: f32 = 5000.;

const TILE_SIZE: u32 = 100;

const LEVEL_LEN: f32 = 5000.;

const ANIM_TIME: f32 = 0.2;

#[derive(Component)]
struct Player;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component, Deref, DerefMut)]
struct AnimationFrameCount(usize);

#[derive(Component)]
struct Brick;

#[derive(Component)]
struct Background;

#[derive(Component)]
struct Velocity {
    velocity: Vec2,
}

impl Velocity {
    fn new() -> Self {
        Self {
            velocity: Vec2::splat(0.),
        }
    }
}

impl From<Vec2> for Velocity {
    fn from(velocity: Vec2) -> Self {
        Self { velocity }
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::Srgba(Srgba::gray(0.25))))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: TITLE.into(),
                resolution: (WIN_W, WIN_H).into(),
                present_mode: PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .add_systems(Update, animate_player.after(move_player))
        .add_systems(Update, move_camera.after(move_player))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2dBundle::default());

    let bg_texture_handle = asset_server.load("small_bg.png");

    let mut x_offset = 0.;
    while x_offset < LEVEL_LEN {
        commands
            .spawn(SpriteBundle {
                texture: bg_texture_handle.clone(),
                transform: Transform::from_xyz(x_offset, 0., 0.),
                ..default()
            })
            .insert(Background);

        x_offset += WIN_W;
    }

    let player_sheet_handle = asset_server.load("walking.png");
    let player_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 4, 1, None, None);
    let player_layout_len = player_layout.textures.len();
    let player_layout_handle = texture_atlases.add(player_layout);
    commands.spawn((
        SpriteBundle {
            texture: player_sheet_handle,
            transform: Transform::from_xyz(0., -(WIN_H / 2.) + ((TILE_SIZE as f32) * 1.5), 900.),
            ..default()
        },
        TextureAtlas {
            layout: player_layout_handle,
            index: 0,
        },
        AnimationTimer(Timer::from_seconds(ANIM_TIME, TimerMode::Repeating)),
        AnimationFrameCount(player_layout_len),
        Velocity::new(),
        Player,
    ));

    let brick_sheet_handle = asset_server.load("bricks.png");
    let brick_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 4, 1, None, None);
    let brick_layout_len = brick_layout.len();
    let brick_layout_handle = texture_atlases.add(brick_layout);

    let mut i = 0;
    let mut t = Vec3::new(
        -WIN_W / 2. + (TILE_SIZE as f32) / 2.,
        -WIN_H / 2. + (TILE_SIZE as f32) / 2.,
        1.,
    );
    while i * TILE_SIZE < (LEVEL_LEN as u32) {
        commands.spawn((
            SpriteBundle {
                texture: brick_sheet_handle.clone(),
                transform: Transform {
                    translation: t,
                    ..default()
                },
                ..default()
            },
            TextureAtlas {
                layout: brick_layout_handle.clone(),
                index: (i as usize) % brick_layout_len,
            },
            Brick,
        ));

        i += 1;
        t += Vec3::new(TILE_SIZE as f32, 0., 0.);
    }
}

fn move_player(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&mut Transform, &mut Velocity), (With<Player>, Without<Background>)>,
) {
    let (mut pt, mut pv) = player.single_mut();

    let mut deltav = Vec2::splat(0.);

    if input.pressed(KeyCode::KeyA) {
        deltav.x -= 1.;
    }

    if input.pressed(KeyCode::KeyD) {
        deltav.x += 1.;
    }

    let deltat = time.delta_seconds();
    let acc = ACCEL_RATE * deltat;

    pv.velocity = if deltav.length() > 0. {
        (pv.velocity + (deltav.normalize_or_zero() * acc)).clamp_length_max(PLAYER_SPEED)
    } else if pv.velocity.length() > acc {
        pv.velocity + (pv.velocity.normalize_or_zero() * -acc)
    } else {
        Vec2::splat(0.)
    };
    let change = pv.velocity * deltat;

    let new_pos = pt.translation + Vec3::new(change.x, 0., 0.);
    if new_pos.x >= -(WIN_W / 2.) + (TILE_SIZE as f32) / 2.
        && new_pos.x <= LEVEL_LEN - (WIN_W / 2. + (TILE_SIZE as f32) / 2.)
    {
        pt.translation = new_pos;
    }

    let new_pos = pt.translation + Vec3::new(0., change.y, 0.);
    if new_pos.y >= -(WIN_H / 2.) + ((TILE_SIZE as f32) * 1.5)
        && new_pos.y <= WIN_H / 2. - (TILE_SIZE as f32) / 2.
    {
        pt.translation = new_pos;
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
    let (v, mut texture_atlas, mut timer, frame_count) = player.single_mut();
    //let temp: () = frame_count; // will tell you what the type is somehow?
    if v.velocity.cmpne(Vec2::ZERO).any() {
        timer.tick(time.delta());

        if timer.just_finished() {
            texture_atlas.index = (texture_atlas.index + 1) % **frame_count;    //** is a double dereference
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
