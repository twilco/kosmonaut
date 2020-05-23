use crate::Gl;
use crate::types::GLint;

pub fn resize_viewport(gl: &Gl, new_width: u32, new_height: u32) {
    unsafe {
        gl.Viewport(0, 0, new_width as GLint, new_height as GLint);
    }
}
