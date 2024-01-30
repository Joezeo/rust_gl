use crate::renderer::gl;

pub fn gl_clear_error(gl: &gl::Gl) {
    unsafe { while gl.GetError() != gl::NO_ERROR {} }
}

pub fn gl_check_error(gl: &gl::Gl) -> bool {
    let mut r = false;
    loop {
        unsafe {
            let error = gl.GetError();
            if error == gl::NO_ERROR {
                return r;
            }

            r = true;
            println!("OpenGL error: 0x{:04X}", error)
        }
    }
}

#[macro_export]
macro_rules! gl_call {
    ( $gl:ident, $epr:ident($($args:tt)*) ) => {
        {
            let fn_name = concat!(stringify!($epr), "(", concat!($(stringify!($args)),*), ")");

            crate::error::gl_clear_error($gl);

            let res = $gl.$epr($($args)*);

            if crate::error::gl_check_error($gl) {
                println!("failed on function: {}, at file {}({},{})", fn_name, file!(), line!(), column!());
                std::process::exit(0);
            }

            res
        }
    };
}
