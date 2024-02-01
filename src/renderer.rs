use crate::{gl_call, samples::{Sample, SampleProps}};
use glutin::display::GlDisplay;
use std::ffi::{CStr, CString};

pub mod gl {
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));

    pub use Gles2 as Gl;
}

pub struct Renderer {
    gl: gl::Gl,
    sample: SampleProps,
}

impl Renderer {
    pub fn new<T: GlDisplay>(display: &T, sample: Sample) -> Self {
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

        let sample = unsafe { sample.create(&gl) };

        Self { gl, sample }
    }

    pub fn draw(&self) {
        unsafe {
            self.sample.draw(&self.gl);
        }
    }

    pub fn snapshot(&self) -> bool {
        unsafe {
            self.sample.snapshot(&self.gl)
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
            self.sample.drop(&self.gl);
        }
    }
}

pub fn get_gl_string(gl: &gl::Gl, variant: gl::types::GLenum) -> Option<&'static CStr> {
    unsafe {
        let s = gl_call!(gl, GetString(variant));
        (!s.is_null()).then(|| CStr::from_ptr(s.cast()))
    }
}
