use bevy::{
    math::{const_vec3},
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use crate::global::TIME_STEP;
use crate::boundary::*;
use crate::tileset::{TileMetadata, TileSprite};

// Player constants
pub const PLAYER_SIZE: f32 = 3.0;
pub const PLAYER_SPEED: f32 = 700.0;
pub const PLAYER_PADDING: f32 = 10.0;
pub const STARTING_Y: f32 = 2500.0;
pub const STARTING_X: f32 = 3000.0;

#[derive(Component)]
pub struct Player;

#[derive(Default)]
pub struct CollisionEvent;

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    collider_query: Query<(&Transform, &TileSprite), (With<Collider>, Without<Player>, Without<Camera>)>,
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
    let future_x_position: Vec3 = const_vec3!([new_player_position_x, player_transform.translation.y, 0.0]);

    // Generate new Y position of the Player based on KeyCode input
    let mut direction_y = 0.0;
    if keyboard_input.pressed(KeyCode::Up) {
        direction_y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        direction_y -= 1.0;
    }
    let new_player_position_y = player_transform.translation.y + direction_y * PLAYER_SPEED * TIME_STEP;
    let future_y_position: Vec3 = const_vec3!([player_transform.translation.x, new_player_position_y, 0.0]);

    let player_size = player_transform.scale.truncate();

    let mut x_collided = false;
    let mut y_collided = false;

    for (transform, tilesprite) in collider_query.iter() {
        let collision_x = collide(
            future_x_position,
            Vec2::splat(PLAYER_SIZE * 10.0),
            transform.translation,
            Vec2::splat(PLAYER_SIZE * 25.0),
        );
        if let Some(collision_x) = collision_x {
            match collision_x {
                Collision::Left => {
                    x_collided = tilesprite.left;
                }
                Collision::Right => {
                    x_collided = tilesprite.right;
                }
                Collision::Top => {/* do nothing */}
                Collision::Bottom => {/* do nothing */}
                Collision::Inside => {/* do nothing */}
            }
        }

        let collision_y = collide(
            future_y_position,
            Vec2::splat(PLAYER_SIZE * 15.0),
            transform.translation,
            Vec2::splat(PLAYER_SIZE * 15.0),
        );

        if let Some(collision_y) = collision_y {
            match collision_y {
                Collision::Left => {/* do nothing */}
                Collision::Right => {/* do nothing */}
                Collision::Top => {
                    y_collided = tilesprite.top;
                }
                Collision::Bottom => {
                    y_collided = tilesprite.bottom;
                }
                Collision::Inside => {/* do nothing */}
            }
        }

        if x_collided || y_collided {
            collision_events.send_default();
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