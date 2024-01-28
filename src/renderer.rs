use glutin::display::GlDisplay;
use std::ffi::CString;

pub mod gl {
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));

    pub use Gles2 as Gl;
}

pub struct Renderer {
    gl: gl::Gl,
}

impl Renderer {
    pub fn new<T: GlDisplay>(display: &T) -> Self {
        let gl = gl::Gl::load_with(|symbol| {
            let symbol = CString::new(symbol).unwrap();
            display.get_proc_address(symbol.as_c_str()).cast()
        });

        Self { gl }
    }
}
