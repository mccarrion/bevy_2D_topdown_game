use bevy::{
    math::{const_vec3},
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use crate::global::TIME_STEP;
use crate::boundary::*;

// Player constants
pub const PLAYER_SIZE: Vec3 = const_vec3!([3.0, 3.0, 0.0]);
pub const PLAYER_SPEED: f32 = 700.0;
pub const PLAYER_PADDING: f32 = 10.0;
pub const STARTING_Y: f32 = 0.0;
pub const STARTING_X: f32 = 0.0;

#[derive(Component)]
pub struct Player;

#[derive(Default)]
pub struct CollisionEvent;

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    collider_query: Query<(&Transform), (With<Collider>, Without<Player>, Without<Camera>)>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let mut player_transform = player_query.single_mut();
    let mut camera_transform = camera_query.single_mut();

    // Generate new X position of the Player based on KeyCode input
    let mut direction_x = 0.0;
    if keyboard_input.pressed(KeyCode::Left) {
        direction_x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        direction_x += 1.0;
    }
    let new_player_position_x = player_transform.translation.x + direction_x * PLAYER_SPEED * TIME_STEP;
    let future_x_position: Vec3 = const_vec3!([new_player_position_x, 0.0, 0.0]);

    // Generate new Y position of the Player based on KeyCode input
    let mut direction_y = 0.0;
    if keyboard_input.pressed(KeyCode::Up) {
        direction_y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        direction_y -= 1.0;
    }
    let new_player_position_y = player_transform.translation.y + direction_y * PLAYER_SPEED * TIME_STEP;
    let future_y_position: Vec3 = const_vec3!([0.0, new_player_position_y, 0.0]);

    let player_size = player_transform.scale.truncate();

    let mut x_collided = false;
    let mut y_collided = false;

    for (transform) in collider_query.iter() {

        let collision_x = collide(
            future_x_position,
            player_size,
            transform.translation,
            transform.scale.truncate(),
        );
        if let Some(_collision_x) = collision_x {
            collision_events.send_default();
            x_collided = true;
        }

        let collision_y = collide(
            future_y_position,
            player_size,
            transform.translation,
            transform.scale.truncate(),
        );
        if let Some(_collision_y) = collision_y {
            collision_events.send_default();
            y_collided = true;
        }
    }

    if !x_collided {
        player_transform.translation.x = new_player_position_x;
        camera_transform.translation.x = new_player_position_x;
    }

    if !y_collided {
        player_transform.translation.y = new_player_position_y;
        camera_transform.translation.y = new_player_position_y;
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

impl AnimationTimer {
    pub fn new(timer: Timer) -> AnimationTimer {
        AnimationTimer(timer)
    }
}

/*
 * TODO: animation starts at first index if you press two KeyCodes at once need to refactor that out
 * TODO: update method to remove hard-coding and implement Event detection
 */
pub fn animate_player_sprite(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (mut timer, mut sprite) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            let num_cols = 4;
            if keyboard_input.pressed(KeyCode::Down) {
                sprite.index = (sprite.index + 1) % (num_cols) + (num_cols * 0);
            }
            if keyboard_input.pressed(KeyCode::Up) {
                sprite.index = (sprite.index + 1) % (num_cols) + (num_cols * 1);
            }
            if keyboard_input.pressed(KeyCode::Left) {
                sprite.index = (sprite.index + 1) % (num_cols) + (num_cols * 2);
            }
            if keyboard_input.pressed(KeyCode::Right) {
                sprite.index = (sprite.index + 1) % (num_cols) + (num_cols * 3);
            }

        }
    }
}