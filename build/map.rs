use std::borrow::Borrow;
use std::collections::HashMap;
use std::fs;
use std::fs::DirEntry;
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
    version: String
}

#[derive(Serialize, Deserialize)]
struct TileSetId {
    firstgid: i16,
    source: String
}

#[derive(Serialize, Deserialize)]
struct TileMap {
    compressionlevel: i16,
    height: i16,
    infinite: bool,
    layers: Vec<TileLayer>, // update to be list of structs
    nextlayerid: i16,
    nextobjectid: i16,
    orientation: String,
    renderorder: String,
    tiledversion: String,
    tileheight: i16,
    tilesets: Vec<TileSetId>, // update to be list of structs
    tilewidth: i16,
    #[serde(alias = "type")]
    tm_type: String,
    version: String,
    width: i16
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

    /*
     * This code generates two maps based on Tiled config
     *      tileset_map: this is a map of the firstgid of a TileSet config to a defined TileSet
     *      range_cache_map: this is a map of each id for a TileSet to the firstgid in a TileSet
     *
     * These two maps are sufficient to begin drawing PNG files of map layers based on Tiled config
     */
    let mut tileset_map: HashMap<i16, TileSet> = HashMap::new();
    let mut range_cache_map: HashMap<i16, i16> = HashMap::new();
    for tileset_id in tileset_ids {
        let str: String = String::from(&tileset_id.source);
        let tileset_dir: String = str.replace("..", "../assets/tiled");
        let tileset_data: String = fs::read_to_string(tileset_dir).unwrap();
        let tileset: TileSet = from_str(&tileset_data).unwrap();
        let lastgid = tileset_id.firstgid + tileset.tilecount;
        for n in tileset_id.firstgid..lastgid {
            range_cache_map.insert(n,tileset_id.firstgid);
        }
        tileset_map.insert(tileset_id.firstgid, tileset);
    }


}