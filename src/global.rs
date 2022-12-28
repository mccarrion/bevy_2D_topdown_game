use bevy::{
    prelude::*,
};

// Global constants
pub const TIME_STEP: f32 = 1.0 / 60.0;  // time between each physics calculation
pub const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

#[derive(Component)]
pub struct Collider;