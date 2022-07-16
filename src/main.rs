mod game;

use game::boundary::*;
use game::global::*;
use game::player::*;

use bevy::{
    core::FixedTimestep,
    prelude::*,
};

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
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    // Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Player
    let texture_handle = asset_server
        .load( "sprout_lands/characters/basic_character_spritesheet.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(50.0, 50.0),
        4,
        4);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn()
        .insert(Player)
        .insert_bundle(SpriteSheetBundle{
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

    // Boundaries
    commands.spawn_bundle(BoundaryBundle::new(BoundaryLocation::Left));
    commands.spawn_bundle(BoundaryBundle::new(BoundaryLocation::Right));
    commands.spawn_bundle(BoundaryBundle::new(BoundaryLocation::Lower));
    commands.spawn_bundle(BoundaryBundle::new(BoundaryLocation::Upper));
}