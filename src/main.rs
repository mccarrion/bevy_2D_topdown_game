pub mod player;
pub mod global;
pub mod tileset;

use std::collections::HashMap;
use std::fs;

use bevy::{
    core::FixedTimestep,
    prelude::*,
};
use serde_json::from_str;
use crate::global::*;
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

    // TileSet
    let texture_atlas_map: HashMap<TileSetId, TextureAtlas> = map_texture_atlas_to_gid(&asset_server);

    let mut atlas_to_sprite_map: HashMap<usize, TileSprite> = HashMap::new();
    for (tilesetid, texture_atlas) in texture_atlas_map.clone() {
        let atlas_handle = texture_atlases.add(texture_atlas.clone());

        let str: String = String::from(&tilesetid.source);

        let metadata_json_dir: String = str.replace("../tilesets", "assets/tiled/metadata");
        let metadata_json: String = fs::read_to_string(metadata_json_dir).unwrap();
        let metadata_vec: Vec<TileMetadata> = from_str(&metadata_json).unwrap();

        for n in 0..texture_atlas.len() {
            let metadata: &TileMetadata = metadata_vec.get(n).clone().unwrap();
            let tile_sprite = TileSprite {
                atlas_handle: atlas_handle.clone(),
                atlas_sprite_id: n,
                left: metadata.left,
                right: metadata.right,
                top: metadata.top,
                bottom: metadata.bottom,
            };
            atlas_to_sprite_map.insert(tilesetid.firstgid as usize + n, tile_sprite);
        }
    }
    z_order = spawn_map(&mut commands, atlas_to_sprite_map, z_order);

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
                scale: Vec3::splat(PLAYER_SIZE),
                ..default()
            },
            texture_atlas: texture_atlas_handle,
            ..default()
        })
        .insert(AnimationTimer::new(Timer::from_seconds(0.1, true)));
}
