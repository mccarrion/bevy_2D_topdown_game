namespace tiles {
    struct tileset {
        int columns;
        std::string image;
        int imageheight;
        int imagewidth;
        int margin;
        std::string name;
        int spacing;
        int tilecount;
        std::string tiledversion;
        int tileheight;
        int tilewidth;
        std::string ts_type;
        std::string version;
    };

    void to_json(nlohmann::json &j, const tileset &ts) {
        j = nlohmann::json{
                {"columns",      ts.columns},
                {"image",        ts.image},
                {"imageheight",  ts.imageheight},
                {"imagewidth",   ts.imagewidth},
                {"margin",       ts.margin},
                {"name",         ts.name},
                {"spacing",      ts.spacing},
                {"tilecount",    ts.tilecount},
                {"tiledversion", ts.tiledversion},
                {"tileheight",   ts.tileheight},
                {"tilewidth",    ts.tilewidth},
                {"type",         ts.ts_type},
                {"version",      ts.version}
        };
    }

    void from_json(const nlohmann::json &j, tileset &ts) {
        j.at("columns").get_to(ts.columns);
        j.at("image").get_to(ts.image);
        j.at("imageheight").get_to(ts.imageheight);
        j.at("imagewidth").get_to(ts.imagewidth);
        j.at("margin").get_to(ts.margin);
        j.at("name").get_to(ts.name);
        j.at("spacing").get_to(ts.spacing);
        j.at("tilecount").get_to(ts.tilecount);
        j.at("tiledversion").get_to(ts.tiledversion);
        j.at("tileheight").get_to(ts.tileheight);
        j.at("tilewidth").get_to(ts.tilewidth);
        j.at("type").get_to(ts.ts_type);
        j.at("version").get_to(ts.version);
    }

    struct tilesetid {
        int firstgid;
        std::string source;
    };

    void to_json(nlohmann::json &j, const tilesetid &ts_id) {
        j = nlohmann::json{
                {"firstgid", ts_id.firstgid},
                {"source",   ts_id.source}
        };
    }

    void from_json(const nlohmann::json &j, tilesetid &ts_id) {
        j.at("firstgid").get_to(ts_id.firstgid);
        j.at("source").get_to(ts_id.source);
    }


    struct tilelayer {
        std::vector<int> data;
        int height;
        int id;
        std::string name;
        int opacity;
        std::string tl_type;
        bool visible;
        int width;
        int x;
        int y;
    };

    void to_json(nlohmann::json &j, const tilelayer &tl) {
        j = nlohmann::json{
                {"data",    tl.data},
                {"height",  tl.height},
                {"id",      tl.id},
                {"name",    tl.name},
                {"opacity", tl.opacity},
                {"type",    tl.tl_type},
                {"visible", tl.visible},
                {"width",   tl.width},
                {"x",       tl.x},
                {"y",       tl.y}
        };
    }

    void from_json(const nlohmann::json &j, tilelayer &tl) {
        j.at("data").get_to(tl.data);
        j.at("height").get_to(tl.height);
        j.at("id").get_to(tl.id);
        j.at("name").get_to(tl.name);
        j.at("opacity").get_to(tl.opacity);
        j.at("type").get_to(tl.tl_type);
        j.at("visible").get_to(tl.visible);
        j.at("width").get_to(tl.width);
        j.at("x").get_to(tl.x);
        j.at("y").get_to(tl.y);
    }

    struct map {
        int compressionlevel;
        int height;
        bool infinite;
        std::vector<tilelayer> layers;
        int nextlayerid;
        int nextobjectid;
        std::string orientation;
        std::string renderorder;
        std::string tiledversion;
        int tileheight;
        std::vector<tilesetid> tilesetids;
        int tilewidth;
        std::string tm_type;
        std::string version;
        int width;
    };

    void to_json(nlohmann::json &j, const map &m) {
        j = nlohmann::json{
                {"compressionlevel", m.compressionlevel},
                {"height",           m.height},
                {"infinite",         m.infinite},
                {"layers",           m.layers},
                {"nextlayerid",      m.nextlayerid},
                {"nextobjectid",     m.nextobjectid},
                {"orientation",      m.orientation},
                {"renderorder",      m.renderorder},
                {"tiledversion",     m.tiledversion},
                {"tileheight",       m.tileheight},
                {"tilesets",         m.tilesetids},
                {"tilewidth",        m.tilewidth},
                {"type",             m.tm_type},
                {"version",          m.version},
                {"width",            m.width}
        };
    }

    void from_json(const nlohmann::json &j, map &m) {
        j.at("compressionlevel").get_to(m.compressionlevel);
        j.at("height").get_to(m.height);
        j.at("infinite").get_to(m.infinite);
        j.at("layers").get_to(m.layers);
        j.at("nextlayerid").get_to(m.nextlayerid);
        j.at("nextobjectid").get_to(m.nextobjectid);
        j.at("orientation").get_to(m.orientation);
        j.at("renderorder").get_to(m.renderorder);
        j.at("tiledversion").get_to(m.tiledversion);
        j.at("tileheight").get_to(m.tileheight);
        j.at("tilesets").get_to(m.tilesetids);
        j.at("tilewidth").get_to(m.tilewidth);
        j.at("type").get_to(m.tm_type);
        j.at("version").get_to(m.version);
        j.at("width").get_to(m.width);
    }

    struct tilemetadata {
        int id;
        bool left;
        bool right;
        bool top;
        bool bottom;
    };

    struct tilesprite {
        int atlas_sprite_id;
        bool left;
        bool right;
        bool top;
        bool bottom;
        int height;
        int width;
        float top_location,
                left_location,
                bottom_location,
                right_location;
    };
}

