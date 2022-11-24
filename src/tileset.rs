use std::collections::HashMap;
use std::fs;
use bevy::{
    core::FixedTimestep,
    prelude::*,
};
use image::{DynamicImage, GenericImage, ImageBuffer};
use serde::*;
use serde_json::*;

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

pub fn map_tile_to_id(
    texture_atlas_map: HashMap<i16, TextureAtlas>,
    atlas_handles_map: HashMap<i16, Handle<TextureAtlas>>
) {

    for (gid, atlas_handle) in atlas_handles_map {
        let texture_atlas: TextureAtlas = texture_atlas_map.get(&gid).unwrap().clone();
    }

    // let sprite = SpriteSheetBundle {
    //     texture_atlas: texture_handle,
    //     sprite: TextureAtlasSprite::new(0),
    //     ..default()
    // };

    // This loop maps all tiles to their gid assigned by Tiled
    // for n in 1..(tile_quantity + 1) {
        // let x_offset: u32 = (tilewidth * col) as u32;
        // let y_offset: u32 = (tileheight * row) as u32;
        // let tile_img: DynamicImage = img.crop(
        //     x_offset,
        //     y_offset,
        //     tileheight as u32,
        //     tileheight as u32);
        // col += 1;
        // if n % columns == 0 {
        //     col = 0;
        //     row += 1;
        // }
        // tile_map.insert(tile_id, tile_img);
        // tile_id += 1;
    // }
}

pub fn draw_tile_layers(
    tile_map: HashMap<i16, DynamicImage>,
    layers: Vec<TileLayer>,
    tilewidth: i16,
    tileheight: i16,
) {
    // let layers: Vec<TileLayer> = tilemap.layers;
    // let tilewidth: i16 = tilemap.tilewidth;
    // let tileheight: i16 = tilemap.tileheight;
    for layer in layers {
        let data: Vec<i16> = layer.data;
        let columns: i16 = layer.width;
        let rows: i16 = layer.height;

        // width and height of png file to draw
        let imgx = (tilewidth * columns) as u32;
        let imgy = (tileheight * rows) as u32;
        let mut img = ImageBuffer::new(imgx, imgy);
        let mut count: i16 = 1;
        let mut col: i16 = 1;
        let mut row: i16 = 1;
        let filename: String = String::from(layer.name) + ".png";

        // This draws the map based on the tile gid and tile location defined by both the data
        // vec and map width and height from the tmj file
        for n in data {
            if n != 0 {
                let tile = tile_map.get(&n).unwrap();
                img.copy_from(tile,
                              ((col - 1) * tilewidth) as u32,
                              ((row - 1) * tileheight) as u32)
                    .expect("Error copying tile to DynamicImage");
            }
            col += 1;
            if count % columns == 0 {
                col = 1;
                row += 1;
            }
            count += 1;
        }
        let mut filepath: String = "../assets/output/map".to_owned();
        fs::create_dir_all(&filepath)
            .expect("Error creating directory for saving map layers");
        filepath.push_str("/");
        filepath.push_str(&filename);
        img.save(filepath)
            .expect("Error saving image");
    }
}