use super::SampleProps;
use crate::shaders::{create_shader, ShaderSource, ShaderType};
use gl::types::{GLsizei, GLsizeiptr};

#[rustfmt::skip]
const VERTEX_DATA: [f32; 6] = [
    -0.5, -0.5,
    0.0, 0.5,
    0.5, -0.5,
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

        gl.VertexAttribPointer(
            0,
            2,
            gl::FLOAT,
            gl::FALSE,
            (std::mem::size_of::<f32>() * 2) as GLsizei,
            0 as *const _,
        );
        gl.EnableVertexAttribArray(0);

        SampleProps::SimpleTriangle { program, vao, vbo }
    }
}