struct tilemap : object {
    std::unordered_map<int, tiles::tilesprite> tilesprite_by_id;
    tiles::map map;
};

std::string read_string_from_path(std::string file_path) {
    std::ifstream json_file(file_path, std::ios::in);
    std::string json_string;
    if (json_file.is_open()) {
        std::stringstream json_buff;
        json_buff << json_file.rdbuf();
        json_string = json_buff.str();
        json_file.close();
    } else {
        printf("Cannot open %s, please check path that is being used.\n", file_path.c_str());
        exit(1);
    }
    return json_string;
}

struct texture_data {
    int width, height, comp, x_atlas_offset, y_atlas_offset;
};

struct texture_atlas {
    int width, height;
    std::unordered_map<std::string, texture_data> image_data_map;
};

unique_ptr<texture_atlas> load_texture_atlas(const char *dir_name) {
    GLuint textures;
    int force_channels = 4;
    stbi_set_flip_vertically_on_load(true);

    std::string parent_dir = {};
    std::unordered_map<std::string, texture_data> image_data_map;

    // naive texture packing algorithm
    int total_width = 0, max_height = 0;
    for (const auto &dir_entry: std::filesystem::recursive_directory_iterator(dir_name)) {
        std::cout << dir_entry << std::endl;
        std::string temp_parent_dir = dir_entry.path().parent_path().filename().generic_string();
        if (parent_dir.empty()) {
            parent_dir = temp_parent_dir;
        }
        if (parent_dir != temp_parent_dir) {
            parent_dir = temp_parent_dir;
        }
        std::string current_file = dir_entry.path().generic_string();
        std::size_t found = current_file.find(".png");
        if (found != std::string::npos) {
            struct texture_data tex_data{};
            stbi_info(current_file.c_str(), &tex_data.width, &tex_data.height, &tex_data.comp);
            tex_data.x_atlas_offset = total_width;
            tex_data.y_atlas_offset = 0;
            total_width += tex_data.width;
            if (max_height < tex_data.height) {
                max_height = tex_data.height;
            }
            image_data_map[current_file] = tex_data;
        }
    }

    unique_ptr<texture_atlas> atlas(new texture_atlas);
    atlas->height = max_height;
    atlas->width = total_width;
    atlas->image_data_map = image_data_map;

    glGenTextures(1, &textures);
    glActiveTexture(GL_TEXTURE0);
    glBindTexture(GL_TEXTURE_2D, textures);
    glTexStorage2D(GL_TEXTURE_2D, 1, GL_RGBA8, atlas->width, atlas->height);
    for (auto &[file_name, tex]: image_data_map) {
        unsigned char *image_data = stbi_load(file_name.c_str(), &tex.width, &tex.height, &tex.comp, force_channels);
        if (!image_data) {
            fprintf(stderr, "ERROR: could not load %s\n", file_name.c_str());
            exit(1);
        }
        glTexSubImage2D(GL_TEXTURE_2D, 0, tex.x_atlas_offset, tex.y_atlas_offset, tex.width, tex.height, GL_RGBA,
                        GL_UNSIGNED_BYTE, image_data);
        stbi_image_free(image_data); // TODO: not sure if this actually removes the data from the heap
    }
    glGenerateMipmap(GL_TEXTURE_2D);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR_MIPMAP_LINEAR);
    GLfloat max_aniso = 0.0f;
    glGetFloatv(GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT, &max_aniso);
    glTexParameterf(GL_TEXTURE_2D, GL_TEXTURE_MAX_ANISOTROPY_EXT, max_aniso);
    return atlas;
}


