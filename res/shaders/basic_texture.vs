#version 330 core

layout(location = 0) in vec4 position;
layout(location = 1) in vec2 tex_coord;

out vec2 v_tex_coord;

uniform mat4 u_proj;

void main() {
    gl_Position = position * u_proj;
    v_tex_coord = tex_coord;
}