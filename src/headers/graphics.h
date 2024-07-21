std::unique_ptr<window_info> init_window_info(GLFWwindow *win) {
    std::unique_ptr<window_info> win_meta(new window_info);
    glfwSetWindowUserPointer(win, &win_meta);
    win_meta->win = win;
    return win_meta;
}

unique_ptr<window_info> update_window_info(unique_ptr<window_info> win_info) {
    struct GLFWwindow *win = win_info->win;
    glfwGetWindowSize(win, &win_info->width, &win_info->height);
    glfwGetFramebufferSize(win, &win_info->display_width, &win_info->display_height);
    return win_info;
}

void shutdown_program(unique_ptr<window_info> *win_info,
                      unique_ptr<gl_shader_objects> shader_objects) {
    glDeleteProgram(shader_objects->program);
    glDeleteBuffers(1, &shader_objects->vertex_buffer_object);
    glDeleteBuffers(1, &shader_objects->element_buffer_object);
    memset(win_info, 0, sizeof(*win_info));
}

static void error_callback(int e, const char *d) {
    printf("Error %d: %s\n", e, d);
}

struct vertex_data {
    std::vector<float> vertices;
    std::vector<float> texcoords; // 2^16 = 65536
    std::vector<int> start_offsets;
    std::vector<int> num_vertices;
    std::unordered_map<std::string, int> start_offset_map;
    int player_vertex_offset{};
    int player_texture_offset{};
};

unique_ptr<vertex_data> calculate_tilemap_vertices(const character *player,
                                                   const tilemap *tm) {
    unique_ptr<character> p(new character);
    unique_ptr<vertex_data> vert_data(new vertex_data);
    std::vector<float> vertices;
    std::vector<float> texcoords; // 2^16 = 65536
    std::vector<int> start_offsets;
    std::vector<int> num_vertices;
    std::unordered_map<std::string, int> start_offset_map;
    int startOffset = 0;

    for (const tiles::tilelayer &tl: tm->map.layers) {
        std::vector<int> data = tl.data;
        float x = -40;
        float y = 0;
        int width = tl.width;
        int count = 0;
        auto tilesprite_by_id = tm->tilesprite_by_id;
        for (int tilegid: data) {
            tiles::tilesprite tsp = tilesprite_by_id[tilegid];
            vertices.insert(vertices.end(), {
                    0.3f + x, -0.3f + y, 0.0f,
                    0.3f + x, 0.3f + y, 0.0f,
                    -0.3f + x, 0.3f + y, 0.0f,
                    -0.3f + x, -0.3f + y, 0.0f
            });
            num_vertices.insert(num_vertices.end(), 4);
            start_offsets.insert(start_offsets.end(), startOffset);
            startOffset += 4;
            texcoords.insert(texcoords.end(), {
                    tsp.right_location, tsp.bottom_location,    // bottom right
                    tsp.right_location, tsp.top_location,       // top right
                    tsp.left_location, tsp.top_location,        // top left
                    tsp.left_location, tsp.bottom_location      // bottom left
            });
            count++;
            x += 0.6f;
            if (count >= width) {
                count = 0;
                x = -40.0f;
                y -= 0.6f;
            }
        }
    }

    num_vertices.insert(num_vertices.end(), 4);
    start_offsets.insert(start_offsets.end(), startOffset);
    startOffset += 4;

    int player_vertex_offset = (int) vertices.size();
    vertices.insert(vertices.end(), {
            0.3f + player->x_translation, -0.3f + player->y_translation, 0.0f,
            0.3f + player->x_translation, 0.3f + player->y_translation, 0.0f,
            -0.3f + player->x_translation, 0.3f + player->y_translation, 0.0f,
            -0.3f + player->x_translation, -0.3f + player->y_translation, 0.0f
    });
    auto frames = player->char_frames;
    auto frame_offset = player->frame_offset;

    auto char_frame = frames[frame_offset];
    int player_texture_offset = (int) texcoords.size();
    texcoords.insert(texcoords.end(), {
            char_frame.right, char_frame.bottom,    // bottom right
            char_frame.right, char_frame.top,       // top right
            char_frame.left, char_frame.top,        // top left
            char_frame.left, char_frame.bottom      // bottom left
    });
    vert_data->vertices = vertices;
    vert_data->texcoords = texcoords;
    vert_data->start_offsets = start_offsets;
    vert_data->num_vertices = num_vertices;
    vert_data->start_offset_map = start_offset_map;
    vert_data->player_texture_offset = player_texture_offset;
    vert_data->player_vertex_offset = player_vertex_offset;
    return vert_data;
}

