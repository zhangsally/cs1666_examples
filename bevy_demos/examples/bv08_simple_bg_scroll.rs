use bevy::{prelude::*, window::PresentMode};
use std::convert::From;

const TITLE: &str = "bv08 Simple BG Scroll";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;

const PLAYER_SPEED: f32 = 500.;
const ACCEL_RATE: f32 = 5000.;

const TILE_SIZE: u32 = 100;

const SCROLL_SPEED: f32 = 120.;

enum PlayerType {
    Bird,
    Plane,
    UFO,
    Helicopter,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Background;

#[derive(Component)]
struct Velocity {
    velocity: Vec2,
}

impl Velocity {
    fn new() -> Self {          // Self is the type we're implementing for
        Self {
            velocity: Vec2::splat(0.),
        }
    }
}

impl From<Vec2> for Velocity {  // rewatch 9/23 lecture 8 minutes in
    fn from(velocity: Vec2) -> Self {   // takes argument of Vec2 (not reference)
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
        .add_systems(Update, scroll_bg)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2dBundle::default());

    for start_x in [0., WIN_W] {    // one spawned at x = 0. and one spawned at x = WIN_W
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("small_bg.png"),
                transform: Transform::from_xyz(start_x, 0., 0.),
                ..default()
            },
            Velocity::from(Vec2::new(SCROLL_SPEED, 0.)),    // speed of the scroll
            Background,
        ));
    }

    let bird_sheet_handle = asset_server.load("birds.png");
    let bird_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 2, 2, None, None); // this vector specifies the size of the sprites in the sprite sheet
    let bird_layout_handle = texture_atlases.add(bird_layout);
    commands.spawn((
        SpriteBundle {
            texture: bird_sheet_handle.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., 900.),
                ..default()
            },
            ..default()
        },
        TextureAtlas {
            layout: bird_layout_handle,
            index: PlayerType::Plane as usize,
        },
        Velocity::new(),
        Player,
    ));
}

fn scroll_bg(
    time: Res<Time>,
    mut bg: Query<(&mut Transform, &Velocity), (With<Background>, Without<Player>)>,
) {
    let deltat = time.delta_seconds();
    for (mut bt, bv) in bg.iter_mut() {
        bt.translation -= Vec3::from((bv.velocity, 0.)) * deltat;
        if bt.translation.x < -WIN_W {
            bt.translation += Vec3::new(WIN_W * 2., 0., 0.);
        }
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

    if input.pressed(KeyCode::KeyW) {
        deltav.y += 1.;
    }

    if input.pressed(KeyCode::KeyS) {
        deltav.y -= 1.;
    }

    let deltat = time.delta_seconds();
    let acc = ACCEL_RATE * deltat;

    pv.velocity = if deltav.length() > 0. { // if player input 
        (pv.velocity + (deltav.normalize_or_zero() * acc)).clamp_length_max(PLAYER_SPEED)   // normalizes the vector so max speed is 1 in any direction
    } else if pv.velocity.length() > acc {  // else if no player input but still have some speed
        pv.velocity + (pv.velocity.normalize_or_zero() * -acc)  // slow down instead of immediately stopping
    } else {                                // else speed is less than acceleration so just stop moving
        Vec2::splat(0.) // not moving anymore
    };
    let change = pv.velocity * deltat;

    let new_pos = pt.translation + Vec3::new(change.x, 0., 0.);
    if new_pos.x >= -(WIN_W / 2.) + (TILE_SIZE as f32) / 2.
        && new_pos.x <= WIN_W / 2. - (TILE_SIZE as f32) / 2.
    {
        pt.translation = new_pos;
    }

    let new_pos = pt.translation + Vec3::new(0., change.y, 0.);
    if new_pos.y >= -(WIN_H / 2.) + (TILE_SIZE as f32) / 2.
        && new_pos.y <= WIN_H / 2. - (TILE_SIZE as f32) / 2.
    {
        pt.translation = new_pos;
    }
}
