typedef std::chrono::high_resolution_clock time_tracker;

struct frame {
    float top, left, bottom, right;
};

struct character {
    float x_translation;
    float y_translation;
    int frame_offset;
    int anim_frame_length;
    std::chrono::steady_clock::time_point last_updated;
    long time_per_frame;
    std::unordered_map<int, frame> char_frames;
};

std::unordered_map<int, frame> init_character_frames(int num_columns, int num_rows, texture_data * tex, texture_atlas * atlas) {
    std::unordered_map<int, frame> character_frames;
    int num_frames = num_columns * num_rows;

    float x_left_crop = 0.08f;
    float x_right_crop = 0.08f;

    float y_bottom_crop = 0.08f;
    float y_top_crop = 0.08f;

    float x_offset = 0.0f;
    float y_offset = 1.0f - 1.0f / (float) num_rows;

    int frame_num = 1;
    for (int i = 0; i < num_frames; i++) {
        struct frame character_frame{};
        character_frame.left = ((0.0f + x_left_crop + x_offset) * (float) tex->width / (float) atlas->width) + ((float) tex->x_atlas_offset / (float) atlas->width);
        character_frame.right = ((1.0f / 4 - x_right_crop + x_offset) * (float) tex->width / (float) atlas->width) + ((float) tex->x_atlas_offset / (float) atlas->width);
        character_frame.bottom = ((0.0f + y_bottom_crop + y_offset) * (float) tex->height / (float) atlas->height) + ((float) tex->y_atlas_offset / (float) atlas->height);
        character_frame.top = ((1.0f / 4 - y_top_crop + y_offset) * (float) tex->height / (float) atlas->height) + ((float) tex->y_atlas_offset / (float) atlas->height);
        x_offset += 1.0f / (float) num_columns;
        if ((int) (100.0f * x_offset) % 100 == 0) {
            x_offset = 0.0;
            y_offset -= 1.0f / (float) num_rows;
        }
        character_frames[frame_num] = character_frame;
        frame_num++;
    }
    return character_frames;
}

struct character initialize_player(texture_atlas * atlas, std::string file_name) {
    struct character p = {};
    p.x_translation = 0;
    p.y_translation = 0;
    p.frame_offset = 16;
    p.time_per_frame = (long) ((double) (1.0 / 10.0) * 1000000000);
    p.last_updated = time_tracker::now();
    p.anim_frame_length = 4;
    texture_data tex = atlas->image_data_map[file_name];
    p.char_frames = init_character_frames(4, 4, &tex, atlas);
    return p;
}

void update_from_user_input(struct window_info *win_meta, struct character *motion) {
    struct GLFWwindow *win = win_meta->win;

    auto current_time = time_tracker::now();
    auto time_elapsed = current_time - motion->last_updated;
    long time_passed = (long) time_elapsed.count();

    if (glfwGetKey(win, GLFW_KEY_W) == GLFW_PRESS) {
        motion->y_translation += 0.25;
        if (time_passed > motion->time_per_frame) {
            motion->last_updated = time_tracker::now();
            motion->anim_frame_length += 1;
            motion->frame_offset = motion->anim_frame_length + 4;
        }
        if (motion->anim_frame_length % 4 == 0 ) {
            motion->anim_frame_length = 1;
        }
    }
    if (glfwGetKey(win, GLFW_KEY_S) == GLFW_PRESS) {
        motion->y_translation -= 0.25;
        if (time_passed > motion->time_per_frame) {
            motion->last_updated = time_tracker::now();
            motion->anim_frame_length += 1;
            motion->frame_offset = motion->anim_frame_length;
        }
        if (motion->anim_frame_length % 4 == 0 ) {
            motion->anim_frame_length = 1;
        }
    }
    if (glfwGetKey(win, GLFW_KEY_A) == GLFW_PRESS) {
        motion->x_translation -= 0.25;
        if (time_passed > motion->time_per_frame) {
            motion->last_updated = time_tracker::now();
            motion->anim_frame_length += 1;
            motion->frame_offset = motion->anim_frame_length + 8;
        }
        if (motion->anim_frame_length % 4 == 0 ) {
            motion->anim_frame_length = 1;
        }
    }
    if (glfwGetKey(win, GLFW_KEY_D) == GLFW_PRESS) {
        motion->x_translation += 0.25;
        if (time_passed > motion->time_per_frame) {
            motion->last_updated = time_tracker::now();
            motion->anim_frame_length += 1;
            motion->frame_offset = motion->anim_frame_length + 12;
        }
        if (motion->anim_frame_length % 4 == 0 ) {
            motion->anim_frame_length = 1;
        }
    }
}
