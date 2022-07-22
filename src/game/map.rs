use std::collections::HashMap;
use std::fs;
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
    version: String
}

#[derive(Serialize, Deserialize)]
pub struct TileMap {
    compressionlevel: String,
    height: i16,
    infinite: String,
    layers: Vec<String>, // update to be list of structs
    nextlayerid: i16,
    nextobjectid: i16,
    orientation: String,
    renderorder: String,
    tiledversion: String,
    tileheight: i16,
    tilesets: Vec<String>, // update to be list of structs
    tilewidth: i16,
    #[serde(alias = "type")]
    tm_type: String,
    version: String,
    width: i16
}

pub fn generate_map_from_tiled_config() {
    let mut tileset_map: HashMap<String, TileSet> = HashMap::new();
    for file in fs::read_dir("./assets/tiled/tilesets").unwrap() {
        let tileset_data = fs::read_to_string(
            file.unwrap().path().display().to_string());
        let tileset: TileSet = from_str(&tileset_data.unwrap()).unwrap();
        tileset_map.insert(String::from(&tileset.name), tileset);
    }
}