use crate::{
    gl_call,
    renderer::gl::{self, types::GLuint},
};
use image::EncodableLayout;
use std::path::PathBuf;
use winit::dpi::Pixel;

pub struct Texture {
    id: GLuint,
    width: u32,
    height: u32,
}

impl Texture {
    pub fn load(gl: &gl::Gl, name: &str) -> Self {
        let path = PathBuf::default().join("res/textures").join(name);
        let img = image::open(path)
            .expect(&format!("Can not open image: {}", name))
            .flipv()
            .to_rgba8();
        let (width, height) = img.dimensions();
        let bytes = img.as_bytes();

        let mut id = 0;
        gl_call!(gl, GenTextures(1, &mut id));
        gl_call!(gl, BindTexture(gl::TEXTURE_2D, id));

        gl_call!(
            gl,
            TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR.cast())
        );
        gl_call!(
            gl,
            TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR.cast())
        );
        gl_call!(
            gl,
            TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE.cast())
        );
        gl_call!(
            gl,
            TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE.cast())
        );

        gl_call!(
            gl,
            TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA8.cast(),
                width.cast(),
                height.cast(),
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                bytes.as_ptr() as *const _
            )
        );

        gl_call!(gl, BindTexture(gl::TEXTURE_2D, 0));

        Self { id, width, height }
    }

    pub fn bind(&self, gl: &gl::Gl, slot: i32) {
        if slot >= 32 || slot < 0 {
            panic!("Texture slot only support 0-31.");
        }
        gl_call!(gl, ActiveTexture(gl::TEXTURE0 + slot as u32));
        gl_call!(gl, BindTexture(gl::TEXTURE_2D, self.id));
    }

    pub fn unbind(&self, gl: &gl::Gl) {
        gl_call!(gl, BindTexture(gl::TEXTURE_2D, 0));
    }

    pub fn drop(&self, gl: &gl::Gl) {
        unsafe { gl.DeleteTextures(1, &self.id) }
    }

    pub fn id(&self) -> GLuint {
        self.id
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
