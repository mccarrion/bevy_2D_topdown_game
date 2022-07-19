mod game;

use std::fs;
use std::fs::File;
use std::io::Read;
use game::boundary::*;
use game::global::*;
use game::player::*;

use bevy::{
    core::FixedTimestep,
    prelude::*,
};
use serde::*;
use serde_json::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(move_player)
                .with_system(move_camera)
                .with_system(animate_player_sprite)
        )
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Player
    let texture_handle = asset_server
        .load("sprout_lands/characters/basic_character_spritesheet.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(50.0, 50.0),
        4,
        4);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn()
        .insert(Player)
        .insert_bundle(SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(STARTING_X, STARTING_Y, 0.0),
                scale: PLAYER_SIZE,
                ..default()
            },
            texture_atlas: texture_atlas_handle,
            ..default()
        })
        .insert(AnimationTimer::new(Timer::from_seconds(0.1, true)))
        .insert(Collider);

    // Spawn background
    #[derive(Component)]
    pub struct Background;

    let background_texture_handle = asset_server.load("tiled/output/water.png");
    commands
        .spawn()
        .insert(Background)
        .insert_bundle(SpriteBundle {
            texture: background_texture_handle,
            transform: Transform {
                scale: PLAYER_SIZE,
                ..default()
            },
            ..Default::default()
        });

    let ts = tileset_struct_from_json();

    // Boundaries
    commands.spawn_bundle(BoundaryBundle::new(BoundaryLocation::Left));
    commands.spawn_bundle(BoundaryBundle::new(BoundaryLocation::Right));
    commands.spawn_bundle(BoundaryBundle::new(BoundaryLocation::Lower));
    commands.spawn_bundle(BoundaryBundle::new(BoundaryLocation::Upper));
}

#[derive(Serialize, Deserialize)]
struct TileSet {
    columns: i8,
    image: String,
    imageheight: i8,
    imagewidth: i8,
    margin: i8,
    name: String,
    spacing: i8,
    tilecount: i8,
    tiledversion: String,
    tileheight: i8,
    tilewidth: i8,
    version: String
}

fn tileset_struct_from_json() -> TileSet {
    let tileset_data = fs::read_to_string("./assets/tiled/tilesets/fences.json");
    return from_str(&tileset_data.unwrap()).unwrap();
}