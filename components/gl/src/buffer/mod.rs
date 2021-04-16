use crate::{Gl, ARRAY_BUFFER};

pub mod framebuffer;
pub mod renderbuffer;
pub mod vbo;

/// Trait to represent behavior of OpenGL buffers (like a vertex buffer object, VBO).
pub trait Buffer {
    /// Binds this buffer to the specified OpenGL context.
    fn bind_to(&self, gl: &Gl);
}

/// Unbinds whatever buffer is currently bound to the specified OpenGL context.
///
/// https://khronos.org/registry/OpenGL-Refpages/gl4/
pub(super) fn unbind_array_buffer_globally(gl: &Gl) {
    unsafe { gl.BindBuffer(ARRAY_BUFFER, 0) }
}
