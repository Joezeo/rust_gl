use std::cell::Cell;

use super::SampleProps;
use crate::{
    gl_call,
    shaders::{create_shader, ShaderSource, ShaderType},
};
use gl::types::{GLsizei, GLsizeiptr};

#[rustfmt::skip]
const VERTEX_DATA: [f32; 8] = [
    -0.5, -0.5,  // 0
     0.5, -0.5,  // 1
     0.5,  0.5,  // 2
    -0.5,  0.5,  // 3
];

#[rustfmt::skip]
const INDICES: [u32; 6] = [
    0, 1, 2, 
    2, 3, 0
];

pub unsafe fn create_sample(gl: &super::gl::Gl) -> SampleProps {
    // Create shaders:
    let program = gl_call!(gl, CreateProgram());
    let vertex_shader = create_shader(
        &gl,
        gl::VERTEX_SHADER,
        &ShaderSource::load("basic", ShaderType::Vertex),
    );
    let fragment_shader = create_shader(
        &gl,
        gl::FRAGMENT_SHADER,
        &ShaderSource::load("basic_uniform", ShaderType::Fragment),
    );
    gl_call!(gl, AttachShader(program, vertex_shader));
    gl_call!(gl, AttachShader(program, fragment_shader));

    gl_call!(gl, LinkProgram(program));
    gl_call!(gl, UseProgram(program));

    gl_call!(gl, DeleteShader(vertex_shader));
    gl_call!(gl, DeleteShader(fragment_shader));

    // Create vertex array object:
    let mut vao = std::mem::zeroed();
    gl_call!(gl, GenVertexArrays(1, &mut vao));
    gl_call!(gl, BindVertexArray(vao));

    // Create vertex buffer object:
    let mut vbo = std::mem::zeroed();
    gl_call!(gl, GenBuffers(1, &mut vbo));
    // In OpenGl, `bind` means `select`.
    gl_call!(gl, BindBuffer(gl::ARRAY_BUFFER, vbo));
    gl_call!(
        gl,
        BufferData(
            gl::ARRAY_BUFFER,
            (VERTEX_DATA.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
            VERTEX_DATA.as_ptr() as *const _,
            gl::STATIC_DRAW,
        )
    );

    // Create index buffer object:
    let mut ibo = std::mem::zeroed();
    gl_call!(gl, GenBuffers(1, &mut ibo));
    gl_call!(gl, BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo));
    gl_call!(
        gl,
        BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (INDICES.len() * std::mem::size_of::<u32>()) as GLsizeiptr,
            INDICES.as_ptr() as *const _,
            gl::STATIC_DRAW,
        )
    );

    gl_call!(
        gl,
        VertexAttribPointer(
            0,
            2,
            gl::FLOAT,
            gl::FALSE,
            (std::mem::size_of::<f32>() * 2) as GLsizei,
            0 as *const _,
        )
    );
    gl_call!(gl, EnableVertexAttribArray(0));

    SampleProps::SimpleSquare {
        program,
        vao,
        vbo,
        ibo,
        r: Cell::new(0.),
        inc: Cell::new(0.05),
    }
}
