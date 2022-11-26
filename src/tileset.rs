use std::borrow::Borrow;
use std::collections::HashMap;
use std::fs;
use bevy::{
    core::FixedTimestep,
    prelude::*,
};
use serde::*;
use serde_json::*;
use crate::boundary::Collider;
use crate::player::PLAYER_SIZE;

#[derive(Serialize, Deserialize)]
pub struct TileSet {
    columns: i16,
    image: String,
    imageheight: i16,
    imagewidth: i16,
    margin: i16,
    name: String,
    spacing: i16,
    tilecount: i16,
    tiledversion: String,
    tileheight: i16,
    tilewidth: i16,
    #[serde(alias = "type")]
    ts_type: String,
    version: String,
}

#[derive(Serialize, Deserialize)]
pub struct TileSetId {
    firstgid: i16,
    source: String,
}

#[derive(Serialize, Deserialize)]
pub struct TileMap {
    compressionlevel: i16,
    height: i16,
    infinite: bool,
    layers: Vec<TileLayer>,
    nextlayerid: i16,
    nextobjectid: i16,
    orientation: String,
    renderorder: String,
    tiledversion: String,
    tileheight: i16,
    tilesets: Vec<TileSetId>,
    tilewidth: i16,
    #[serde(alias = "type")]
    tm_type: String,
    version: String,
    width: i16,
}

#[derive(Serialize, Deserialize)]
pub struct TileLayer {
    data: Vec<i16>,
    height: i16,
    id: i16,
    name: String,
    opacity: i16,
    #[serde(alias = "type")]
    tl_type: String,
    visible: bool,
    width: i16,
    x: i16,
    y: i16,
}

#[derive(Clone)]
pub struct TileSprite {
    pub atlas_handle: Handle<TextureAtlas>,
    pub atlas_sprite: TextureAtlasSprite
}

pub fn map_texture_atlas_to_gid(
    asset_server: &Res<AssetServer>,
) -> HashMap<i16, TextureAtlas> {
    // Generate TileMap struct from JSON file, the ".tmj" file
    let tilemap_data = fs::read_to_string("assets/tiled/maps/sprout_land.tmj");
    let tilemap: TileMap = from_str(&tilemap_data.unwrap()).unwrap();

    // Extract data necessary to generate map layers from TileMap struct
    let tilesetids: Vec<TileSetId> = tilemap.tilesets;

    let mut tile_map: HashMap<i16, TextureAtlas> = HashMap::new();
    for tilesetid in tilesetids {
        let str: String = String::from(&tilesetid.source);
        let tileset_json_dir: String = str.replace("..", "assets/tiled");
        let tileset_data: String = fs::read_to_string(tileset_json_dir).unwrap();
        let tileset: TileSet = from_str(&tileset_data).unwrap();
        let dir: String = String::from(&tileset.image);
        let tileset_img_dir: String = dir.replace("../../", "");
        let tile_quantity: usize = tileset.tilecount as usize;
        let mut tile_id = tilesetid.firstgid;
        let columns: usize = tileset.columns as usize;
        let rows: usize = tile_quantity / columns;

        let texture_handle: Handle<Image> = asset_server
            .load(&tileset_img_dir);
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(
                tileset.tileheight as f32,
                tileset.tilewidth as f32),
            columns,
            rows);
        tile_map.insert(tile_id, texture_atlas);
    }
    return tile_map;
}

pub fn spawn_map(
    mut commands: &mut Commands,
    mut atlas_to_sprite_map: HashMap<usize, TileSprite>,
) {
    // Generate TileMap struct from JSON file, the ".tmj" file
    let tilemap_data = fs::read_to_string("assets/tiled/maps/sprout_land.tmj");
    let tilemap: TileMap = from_str(&tilemap_data.unwrap()).unwrap();
    let layers: Vec<TileLayer> = tilemap.layers;
    let tilewidth: i16 = tilemap.tilewidth;
    let tileheight: i16 = tilemap.tileheight;
    for layer in layers {
        let data: Vec<i16> = layer.data;
        let columns: i16 = layer.width;
        let rows: i16 = layer.height;

        // width and height of png file to draw
        let mut count: i16 = 1;
        let mut col: i16 = 1;
        let mut row: i16 = 1;

        // This draws the map based on the tile gid and tile location defined by both the data
        // vec and map width and height from the tmj file
        for n in data {
            if n != 0 {
                let tile_sprite: TileSprite = atlas_to_sprite_map.remove(&(n as usize)).unwrap();
                atlas_to_sprite_map.insert(n as usize, tile_sprite.clone());
                commands.spawn().insert_bundle(SpriteSheetBundle {
                    transform: Transform {
                        translation: Vec3::new(((col - 1) * tilewidth * PLAYER_SIZE.x as i16) as f32,
                                               ((row - 1) * tileheight * PLAYER_SIZE.y as i16) as f32,
                                               0.0),
                        scale: PLAYER_SIZE,
                        ..default()
                    },
                    texture_atlas: tile_sprite.atlas_handle,
                    sprite: tile_sprite.atlas_sprite,
                    ..default()
                }).insert(Collider);
            }
            col += 1;
            if count % columns == 0 {
                col = 1;
                row += 1;
            }
            count += 1;
        }
    }
}