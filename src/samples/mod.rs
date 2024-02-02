mod simple_square;
mod simple_triangle;
mod textured_square;

use crate::{gl_call, renderer::gl, shaders::Shader, texture::Texture};
use std::{cell::Cell, ptr::null};

pub enum Sample {
    SimpleTriangle,
    SimpleSquare,
    TexturedSquare,
}

pub enum SampleProps {
    SimpleTriangle {
        shader: Shader,
        vao: u32,
        vbo: u32,
    },
    SimpleSquare {
        shader: Shader,
        vao: u32,
        vbo: u32,
        ibo: u32,

        r: Cell<f32>,
        inc: Cell<f32>,
    },
    TexturedSquare {
        shader: Shader,
        vao: u32,
        vbo: u32,
        ibo: u32,
        texture: Texture,
    },
}

impl Sample {
    pub unsafe fn create(&self, gl: &gl::Gl) -> SampleProps {
        gl_call!(gl, BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA));

        match self {
            Self::SimpleTriangle => simple_triangle::create_sample(gl),
            Self::SimpleSquare => simple_square::create_sample(gl),
            Self::TexturedSquare => textured_square::create_sample(gl),
        }
    }
}

impl SampleProps {
    pub fn draw(&self, gl: &gl::Gl) {
        self.bind(gl);

        match self {
            Self::SimpleTriangle { .. } => {
                gl_call!(gl, ClearColor(0.1, 0.1, 0.1, 0.9));
                gl_call!(gl, Clear(gl::COLOR_BUFFER_BIT));

                gl_call!(gl, DrawArrays(gl::TRIANGLES, 0, 3));
            }

            Self::SimpleSquare { .. } => {
                gl_call!(gl, ClearColor(0.1, 0.1, 0.1, 0.9));
                gl_call!(gl, Clear(gl::COLOR_BUFFER_BIT));

                gl_call!(gl, DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, null()));
            }

            Self::TexturedSquare { .. } => {
                gl_call!(gl, ClearColor(0.1, 0.1, 0.1, 0.9));
                gl_call!(gl, Clear(gl::COLOR_BUFFER_BIT));

                gl_call!(gl, DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, null()));
            }
        }

        self.unbind(gl);
    }

    fn bind(&self, gl: &gl::Gl) {
        match self {
            Self::SimpleTriangle { shader, vao, .. } => {
                shader.bind(gl);

                gl_call!(gl, BindVertexArray(*vao));
            }
            Self::SimpleSquare {
                shader, vao, ibo, ..
            } => {
                shader.bind(gl);

                gl_call!(gl, BindVertexArray(*vao));
                gl_call!(gl, BindBuffer(gl::ELEMENT_ARRAY_BUFFER, *ibo));
            }
            Self::TexturedSquare {
                shader,
                vao,
                ibo,
                texture,
                ..
            } => {
                shader.bind(gl);

                let slot = 0;
                texture.bind(gl, slot);
                shader.set_uniform_1i(gl, "u_texture\0", slot);

                gl_call!(gl, BindVertexArray(*vao));
                gl_call!(gl, BindBuffer(gl::ELEMENT_ARRAY_BUFFER, *ibo));
            }
        }
    }

    fn unbind(&self, gl: &gl::Gl) {
        match self {
            Self::SimpleTriangle { shader, .. } => {
                shader.unbind(gl);

                gl_call!(gl, BindVertexArray(0));
            }
            Self::SimpleSquare { shader, .. } => {
                shader.unbind(gl);

                gl_call!(gl, BindVertexArray(0));
                gl_call!(gl, BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0));
            }
            Self::TexturedSquare {
                shader, texture, ..
            } => {
                shader.unbind(gl);
                texture.unbind(gl);

                gl_call!(gl, BindVertexArray(0));
                gl_call!(gl, BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0));
            }
        }
    }

    pub fn snapshot(&self, gl: &gl::Gl) -> bool {
        match self {
            Self::SimpleSquare { shader, r, inc, .. } => {
                let rc = r.get();
                shader.set_uniform_4f(gl, "u_color\0", rc, 0.2, 0.75, 0.9);

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
            Self::SimpleTriangle { shader, vao, vbo } => {
                shader.drop(gl);

                gl.DeleteBuffers(1, vbo);
                gl.DeleteVertexArrays(1, vao);
            }

            Self::SimpleSquare {
                shader,
                vao,
                vbo,
                ibo,
                ..
            } => {
                shader.drop(gl);

                gl.DeleteBuffers(1, vbo);
                gl.DeleteBuffers(1, ibo);
                gl.DeleteVertexArrays(1, vao);
            }
            Self::TexturedSquare {
                shader,
                vao,
                vbo,
                ibo,
                texture,
                ..
            } => {
                shader.drop(gl);
                texture.drop(gl);

                gl.DeleteBuffers(1, vbo);
                gl.DeleteBuffers(1, ibo);
                gl.DeleteVertexArrays(1, vao);
            }
        }
    }
}
