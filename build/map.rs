use std::borrow::Borrow;
use std::collections::HashMap;
use std::fs;
use std::fs::DirEntry;
use image::{DynamicImage, GenericImage, ImageBuffer, RgbImage};
use image::imageops::tile;
use image::math::Rect;
use serde::*;
use serde_json::*;

#[derive(Serialize, Deserialize)]
struct TileSet {
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
struct TileSetId {
    firstgid: i16,
    source: String,
}

#[derive(Serialize, Deserialize)]
struct TileMap {
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
struct TileLayer {
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

pub fn generate_map_from_tiled_config() {
    let mut tilemap_data = fs::read_to_string("../assets/tiled/maps/sprout_land.tmj");
    let mut tilemap: TileMap = from_str(&tilemap_data.unwrap()).unwrap();
    let tileset_ids: Vec<TileSetId> = tilemap.tilesets;

    let mut tilewidth: i16 = 0;
    let mut tileheight: i16 = 0;

    let mut tile_map: HashMap<i16, DynamicImage> = HashMap::new();
    for tileset_id in tileset_ids {
        let str: String = String::from(&tileset_id.source);
        let tileset_json_dir: String = str.replace("..", "../assets/tiled");
        let tileset_data: String = fs::read_to_string(tileset_json_dir).unwrap();
        let tileset: TileSet = from_str(&tileset_data).unwrap();
        let dir: String = String::from(&tileset.image);
        let tileset_img_dir: String = dir.replace("../..", "../assets");
        let mut img: DynamicImage = image::open(tileset_img_dir).unwrap();
        let tile_quantity: i16 = tileset.tilecount;
        let mut col: i16 = 0;
        let mut row: i16 = 0;
        let mut tile_id = tileset_id.firstgid;
        let columns: i16 = tileset.columns;

        if tilewidth == 0 {
            tilewidth = tileset.tilewidth;
        }

        if tileheight == 0 {
            tileheight = tileset.tileheight;
        }

        // This loop maps all tiles to their gid assigned by Tiled
        // TODO: this loop has an off-by-one bug need to fix
        for n in 0..tile_quantity {
            let x_offset: u32 = (tilewidth * col) as u32;
            let y_offset: u32 = (tileheight * row) as u32;
            let tile_img = img.crop(
                x_offset,
                y_offset,
                tileheight as u32,
                tileheight as u32);
            if n % columns == 0 {
                col = 0;
                row += 1;
            }
            tile_map.insert(tile_id, tile_img);
            col += 1;
            tile_id += 1;
        }
    }

    let layers: Vec<TileLayer> = tilemap.layers;
    for layer in layers {
        let data: Vec<i16> = layer.data;
        let columns: i16 = layer.width;
        let rows: i16 = layer.height;
        let imgx = (tilewidth * columns) as u32;
        let imgy = (tileheight * rows) as u32;
        let mut img: DynamicImage = DynamicImage::new_rgb16(imgx, imgy);
        let mut count: i16 = 0;
        let mut col: i16 = 0;
        let mut row: i16 = 0;
        let filename: String = String::from(layer.name) + ".png";
        // TODO: need to fix bugs for placing tiles at the correct offsets in the image buffer
        for n in data {
            if n != 0 {
                let tile = tile_map.get(&n).unwrap();
                img.copy_from(tile, (col * tilewidth) as u32, (row * tileheight) as u32).expect("TODO: panic message");
            }
            if count % columns == 0 {
                col = 0;
                row += 1;
            }
            count += 1;
        }
        img.save(&filename).expect("TODO: panic message");
    }
}