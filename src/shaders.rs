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
}

pub struct ShaderSource;
impl ShaderSource {
    pub fn load<T: ToString>(name: T, ty: ShaderType) -> Vec<u8> {
        let mut name = name.to_string();
        name.push_str(ty.suffix());

        let path = PathBuf::default().join("res/shaders").join(name);
        let os_string = path.as_os_str().to_os_string();
        let path_str = os_string.to_string_lossy();

        let mut file = File::open(path).expect(&format!("Open file {} failed.", path_str));
        let mut buf = vec![];
        file.read_to_end(&mut buf)
            .expect(&format!("Read file {} failed.", path_str));
        buf.push(0);
        buf
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
