mod simple_triangle;

use crate::renderer::gl;

pub enum Sample {
    SimpleTriangle,
}

pub enum SampleProps {
    SimpleTriangle {
        program: gl::types::GLuint,
        vao: u32,
        vbo: u32,
    },
}

impl Sample {
    pub fn create(&self, gl: &gl::Gl) -> SampleProps {
        match self {
            Self::SimpleTriangle => simple_triangle::create_sample(gl),
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
        }
    }

    pub unsafe fn drop(&mut self, gl: &gl::Gl) {
        match self {
            Self::SimpleTriangle { program, vao, vbo } => {
                gl.DeleteProgram(*program);
                gl.DeleteBuffers(1, vbo);
                gl.DeleteVertexArrays(1, vao);
            }
        }
    }
}
