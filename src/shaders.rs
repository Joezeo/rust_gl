use crate::renderer::gl;
use ::gl::types::{GLchar, GLenum};
use gl::types::GLuint;
use nalgebra::Matrix4;
use std::{fs::File, io::Read, path::PathBuf};
use winit::dpi::Pixel;

use crate::gl_call;

pub enum ShaderType {
    Vertex,
    Fragment,
}
impl ShaderType {
    fn suffix(&self) -> &'static str {
        match self {
            Self::Vertex => ".vs",
            Self::Fragment => ".fs",
        }
    }

    fn gl_type(&self) -> GLenum {
        match self {
            Self::Vertex => gl::VERTEX_SHADER,
            Self::Fragment => gl::FRAGMENT_SHADER,
        }
    }

    fn create_shader<T: ToString>(&self, gl: &gl::Gl, name: T) -> GLuint {
        let mut name = name.to_string();
        name.push_str(self.suffix());

        let path = PathBuf::default().join("res/shaders").join(name);
        let os_string = path.as_os_str().to_os_string();
        let path_str = os_string.to_string_lossy();

        let mut file = File::open(path).expect(&format!("Open file {} failed.", path_str));
        let mut buf = vec![];
        file.read_to_end(&mut buf)
            .expect(&format!("Read file {} failed.", path_str));
        buf.push(0);

        unsafe { create_shader(gl, self.gl_type(), &buf) }
    }
}

pub struct Shader {
    program: GLuint,
}
impl Shader {
    pub fn load<T: ToString>(gl: &gl::Gl, vertex_name: T, fragment_name: T) -> Self {
        let vertex_shader = ShaderType::Vertex.create_shader(gl, vertex_name);
        let fragment_shader = ShaderType::Fragment.create_shader(gl, fragment_name);

        let program = gl_call!(gl, CreateProgram());
        gl_call!(gl, AttachShader(program, vertex_shader));
        gl_call!(gl, AttachShader(program, fragment_shader));

        gl_call!(gl, LinkProgram(program));
        gl_call!(gl, UseProgram(program));

        gl_call!(gl, DeleteShader(vertex_shader));
        gl_call!(gl, DeleteShader(fragment_shader));

        Self { program }
    }

    pub fn set_uniform_1i(&self, gl: &gl::Gl, name: &str, v0: i32) {
        self.bind(gl);

        gl_call!(gl, Uniform1i(self.location(gl, name), v0));
    }

    pub fn set_uniform_4f(&self, gl: &gl::Gl, name: &str, v0: f32, v1: f32, v2: f32, v3: f32) {
        self.bind(gl);

        gl_call!(gl, Uniform4f(self.location(gl, name), v0, v1, v2, v3));
    }

    pub fn set_uniform_mat_4f(&self, gl: &gl::Gl, name: &str, mat: &Matrix4<f32>) {
        self.bind(gl);

        gl_call!(gl, UniformMatrix4fv(self.location(gl, name), 1, gl::FALSE, mat.as_ptr()))
    }

    pub fn bind(&self, gl: &gl::Gl) {
        gl_call!(gl, UseProgram(self.program));
    }

    pub fn unbind(&self, gl: &gl::Gl) {
        gl_call!(gl, UseProgram(0));
    }

    pub fn drop(&self, gl: &gl::Gl) {
        unsafe {
            gl.DeleteProgram(self.program);
        }
    }

    pub fn program(&self) -> GLuint {
        self.program
    }

    unsafe fn location(&self, gl: &gl::Gl, name: &str) -> i32 {
        let location = gl_call!(gl, GetUniformLocation(self.program, as_gl_char_ptr(name)));
        assert!(location != -1);
        location
    }
}

pub unsafe fn create_shader(
    gl: &crate::renderer::gl::Gl,
    shader: gl::types::GLenum,
    source: &[u8],
) -> gl::types::GLuint {
    let shader = gl_call!(gl, CreateShader(shader));
    gl_call!(
        gl,
        ShaderSource(
            shader,
            1,
            [source.as_ptr().cast()].as_ptr(),
            std::ptr::null(),
        )
    );
    gl_call!(gl, CompileShader(shader));

    let mut result = std::mem::zeroed();
    gl_call!(gl, GetShaderiv(shader, gl::COMPILE_STATUS, &mut result));
    if result == gl::FALSE.cast() {
        let mut length = std::mem::zeroed();
        gl_call!(gl, GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut length));

        let mut message = vec![0.cast(); length as usize];
        gl_call!(
            gl,
            GetShaderInfoLog(shader, length.cast(), &mut length, message.as_mut_ptr())
        );
        let string_result = String::from_utf8(message.iter().map(|&x| x as u8).collect()).unwrap();
        println!("Shader compile error: {}", string_result);
    }

    shader
}

/// #### The string that interacts with OpenGl Api needs to end with '\0'
#[inline]
pub fn as_gl_char_ptr(str: &str) -> *const GLchar {
    str.as_bytes() as *const [u8] as *const u8 as *const GLchar
}
