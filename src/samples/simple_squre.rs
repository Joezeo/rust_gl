use super::SampleProps;
use crate::shaders::{create_shader, ShaderSource, ShaderType};
use gl::types::{GLsizei, GLsizeiptr};

#[rustfmt::skip]
const VERTEX_DATA: [f32; 8] = [
    -0.5, -0.5, // 0
    0.5, -0.5,  // 1
    0.5, 0.5,   // 2
    -0.5, 0.5,  // 3
];

#[rustfmt::skip]
const INDICES: [u32; 6] = [
    0, 1, 2, 
    2, 3, 0
];

pub fn create_sample(gl: &super::gl::Gl) -> SampleProps {
    unsafe {
        // Create shaders:
        let program = gl.CreateProgram();
        let vertex_shader = create_shader(
            &gl,
            gl::VERTEX_SHADER,
            &ShaderSource::load("simple_triangle", ShaderType::Vertex),
        );
        let fragment_shader = create_shader(
            &gl,
            gl::FRAGMENT_SHADER,
            &ShaderSource::load("simple_triangle", ShaderType::Fragment),
        );
        gl.AttachShader(program, vertex_shader);
        gl.AttachShader(program, fragment_shader);

        gl.LinkProgram(program);
        gl.UseProgram(program);

        gl.DeleteShader(vertex_shader);
        gl.DeleteShader(fragment_shader);

        // Create vertex array object:
        let mut vao = std::mem::zeroed();
        gl.GenVertexArrays(1, &mut vao);
        gl.BindVertexArray(vao);

        // Create vertex buffer object:
        let mut vbo = std::mem::zeroed();
        gl.GenBuffers(1, &mut vbo);
        // In OpenGl, `bind` means `select`.
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl.BufferData(
            gl::ARRAY_BUFFER,
            (VERTEX_DATA.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
            VERTEX_DATA.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        // Create index buffer object:
        let mut ibo = std::mem::zeroed();
        gl.GenBuffers(1, &mut ibo);
        gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);
        gl.BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (INDICES.len() * std::mem::size_of::<u32>()) as GLsizeiptr,
            INDICES.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        gl.VertexAttribPointer(
            0,
            2,
            gl::FLOAT,
            gl::FALSE,
            (std::mem::size_of::<f32>() * 2) as GLsizei,
            0 as *const _,
        );
        gl.EnableVertexAttribArray(0);

        SampleProps::SimpleSquare { program, vao, vbo, ibo }
    }
}