unique_ptr<vertex_data> update_player_vertices(unique_ptr<vertex_data> vert_data, character *player) {
    std::vector<float> vertices = vert_data->vertices;
    std::vector<float> texcoords = vert_data->texcoords; // 2^16 = 65536
    float player_vertices[12] = {
            0.3f + player->x_translation, -0.3f + player->y_translation, 0.0f,
            0.3f + player->x_translation, 0.3f + player->y_translation, 0.0f,
            -0.3f + player->x_translation, 0.3f + player->y_translation, 0.0f,
            -0.3f + player->x_translation, -0.3f + player->y_translation, 0.0f
    };
    for (int i = 0; i < 12; i++) {
        vertices.at(vert_data->player_vertex_offset + i) = player_vertices[i];
    }

    auto frames = player->char_frames;
    auto frame_offset = player->frame_offset;

    auto char_frame = frames[frame_offset];
    float player_texcoords[8] = {
            char_frame.right, char_frame.bottom,    // bottom right
            char_frame.right, char_frame.top,       // top right
            char_frame.left, char_frame.top,        // top left
            char_frame.left, char_frame.bottom      // bottom left
    };
    for (int i = 0; i < 8; i++) {
        texcoords.at(vert_data->player_texture_offset + i) = player_texcoords[i];
    }
    vert_data->texcoords = texcoords;
    vert_data->vertices = vertices;
    return vert_data;
}

void render_scene(const gl_shader_objects *shader_objects, const character *player,
                  const vertex_data *vert_data,
                  int window_width, int window_height) {
    // Create projection matrix using an Orthographic camera
    glm::mat4 model = glm::mat4(1.0f);
    glm::mat4 view = glm::lookAt(
            glm::vec3(0, 3, 3),
            glm::vec3(0, 0, 0),
            glm::vec3(0, 1, 0)
    );
    float aspect = (float) window_width / (float) window_height;
    glm::mat4 projection = glm::ortho(-4.0f * aspect + player->x_translation, 4.0f * aspect + player->x_translation,
                                      -3.0f + (player->y_translation / (aspect - 0.1f)),
                                      3.0f + (player->y_translation / (aspect - 0.1f)),
                                      0.0f, 100.0f);
    glm::mat4 projection_matrix = projection * view * model;

    std::vector<float> vertices = vert_data->vertices;
    std::vector<float> texcoords = vert_data->texcoords; // 2^16 = 65536
    std::vector<int> start_offsets = vert_data->start_offsets;
    std::vector<int> num_vertices = vert_data->num_vertices;

    // setup program
    glUseProgram(shader_objects->program);
    glUniformMatrix4fv(shader_objects->projection_matrix, 1, GL_FALSE, &projection_matrix[0][0]);

    // Bind buffer of vertices and start at point 0 in attrib array
    glBindVertexArray(shader_objects->vertex_array_object);
    glBindBuffer(GL_ARRAY_BUFFER, shader_objects->vertex_buffer_object);
    glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, shader_objects->element_buffer_object);
    glBufferData(GL_ARRAY_BUFFER, vertices.size() * sizeof(float), &vertices.front(), GL_STATIC_DRAW);

    GLuint texcoords_vertex_buffer_object;
    glGenBuffers(1, &texcoords_vertex_buffer_object);
    glBindBuffer(GL_ARRAY_BUFFER, texcoords_vertex_buffer_object);
    glBufferData(GL_ARRAY_BUFFER, texcoords.size() * sizeof(float), &texcoords.front(), GL_STATIC_DRAW);
    glBindBuffer(GL_ARRAY_BUFFER, shader_objects->vertex_buffer_object);

    glVertexAttribPointer(
            0,
            3,
            GL_FLOAT,
            GL_FALSE,
            0,
            NULL
    );
    glBindBuffer(GL_ARRAY_BUFFER, texcoords_vertex_buffer_object);
    glVertexAttribPointer(1, 2, GL_FLOAT, GL_FALSE, 0, NULL); // normalise!
    glEnableVertexAttribArray(0);
    glEnableVertexAttribArray(1);

    // Draw the shape
    glMultiDrawArrays(GL_TRIANGLE_FAN, &start_offsets.front(), &num_vertices.front(), (int) start_offsets.size());

    // clean up
    glUseProgram(0);
    glBindBuffer(GL_ARRAY_BUFFER, 0);
    glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0);
    glBindVertexArray(0);
    glDisableVertexAttribArray(0);
}