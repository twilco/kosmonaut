use crate::buffer::Buffer;
use crate::types::{GLenum, GLsizei, GLuint};
use crate::{Gl, RENDERBUFFER, RGB8, RGBA};
use std::cell::RefCell;

pub type RenderBufferId = GLuint;

/// A `RenderBuffer` is an OpenGL object that contains an image, typically for use with a
/// FrameBuffer object.
///
/// Storage allocation for `RenderBuffer`s happens lazily, specifically after the first time it is
/// bound.
///
/// https://www.khronos.org/opengl/wiki/Renderbuffer_Object
#[derive(Clone, Debug)]
pub struct RenderBuffer {
    gl: Gl,
    /// Height in pixels.  Used to inform how much storage this `RenderBuffer` allocates.
    height_px: GLsizei,
    /// The ID (or name, as OpenGL calls it) of this `RenderBuffer`.
    id: RenderBufferId,
    /// The internal pixel format of this `RenderBuffer`.
    internal_format: RenderBufferFormat,
    /// Whether or not this `RenderBuffer` has allocated its storage using its width, height, and
    /// pixel format.
    storage_allocated: RefCell<bool>,
    /// Width in pixels.  Used to inform how much storage this `RenderBuffer` allocates.
    width_px: GLsizei,
}

/// Allowed pixel formats for a `RenderBuffer`.  There are more allowed than are currently present
/// here -- see docs for the rules around what formats will work:
///
/// https://www.khronos.org/opengl/wiki/GLAPI/glRenderbufferStorage
#[derive(Clone, Debug)]
pub enum RenderBufferFormat {
    RGB8,
    RGBA,
}

impl RenderBufferFormat {
    pub fn to_gl_enum(&self) -> GLenum {
        match self {
            RenderBufferFormat::RGB8 => RGB8,
            RenderBufferFormat::RGBA => RGBA,
        }
    }
}

impl RenderBuffer {
    pub fn new(gl: &Gl, width_px: GLsizei, height_px: GLsizei) -> RenderBuffer {
        let mut id: GLuint = 0;
        unsafe {
            gl.GenRenderbuffers(1, &mut id);
        }

        RenderBuffer {
            gl: gl.clone(),
            height_px,
            id,
            internal_format: RenderBufferFormat::RGBA,
            storage_allocated: RefCell::new(false),
            width_px,
        }
    }

    pub fn id(&self) -> RenderBufferId {
        self.id
    }
}

impl Drop for RenderBuffer {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteRenderbuffers(1, &self.id) }
    }
}

impl Buffer for RenderBuffer {
    fn bind_to(&self, gl: &Gl) {
        unsafe {
            gl.BindRenderbuffer(RENDERBUFFER, self.id);
        }
        if !*self.storage_allocated.borrow() {
            unsafe {
                gl.RenderbufferStorage(
                    RENDERBUFFER,
                    self.internal_format.to_gl_enum(),
                    self.width_px as _,
                    self.height_px as _,
                );
            }
            self.storage_allocated.replace(true);
        }
    }
}

pub fn unbind_renderbuffer_globally(gl: &Gl) {
    unsafe {
        gl.BindRenderbuffer(RENDERBUFFER, 0);
    }
}
