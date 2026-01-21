use crate::buffer::renderbuffer::RenderBuffer;
use crate::buffer::Buffer;
use crate::types::{GLenum, GLuint};
use crate::{
    Gl, COLOR_ATTACHMENT0, FRAMEBUFFER, FRAMEBUFFER_COMPLETE, FRAMEBUFFER_INCOMPLETE_ATTACHMENT,
    FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER, FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS,
    FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT, FRAMEBUFFER_INCOMPLETE_MULTISAMPLE,
    FRAMEBUFFER_INCOMPLETE_READ_BUFFER, FRAMEBUFFER_UNDEFINED, FRAMEBUFFER_UNSUPPORTED,
    RENDERBUFFER,
};

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

/// Possible framebuffer statuses.  See possible values here:
/// https://docs.gl/gl3/glCheckFramebufferStatus
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrameBufferStatus {
    Complete,
    Undefined,
    IncompleteAttachment,
    IncompleteMissingAttachment,
    IncompleteDrawBuffer,
    IncompleteReadBuffer,
    Unsupported,
    IncompleteMultiSample,
    IncompleteLayerTargets,
}

impl From<GLenum> for FrameBufferStatus {
    fn from(enum_val: GLenum) -> Self {
        match enum_val {
            FRAMEBUFFER_COMPLETE => FrameBufferStatus::Complete,
            FRAMEBUFFER_UNDEFINED => FrameBufferStatus::Undefined,
            FRAMEBUFFER_INCOMPLETE_ATTACHMENT => FrameBufferStatus::IncompleteAttachment,
            FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => {
                FrameBufferStatus::IncompleteMissingAttachment
            }
            FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER => FrameBufferStatus::IncompleteDrawBuffer,
            FRAMEBUFFER_INCOMPLETE_READ_BUFFER => FrameBufferStatus::IncompleteDrawBuffer,
            FRAMEBUFFER_UNSUPPORTED => FrameBufferStatus::Unsupported,
            FRAMEBUFFER_INCOMPLETE_MULTISAMPLE => FrameBufferStatus::IncompleteMultiSample,
            FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS => FrameBufferStatus::IncompleteLayerTargets,
            _ => panic!("unexpected framebuffer status of value: {}", enum_val),
        }
    }
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

/// Gets the status of the framebuffer attached to the given `gl` handle.
///
/// https://docs.gl/gl3/glCheckFramebufferStatus
pub fn globally_attached_framebuffer_status(gl: &Gl) -> FrameBufferStatus {
    unsafe { gl.CheckFramebufferStatus(FRAMEBUFFER).into() }
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
        // First make sure our associated render buffer is bound to RENDERBUFFER.
        self.render_buffer.bind_to(&self.gl);
        unsafe {

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