unique_ptr<tilemap> load_tilemap(const texture_atlas *atlas) {
    unique_ptr<tilemap> tm(new tilemap);
    std::string tileset_map_json = read_string_from_path("../assets/tiled/maps/sprout_land.tmj");
    nlohmann::json tmj = nlohmann::json::parse(tileset_map_json);
    auto tmap = tmj.template get<tiles::map>();
    std::vector<tiles::tilesetid> tilesetids = tmap.tilesetids;
    std::unordered_map<int, tiles::tilesprite> tilesprite_map;
    for (tiles::tilesetid tsid: tilesetids) {
        std::string tsid_source = tsid.source;
        tsid_source.replace(tsid_source.begin(), tsid_source.begin() + 2, "../assets/tiled");
        std::string tileset_json = read_string_from_path(tsid_source);
        nlohmann::json tsj = nlohmann::json::parse(tileset_json);
        tiles::tileset ts = tsj.template get<tiles::tileset>();
        std::string image_location = ts.image;
        image_location.replace(image_location.begin(), image_location.begin() + 5, "../assets");
        auto image_data_map = atlas->image_data_map;
        texture_data tex = image_data_map[image_location];
        int col = 0;
        int row = 0;
        for (int i = tsid.firstgid; i < tsid.firstgid + ts.tilecount; i++) {
            tiles::tilesprite tsp = {};
            tsp.top_location =
                    (float) (tex.y_atlas_offset + ts.imageheight - (ts.tileheight * row)) /
                    (float) atlas->height;
            tsp.bottom_location =
                    (float) (tex.y_atlas_offset + ts.imageheight - ts.tileheight - (ts.tileheight * row)) /
                    (float) atlas->height;
            tsp.left_location =
                    (float) (tex.x_atlas_offset + (ts.tilewidth * col)) /
                    (float) atlas->width;
            tsp.right_location =
                    (float) (tex.x_atlas_offset + ts.tilewidth + (ts.tilewidth * col)) /
                    (float) atlas->width;
            tsp.height = ts.tileheight;
            tsp.width = ts.tilewidth;
            col++;
            if (col % ts.columns == 0) {
                col = 0;
                row++;
            }
            tilesprite_map[i] = tsp;
        }
    }
    tm->tilesprite_by_id = tilesprite_map;
    tm->map = tmap;
    return tm;
}
