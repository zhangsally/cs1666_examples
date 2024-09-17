use bevy::{prelude::*, window::PresentMode};
use rand::Rng;

const TITLE: &str = "bv06 Tiling";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;

const TILE_SIZE: u32 = 100;
const NUM_BIRDS: usize = 8;

#[derive(Component)]
struct Bird;

#[derive(Component)]
struct Brick;

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
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let bird_sheet_handle = asset_server.load("birds.png");
    let bird_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 2, 2, None, None);
    let bird_layout_len = bird_layout.textures.len();
    let bird_layout_handle = texture_atlases.add(bird_layout);

    let brick_sheet_handle = asset_server.load("bricks.png");
    let brick_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 4, 1, None, None);
    let brick_layout_len = brick_layout.textures.len();
    let brick_layout_handle = texture_atlases.add(brick_layout);

    commands.spawn(Camera2dBundle::default());

    let mut rng = rand::thread_rng();
    let x_bound = WIN_W / 2. - (TILE_SIZE as f32) / 2.;
    let y_bound = WIN_H / 2. - (TILE_SIZE as f32) / 2.;

    for i in 0..NUM_BIRDS {
        let t = Vec3::new(
            rng.gen_range(-x_bound..x_bound),
            rng.gen_range(-y_bound..y_bound),
            900.,
        );
        commands.spawn((
            SpriteBundle {
                texture: bird_sheet_handle.clone(),
                transform: Transform {
                    translation: t,
                    ..default()
                },
                ..default()
            },
            TextureAtlas {
                index: i % bird_layout_len,
                layout: bird_layout_handle.clone(),
            },
            Bird,
        ));
    }

    let mut i = 0;
    let mut t = Vec3::new(-x_bound, -y_bound, 0.);
    while (i as f32) * (TILE_SIZE as f32) < WIN_W {
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
                index: i % brick_layout_len,
                layout: brick_layout_handle.clone(),
            },
            Brick,
        ));

        i += 1;
        t += Vec3::new(TILE_SIZE as f32, 0., 0.);
    }
}
