mod map;

use std::fs;
use map::*;

fn main() {
    // Creates output directory if it does not exist for all assets
    // and config files generated from build process
    fs::create_dir_all("../assets/output")
        .expect("Could not create output directory successfully");

    // Generate map layers from Tiled config
    generate_map_from_tiled_config();
}