struct gl_shader_objects {
    GLuint vertex_buffer_object,
            vertex_array_object,
            element_buffer_object;
    GLuint program;
    GLint attrib_position;
    GLint projection_matrix;
};

struct window_info {
    GLFWwindow *win;
    int width, height;
    int display_width, display_height;
};

GLuint generate_shader_program(const char *vertex_file_path,
                               const char *fragment_file_path) {
    // Read the Vertex Shader code from the file
    std::ifstream vert_file(vertex_file_path, std::ios::in);
    std::string vert_string;
    if (vert_file.is_open()) {
        std::stringstream vert_buff;
        vert_buff << vert_file.rdbuf();
        vert_string = vert_buff.str();
        vert_file.close();
    } else {
        printf("Cannot open %s, please check path that is being used.\n", vertex_file_path);
        exit(1);
    }
    char const *vert_shader = vert_string.c_str();

    // Read the Fragment Shader code from the file
    std::string frag_string;
    std::ifstream frag_file(fragment_file_path, std::ios::in);
    if (frag_file.is_open()) {
        std::stringstream frag_buff;
        frag_buff << frag_file.rdbuf();
        frag_string = frag_buff.str();
        frag_file.close();
    } else {
        printf("Cannot open %s, please check path that is being used.\n", fragment_file_path);
        exit(1);
    }
    char const *frag_shader = frag_string.c_str();

    // Create the shaders
    GLint status = GL_FALSE;
    int log_length;
    GLuint vertex_shader = glCreateShader(GL_VERTEX_SHADER);
    GLuint fragment_shader = glCreateShader(GL_FRAGMENT_SHADER);

    // Compile Vertex Shader
    printf("Compiling shader : %s\n", vertex_file_path);
    glShaderSource(vertex_shader, 1, &vert_shader, nullptr);
    glCompileShader(vertex_shader);
    glGetShaderiv(vertex_shader, GL_COMPILE_STATUS, &status);
    glGetShaderiv(vertex_shader, GL_INFO_LOG_LENGTH, &log_length);
    assert(status == GL_TRUE);

    // Compile Fragment Shader
    glShaderSource(fragment_shader, 1, &frag_shader, nullptr);
    glCompileShader(fragment_shader);
    glGetShaderiv(fragment_shader, GL_COMPILE_STATUS, &status);
    glGetShaderiv(fragment_shader, GL_INFO_LOG_LENGTH, &log_length);
    assert(status == GL_TRUE);

    // Create Shader Program
    GLuint program = glCreateProgram();
    glAttachShader(program, vertex_shader);
    glAttachShader(program, fragment_shader);
    glLinkProgram(program);
    glGetProgramiv(program, GL_LINK_STATUS, &status);
    glGetProgramiv(program, GL_INFO_LOG_LENGTH, &log_length);
    assert(status == GL_TRUE);

    // clean up shader data
    glDetachShader(program, vertex_shader);
    glDetachShader(program, fragment_shader);
    glDeleteShader(vertex_shader);
    glDeleteShader(fragment_shader);

    // return pointer for shader program
    return program;
}

struct gl_shader_objects load_shader_objects(const char *vertex_file_path,
                                             const char *fragment_file_path) {
    // Begin initializing draw data
    gl_shader_objects shader_objects{};
    shader_objects.program = generate_shader_program(vertex_file_path, fragment_file_path);

    // initialize location data
    shader_objects.projection_matrix = glGetUniformLocation(shader_objects.program, "projection_matrix");
    shader_objects.attrib_position = glGetAttribLocation(shader_objects.program, "attrib_position");

    glGenBuffers(1, &shader_objects.vertex_buffer_object);
    glGenBuffers(1, &shader_objects.element_buffer_object);
    glGenVertexArrays(1, &shader_objects.vertex_array_object);

    glBindVertexArray(shader_objects.vertex_array_object);
    glBindBuffer(GL_ARRAY_BUFFER, shader_objects.vertex_buffer_object);
    glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, shader_objects.element_buffer_object);

    glEnableVertexAttribArray((GLuint) shader_objects.attrib_position);
    glVertexAttribPointer((GLuint) shader_objects.attrib_position, 2, GL_FLOAT, GL_FALSE, 0, NULL);

    glBindTexture(GL_TEXTURE_2D, 0);
    glBindBuffer(GL_ARRAY_BUFFER, 0);
    glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0);
    glBindVertexArray(0);

    return shader_objects;
}