use crate::types::GLint;
use crate::Gl;

pub unsafe fn resize_viewport(gl: &Gl, new_width: u32, new_height: u32) {
    gl.Viewport(0, 0, new_width as GLint, new_height as GLint);
}
