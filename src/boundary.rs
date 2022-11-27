use bevy::prelude::*;

// Stage boundary constants
pub const LEFT_BOUND: f32 = -4000.;      // X boundary
pub const RIGHT_BOUND: f32 = 4000.;      // X boundary
pub const LOWER_BOUND: f32 = -4000.;     // Y boundary
pub const UPPER_BOUND: f32 = 4000.;      // Y boundary
pub const WALL_THICKNESS: f32 = 10.0;   // boundary thickness
pub const WALL_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);

#[derive(Component)]
pub struct Collider;

#[derive(Bundle)]
pub struct BoundaryBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

impl BoundaryBundle {
    pub fn new(location: BoundaryLocation) -> BoundaryBundle {
        BoundaryBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position().extend(0.0),
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}

pub enum BoundaryLocation {
    Left,
    Right,
    Lower,
    Upper
}

impl BoundaryLocation {
    fn position(&self) -> Vec2 {
        match self {
            BoundaryLocation::Left => Vec2::new(LEFT_BOUND, 0.),
            BoundaryLocation::Right => Vec2::new(RIGHT_BOUND, 0.),
            BoundaryLocation::Lower => Vec2::new(0., LOWER_BOUND),
            BoundaryLocation::Upper => Vec2::new(0., UPPER_BOUND),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = UPPER_BOUND - LOWER_BOUND;
        let arena_width = RIGHT_BOUND - LEFT_BOUND;

        // Validation check of constants
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            BoundaryLocation::Left | BoundaryLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            BoundaryLocation::Lower | BoundaryLocation::Upper => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}