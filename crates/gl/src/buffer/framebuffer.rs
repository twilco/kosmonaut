use crate::buffer::renderbuffer::RenderBuffer;
use crate::buffer::Buffer;
use crate::types::GLuint;
use crate::{Gl, COLOR_ATTACHMENT0, FRAMEBUFFER, RENDERBUFFER};

pub type FrameBufferId = GLuint;

/// A `FrameBuffer` is an OpenGL object that allows creation of user-defined
/// [framebuffers](https://www.khronos.org/opengl/wiki/Framebuffer) (as opposed to using the
/// default framebuffer that is created with the OpenGL context).
///
/// https://www.khronos.org/opengl/wiki/Framebuffer_Object
#[derive(Clone, Debug)]
pub struct FrameBuffer {
    gl: Gl,
    /// The ID (or name, as OpenGL calls it) of this `FrameBuffer`.
    id: FrameBufferId,
    /// The `RenderBuffer` to attach when this `FrameBuffer` is bound.  This is essentially the
    /// underlying storage of this `FrameBuffer`.
    render_buffer: RenderBuffer,
}

impl FrameBuffer {
    pub fn new(render_buffer: RenderBuffer, gl: &Gl) -> FrameBuffer {
        let mut id: GLuint = 0;
        unsafe {
            gl.GenFramebuffers(1, &mut id);
        }

        FrameBuffer {
            gl: gl.clone(),
            id,
            render_buffer,
        }
    }
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteFramebuffers(1, &self.id);
        }
    }
}

impl Buffer for FrameBuffer {
    fn bind_to(&self, gl: &Gl) {
        unsafe {
            // First make sure our associated render buffer is bound to RENDERBUFFER.
            self.render_buffer.bind_to(&self.gl);

            // Now bind ourselves.
            gl.BindFramebuffer(FRAMEBUFFER, self.id);
            gl.FramebufferRenderbuffer(
                FRAMEBUFFER,
                // Read more about what this attachment parameter does:
                // https://learnopengl.com/Advanced-Lighting/Bloom
                // https://stackoverflow.com/questions/34972716/what-does-gl-color-attachment-do
                COLOR_ATTACHMENT0,
                RENDERBUFFER,
                self.render_buffer.id(),
            );
        }
    }
}
