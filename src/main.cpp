#include <cstdio>
#include <cstdlib>
#include <cstdint>
#include <cstdarg>
#include <cstring>
#include <chrono>
#include <cmath>
#include <cassert>
#include <climits>
#include <ctime>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <sstream>
#include <unordered_map>

#include <GL/glew.h>
#include <GLFW/glfw3.h>
#include <nlohmann/json.hpp>

#define STB_IMAGE_IMPLEMENTATION

#include "stb_image.h"

#include <glm/glm.hpp>
#include <glm/gtc/matrix_transform.hpp>

#define WINDOW_WIDTH 1200
#define WINDOW_HEIGHT 800

#include "headers/shader.h"
#include "headers/tileset.h"
#include "headers/character.h"
#include "headers/graphics.h"

using namespace std;

struct background_color {
    float r, g, b, a;
};

int main() {

    /* GLFW */
    glfwSetErrorCallback(error_callback);
    if (!glfwInit()) {
        fprintf(stdout, "[GFLW] failed to init!\n");
        exit(1);
    }
    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 4);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 1);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
    glfwWindowHint(GLFW_TRANSPARENT_FRAMEBUFFER, 1);

#ifdef __APPLE__
    glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, GL_TRUE);
#endif

    static GLFWwindow *win = glfwCreateWindow(WINDOW_WIDTH,
                                              WINDOW_HEIGHT,
                                              "Topdown Game",
                                              nullptr,
                                              nullptr);
    glfwMakeContextCurrent(win);
    int width = 0, height = 0;
    float xscale = 1, yscale = 1;
    glfwGetWindowSize(win, &width, &height);
    glfwGetWindowContentScale(win, &xscale, &yscale);
    glViewport(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT);

    /* GLEW */
    glewExperimental = 1;
    if (glewInit() != GLEW_OK) {
        fprintf(stderr, "Failed to setup GLEW\n");
        exit(1);
    }

    // Init data and context structs
    window_info win_info = init_window_info(win);

    unique_ptr<background_color> bg(new background_color);
    bg->r = 182.0f / 255.0f,
    bg->g = 186.0f / 255.0f,
    bg->b = 186.0f / 255.0f,
    bg->a = 1.0f;

    // Create and compile our GLSL program from the shaders
    struct gl_shader_objects shader_objects = load_shader_objects("../src/shaders/vertex_shader.glsl",
                                                                  "../src/shaders/fragment_shader.glsl");


    texture_atlas atlas = load_texture_atlas("../assets/sprout_lands");
    struct character player = initialize_player(&atlas,
                                                "../assets/sprout_lands/characters/basic_character_spritesheet.png");
    struct tilemap tm = load_tilemap(&atlas); // TODO: map offsets from atlas to tilemap

    glEnable(GL_BLEND);
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
    glEnable(GL_CULL_FACE); // cull face
    glCullFace(GL_BACK);    // cull back face
    glFrontFace(GL_CCW);    // GL_CCW for counter clock-wise

    while (!glfwWindowShouldClose(win)) {
        // Collect events from inbetween last frame and current frame
        glfwPollEvents();
        update_window_info(&win_info);
        update_from_user_input(&win_info, &player);

        // Draw new frame
        glfwGetWindowSize(win, &width, &height);
        glViewport(0, 0, width * xscale, height * yscale);
        glClearColor(bg->r, bg->g, bg->b, bg->a);
        glClear(GL_COLOR_BUFFER_BIT);
        render_scene(shader_objects, player, tm, width, height);

        // Swap buffers
        glfwSwapBuffers(win);
    }
    shutdown_program(&win_info, &shader_objects);
    glfwTerminate();
    return 0;
}
