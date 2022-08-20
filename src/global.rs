use bevy::{
    math::{const_vec3},
    prelude::*,
};
use crate::boundary::*;

// Global constants
pub const TIME_STEP: f32 = 1.0 / 60.0;  // time between each physics calculation
pub const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

// TODO: copied player constants. Should probably change later
pub const CAMERA_SIZE: Vec3 = const_vec3!([60.0, 60.0, 0.0]);
pub const CAMERA_SPEED: f32 = 700.0;
pub const CAMERA_PADDING: f32 = 10.0;

pub fn move_camera(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>
) {
    let mut camera_transform = query.single_mut();

    // Update X position of the Camera
    let mut direction_x = 0.0;
    if keyboard_input.pressed(KeyCode::Left) {
        direction_x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        direction_x += 1.0;
    }
    let new_camera_position_x = camera_transform.translation.x + direction_x * CAMERA_SPEED * TIME_STEP;
    let left_bound = LEFT_BOUND + WALL_THICKNESS / 2.0 + CAMERA_SIZE.x / 2.0 + CAMERA_PADDING;
    let right_bound = RIGHT_BOUND - WALL_THICKNESS / 2.0 - CAMERA_SIZE.x / 2.0 - CAMERA_PADDING;
    camera_transform.translation.x = new_camera_position_x.clamp(left_bound, right_bound);

    // Update Y position of the Camera
    let mut direction_y = 0.0;
    if keyboard_input.pressed(KeyCode::Up) {
        direction_y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        direction_y -= 1.0;
    }
    let new_camera_position_y = camera_transform.translation.y + direction_y * CAMERA_SPEED * TIME_STEP;
    let lower_bound = LOWER_BOUND + WALL_THICKNESS / 2.0 + CAMERA_SIZE.y / 2.0 + CAMERA_PADDING;
    let upper_bound = UPPER_BOUND - WALL_THICKNESS / 2.0 - CAMERA_SIZE.y / 2.0 - CAMERA_PADDING;
    camera_transform.translation.y = new_camera_position_y.clamp(lower_bound, upper_bound);

}