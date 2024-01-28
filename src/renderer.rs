use ::gl::types::{GLsizei, GLsizeiptr};
use glutin::display::GlDisplay;
use std::ffi::{CStr, CString};
use winit::dpi::Pixel;

use crate::shaders::*;

pub mod gl {
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));

    pub use Gles2 as Gl;
}

pub struct Renderer {
    program: gl::types::GLuint,
    gl: gl::Gl,
    vao: u32,
    vbo: u32,
}

impl Renderer {
    pub fn new<T: GlDisplay>(display: &T) -> Self {
        unsafe {
            let gl = gl::Gl::load_with(|symbol| {
                let symbol = CString::new(symbol).unwrap();
                display.get_proc_address(symbol.as_c_str()).cast()
            });

            if let Some(renderer) = get_gl_string(&gl, gl::RENDERER) {
                println!("Running on {}", renderer.to_string_lossy());
            }
            if let Some(version) = get_gl_string(&gl, gl::VERSION) {
                println!("OpenGL Version {}", version.to_string_lossy());
            }

            if let Some(shaders_version) = get_gl_string(&gl, gl::SHADING_LANGUAGE_VERSION) {
                println!("Shaders version on {}", shaders_version.to_string_lossy());
            }
            // Create shaders:
            let program = gl.CreateProgram();
            let vertex_shader = create_shader(&gl, gl::VERTEX_SHADER, SIMPLE_VERTEX_SHADER_SOURCE);
            let fragment_shader =
                create_shader(&gl, gl::FRAGMENT_SHADER, SIMPLE_FRAGMENT_SHADER_SOURCE);
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
                (SIMPLE_VERTEX_DATA.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
                SIMPLE_VERTEX_DATA.as_ptr() as *const _,
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

            Self {
                program,
                gl,
                vao,
                vbo,
            }
        }
    }

    pub fn draw(&self) {
        unsafe {
            self.gl.UseProgram(self.program);

            self.gl.BindVertexArray(self.vao);
            self.gl.BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            self.gl.ClearColor(0.1, 0.1, 0.1, 0.9);
            self.gl.Clear(gl::COLOR_BUFFER_BIT);

            self.gl.DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }

    pub fn resize(&self, width: i32, height: i32) {
        unsafe {
            self.gl.Viewport(0, 0, width, height);
        }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteProgram(self.program);
            self.gl.DeleteBuffers(1, &self.vbo);
            self.gl.DeleteVertexArrays(1, &self.vao);
        }
    }
}

unsafe fn create_shader(
    gl: &gl::Gl,
    shader: gl::types::GLenum,
    source: &[u8],
) -> gl::types::GLuint {
    let shader = gl.CreateShader(shader);
    gl.ShaderSource(
        shader,
        1,
        [source.as_ptr().cast()].as_ptr(),
        std::ptr::null(),
    );
    gl.CompileShader(shader);

    let mut result = std::mem::zeroed();
    gl.GetShaderiv(shader, gl::COMPILE_STATUS, &mut result);
    if result == gl::FALSE.cast() {
        let mut length = std::mem::zeroed();
        gl.GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut length);

        let mut message = vec![0.cast(); length as usize];
        gl.GetShaderInfoLog(shader, length.cast(), &mut length, message.as_mut_ptr());
        let string_result = String::from_utf8(message.iter().map(|&x| x as u8).collect()).unwrap();
        println!("Shader compile error: {}", string_result);
    }

    shader
}

fn get_gl_string(gl: &gl::Gl, variant: gl::types::GLenum) -> Option<&'static CStr> {
    unsafe {
        let s = gl.GetString(variant);
        (!s.is_null()).then(|| CStr::from_ptr(s.cast()))
    }
}

#[rustfmt::skip]
const SIMPLE_VERTEX_DATA: [f32; 6] = [
    -0.5, -0.5,
    0.0, 0.5,
    0.5, -0.5,
];
