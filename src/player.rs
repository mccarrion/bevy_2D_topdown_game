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
    mut query: Query<&mut Transform, With<Player>>,
    collider_query: Query<(&Transform), (With<Collider>, Without<Player>)>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let mut player_transform = query.single_mut();

    // Update X position of the Player
    let mut direction_x = 0.0;
    if keyboard_input.pressed(KeyCode::Left) {
        direction_x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        direction_x += 1.0;
    }
    let new_player_position_x = player_transform.translation.x + direction_x * PLAYER_SPEED * TIME_STEP;
    let left_bound = LEFT_BOUND + WALL_THICKNESS / 2.0 + PLAYER_SIZE.x / 2.0 + PLAYER_PADDING;
    let right_bound = RIGHT_BOUND - WALL_THICKNESS / 2.0 - PLAYER_SIZE.x / 2.0 - PLAYER_PADDING;
    player_transform.translation.x = new_player_position_x.clamp(left_bound, right_bound);

    // Update Y position of the Player
    let mut direction_y = 0.0;
    if keyboard_input.pressed(KeyCode::Up) {
        direction_y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        direction_y -= 1.0;
    }
    let new_player_position_y = player_transform.translation.y + direction_y * PLAYER_SPEED * TIME_STEP;
    let lower_bound = LOWER_BOUND + WALL_THICKNESS / 2.0 + PLAYER_SIZE.y / 2.0 + PLAYER_PADDING;
    let upper_bound = UPPER_BOUND - WALL_THICKNESS / 2.0 - PLAYER_SIZE.y / 2.0 - PLAYER_PADDING;
    player_transform.translation.y = new_player_position_y.clamp(lower_bound, upper_bound);

    let player_size = player_transform.scale.truncate();

    for (transform) in collider_query.iter() {
        let collision = collide(
            player_transform.translation,
            player_size,
            transform.translation,
            transform.scale.truncate(),
        );
        if let Some(collision) = collision {
            collision_events.send_default();

            let mut stop_x = false;
            let mut stop_y = false;

            match collision {
                Collision::Left => stop_x = player_transform.translation.x < 0.0,
                Collision::Right => stop_x = player_transform.translation.x > 0.0,
                Collision::Top => stop_y = player_transform.translation.y > 0.0,
                Collision::Bottom => stop_y = player_transform.translation.y < 0.0,
                Collision::Inside => { /* do nothing */ }
            }

            if stop_x {
                player_transform.translation.x = -1.0;
            }

            if stop_y {
                player_transform.translation.y = -1.0;
            }
        }
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