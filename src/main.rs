pub mod player;
pub mod boundary;
pub mod global;
pub mod map;
pub mod tileset;

use std::fs;
use std::path::Path;

use bevy::{
    core::FixedTimestep,
    prelude::*,
};
use crate::boundary::*;
use crate::global::*;
use crate::map::*;
use crate::player::*;
use crate::tileset::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_event::<CollisionEvent>()
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(move_player)
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

    // Control render ordering
    let mut z_order: f32 = 0.0;

    // Map
    // let map_paths = fs::read_dir("./assets/output/map/").unwrap();
    // for map_path in map_paths {
    //     commands.spawn_bundle(MapBundle::new(
    //         map_path.unwrap(),
    //         z_order,
    //         &asset_server));
    //     z_order += 0.1;
    // }

    // TileSet
    generate_map_from_tiled_config(&asset_server, &texture_atlases);

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
                translation: Vec3::new(STARTING_X, STARTING_Y, z_order),
                scale: PLAYER_SIZE,
                ..default()
            },
            texture_atlas: texture_atlas_handle,
            ..default()
        })
        .insert(AnimationTimer::new(Timer::from_seconds(0.1, true)));
    z_order += 0.1;
}
