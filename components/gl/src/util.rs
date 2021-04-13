use crate::types::GLint;
use crate::{Gl, VERSION};
use std::ffi::{CStr, CString};

pub fn bool_from_glint(glint: GLint) -> bool {
    match glint {
        0 => false,
        1 => true,
        _ => panic!("could not convert GLint to bool: {}", glint),
    }
}

pub fn opengl_version(gl: &Gl) -> String {
    unsafe {
        let data = CStr::from_ptr(gl.GetString(VERSION) as *const _)
            .to_bytes()
            .to_vec();
        String::from_utf8(data).unwrap()
    }
}

pub(crate) fn create_whitespace_cstring(len: usize) -> CString {
    // `len` + 1 to give space for the nul-terminate byte at the end
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}
