mod simple_squre;
mod simple_triangle;

use crate::{
    gl_call,
    renderer::{as_gl_char_ptr, gl},
};
use std::{cell::Cell, ptr::null};

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

        r: Cell<f32>,
        inc: Cell<f32>,
    },
}

impl Sample {
    pub unsafe fn create(&self, gl: &gl::Gl) -> SampleProps {
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
                gl_call!(gl, UseProgram(*program));

                gl_call!(gl, BindVertexArray(*vao));
                gl_call!(gl, BindBuffer(gl::ARRAY_BUFFER, *vbo));

                gl_call!(gl, ClearColor(0.1, 0.1, 0.1, 0.9));
                gl_call!(gl, Clear(gl::COLOR_BUFFER_BIT));

                gl_call!(gl, DrawArrays(gl::TRIANGLES, 0, 3));
            }

            Self::SimpleSquare {
                program,
                vao,
                vbo,
                ibo,
                ..
            } => {
                gl_call!(gl, UseProgram(*program));

                gl_call!(gl, BindVertexArray(*vao));
                gl_call!(gl, BindBuffer(gl::ARRAY_BUFFER, *vbo));
                gl_call!(gl, BindBuffer(gl::ELEMENT_ARRAY_BUFFER, *ibo));

                gl_call!(gl, ClearColor(0.1, 0.1, 0.1, 0.9));
                gl_call!(gl, Clear(gl::COLOR_BUFFER_BIT));

                gl_call!(gl, DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, null()));
            }
        }
    }

    pub unsafe fn snapshot(&self, gl: &gl::Gl) -> bool {
        match self {
            Self::SimpleSquare { program, r, inc, .. } => {
                gl_call!(gl, UseProgram(*program));

                let location =
                    gl_call!(gl, GetUniformLocation(*program, as_gl_char_ptr("u_color\0")));
                assert!(location != -1);

                let rc = r.get();
                gl_call!(gl, Uniform4f(location, rc, 0.2, 0.75, 0.9));

                if rc > 1. {
                    inc.set(-0.05)
                } else if rc < 0. {
                    inc.set(0.05)
                }
                r.set(rc + inc.get());

                true
            }
            _ => false,
        }
    }

    pub unsafe fn drop(&mut self, gl: &gl::Gl) {
        match self {
            Self::SimpleTriangle { program, vao, vbo } => {
                gl.DeleteProgram(*program);
                gl.DeleteBuffers(1, vbo);
                gl.DeleteVertexArrays(1, vao);
            }

            Self::SimpleSquare {
                program,
                vao,
                vbo,
                ibo,
                ..
            } => {
                gl.DeleteProgram(*program);
                gl.DeleteBuffers(1, vbo);
                gl.DeleteBuffers(1, ibo);
                gl.DeleteVertexArrays(1, vao);
            }
        }
    }
}
