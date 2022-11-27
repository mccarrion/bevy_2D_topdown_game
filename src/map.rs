use std::error::Error;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;

use bevy::{
    core::FixedTimestep,
    prelude::*,
};
use crate::{Collider, PLAYER_SIZE};

#[derive(Component)]
pub struct MapLayer;

#[derive(Bundle)]
pub struct MapBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

impl MapBundle {
    pub fn new(
        map_dir: DirEntry,
        z_order: f32,
        asset_server: &Res<AssetServer>
    ) -> MapBundle {
        let path = map_dir.path()
            .display()
            .to_string()
            .replace("./assets/", "");
        let layer_path = Path::new(&path);
        let background_texture_handle = asset_server.load(layer_path);
        MapBundle {
            sprite_bundle: SpriteBundle {
                texture: background_texture_handle,
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, z_order),
                    scale: PLAYER_SIZE,
                    ..default()
                },
                ..Default::default()
            },
            collider: Collider,
        }
    }
}