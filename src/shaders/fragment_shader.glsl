#version 410 core

in vec2 texture_coordinates;
uniform sampler2D basic_texture;
out vec4 color;

void main() {
    vec4 texel = texture (basic_texture, texture_coordinates);
    color = texel;
}
