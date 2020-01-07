/// This module is named `kgl` to avoid ambiguity with the `gl` crate.  This module will contain
/// Kosmonaut-specific gl/OpenGL customizations, wrappers, and more.
use glutin::{PossiblyCurrent, WindowedContext};
use std::ffi::{CStr, CString};

pub mod info_log;

pub fn print_gl_info(windowed_context: &WindowedContext<PossiblyCurrent>) {
    println!("-------------------------------------------------");
    println!(
        "Pixel format of the window's GL context: {:?}",
        windowed_context.get_pixel_format()
    );
    println!("OpenGL API in use: {:?}", windowed_context.get_api());
    println!("OpenGL version {}", opengl_version());
    println!("-------------------------------------------------");
}

fn opengl_version() -> String {
    unsafe {
        let data = CStr::from_ptr(gl::GetString(gl::VERSION) as *const _)
            .to_bytes()
            .to_vec();
        String::from_utf8(data).unwrap()
    }
}

pub fn create_whitespace_cstring(len: usize) -> CString {
    // `len` + 1 to give space for the nul-terminate byte at the end
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}

pub fn bool_from_glint(glint: gl::types::GLint) -> bool {
    match glint {
        0 => false,
        1 => true,
        _ => panic!("could not convert GLint to bool: {}", glint),
    }
}
