mod simple_squre;
mod simple_triangle;

use std::ptr::null;

use crate::renderer::gl;

pub enum Sample {
    SimpleTriangle,
    SimpleSquare,
}

pub enum SampleProps {
    SimpleTriangle {
        program: gl::types::GLuint,
        vao: u32,
        vbo: u32,
    },
    SimpleSquare {
        program: gl::types::GLuint,
        vao: u32,
        vbo: u32,
        ibo: u32,
    },
}

impl Sample {
    pub fn create(&self, gl: &gl::Gl) -> SampleProps {
        match self {
            Self::SimpleTriangle => simple_triangle::create_sample(gl),
            Self::SimpleSquare => simple_squre::create_sample(gl),
        }
    }
}

impl SampleProps {
    pub unsafe fn draw(&self, gl: &gl::Gl) {
        match self {
            Self::SimpleTriangle { program, vao, vbo } => {
                gl.UseProgram(*program);

                gl.BindVertexArray(*vao);
                gl.BindBuffer(gl::ARRAY_BUFFER, *vbo);

                gl.ClearColor(0.1, 0.1, 0.1, 0.9);
                gl.Clear(gl::COLOR_BUFFER_BIT);

                gl.DrawArrays(gl::TRIANGLES, 0, 3);
            }

            Self::SimpleSquare { program, vao, vbo, ibo } => {
                gl.UseProgram(*program);

                gl.BindVertexArray(*vao);
                gl.BindBuffer(gl::ARRAY_BUFFER, *vbo);
                gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, *ibo);

                gl.ClearColor(0.1, 0.1, 0.1, 0.9);
                gl.Clear(gl::COLOR_BUFFER_BIT);

                gl.DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, null())
            }
        }
    }

    pub unsafe fn drop(&mut self, gl: &gl::Gl) {
        match self {
            Self::SimpleTriangle { program, vao, vbo } => {
                gl.DeleteProgram(*program);
                gl.DeleteBuffers(1, vbo);
                gl.DeleteVertexArrays(1, vao);
            }

            Self::SimpleSquare { program, vao, vbo, ibo } => {
                gl.DeleteProgram(*program);
                gl.DeleteBuffers(1, vbo);
                gl.DeleteBuffers(1, ibo);
                gl.DeleteVertexArrays(1, vao);
            }
        }
    }
}
