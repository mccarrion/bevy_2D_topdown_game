#version 410 core

in vec2 attrib_position;
uniform mat4 projection_matrix;
layout (location = 1) in vec2 vt; // per-vertex texture co-ords
out vec2 texture_coordinates;

void main() {
    texture_coordinates = vt;
    gl_Position = projection_matrix * vec4(attrib_position.xy, 0, 1);
}