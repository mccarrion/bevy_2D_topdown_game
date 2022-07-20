use std::collections::HashMap;
use std::fs;
use serde::*;
use serde_json::*;

#[derive(Serialize, Deserialize)]
pub struct TileSet {
    columns: i8,
    image: String,
    imageheight: i8,
    imagewidth: i8,
    margin: i8,
    name: String,
    spacing: i8,
    tilecount: i8,
    tiledversion: String,
    tileheight: i8,
    tilewidth: i8,
    #[serde(alias = "type")]
    ts_type: String,
    version: String
}

pub fn tileset_map_from_json() -> HashMap<String, TileSet> {
    let mut tileset_map: HashMap<String, TileSet> = HashMap::new();
    for file in fs::read_dir("./assets/tiled/tilesets").unwrap() {
        let result = file.unwrap().path().display();
    }
    let tileset_data = fs::read_to_string("./assets/tiled/tilesets/fences.json");
    //return from_str(&tileset_data.unwrap()).unwrap();
    return tileset_map;
}