#version 330 core

in vec2 v_tex_coord;

out vec4 color;

uniform sampler2D u_texture;

void main() {
    vec4 tex_color = texture(u_texture, v_tex_coord);
    color = tex_color;
}