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
        )
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Player
    commands
        .spawn()
        .insert(Player)
        .insert_bundle(SpriteBundle{
            transform: Transform {
                translation: Vec3::new(STARTING_X, STARTING_Y, 0.0),
                scale: PLAYER_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: PLAYER_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Collider);

    // Boundaries
    commands.spawn_bundle(BoundaryBundle::new(BoundaryLocation::Left));
    commands.spawn_bundle(BoundaryBundle::new(BoundaryLocation::Right));
    commands.spawn_bundle(BoundaryBundle::new(BoundaryLocation::Lower));
    commands.spawn_bundle(BoundaryBundle::new(BoundaryLocation::Upper));
}