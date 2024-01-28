pub const SIMPLE_VERTEX_SHADER_SOURCE: &[u8] = b"
#version 330 core

layout(location = 0) in vec4 position;

void main() {
    gl_Position = position;
}
\0";

pub const SIMPLE_FRAGMENT_SHADER_SOURCE: &[u8] = b"
#version 330 core

layout(location = 0) out vec4 color;

void main() {
    color = vec4(1.0, 1.0, 1.0, 1.0);
}
\0";

pub const VERTEX_SHADER_SOURCE: &[u8] = b"
#version 100
precision mediump float;

attribute vec2 position;
attribute vec3 color;

varying vec3 v_color;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    v_color = color;
}
\0";

pub const FRAGMENT_SHADER_SOURCE: &[u8] = b"
#version 100
precision mediump float;

varying vec3 v_color;

void main() {
    gl_FragColor = vec4(v_color, 1.0);
}
\0";