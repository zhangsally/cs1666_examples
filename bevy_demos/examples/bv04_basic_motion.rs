use bevy::{prelude::*, window::PresentMode};

const TITLE: &str = "bv04 Basic Motion";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;
const PLAYER_SZ: f32 = 32.;

#[derive(Component)]
struct Player;

use bevy::color::palettes::css::SEA_GREEN;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::Srgba(Srgba::gray(0.25))))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: TITLE.into(),
                resolution: (WIN_W, WIN_H).into(),
                present_mode: PresentMode::Fifo,
                ..default()     // indicates using the implemented default trait
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::Srgba(SEA_GREEN),
                custom_size: Some(Vec2::splat(PLAYER_SZ)),
                ..default()
            },
            ..default()
        })
        .insert(Player);
}

fn move_player(input: Res<ButtonInput<KeyCode>>, mut player: Query<&mut Transform, With<Player>>) {
    // transform = position within game world; querying state of game world (attributes) of items with player attribute
    let mut player_transform = player.single_mut(); // we know there is only going to be one value, but if there were more, this would crash

    let mut x_vel = 0.;
    let mut y_vel = 0.;

    if input.pressed(KeyCode::KeyA) {   // .pressed() checks if it's currently being pressed
        x_vel -= 1.;
    }

    if input.pressed(KeyCode::KeyD) {
        x_vel += 1.;
    }

    if input.pressed(KeyCode::KeyW) {
        y_vel += 1.;
    }

    if input.pressed(KeyCode::KeyS) {
        y_vel -= 1.;
    }

    player_transform.translation.x += x_vel;
    player_transform.translation.y += y_vel;
}

/* TODO:
 * Can we slowly ramp up to speed limit instead of max accel?
 * What about different refresh rates?
 * How do we stay inside the window?
 * How do we avoid speeding up along the diagonal?
 */
