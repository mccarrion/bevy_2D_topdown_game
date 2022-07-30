mod map;

use map::*;

fn main() {
    // Note: below method call will eventually generate a series of PNG files based on Tiled config
    generate_map_from_tiled_config();
}