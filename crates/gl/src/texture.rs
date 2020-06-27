use crate::{types, Gl, TEXTURE_2D};
use crate::bindings::types::GLuint;

pub type TextureId = types::GLuint;

/// A `Texture` is an [OpenGL Object](https://www.khronos.org/opengl/wiki/OpenGL_Objects) that
/// contains one or more images that all have the same [image format](https://www.khronos.org/opengl/wiki/Image_Formats).
/// In this context, an "image" is defined as a single array of pixels of a certain dimensionality (1D, 2D, 3D),
/// with a particular size, and a specific image format.
///
/// Textures can be a target of rendering, or used as a resource by shaders.
///
/// https://www.khronos.org/opengl/wiki/Texture
#[derive(Clone, Debug)]
pub struct Texture {
    id: TextureId,
    kind: TextureKind,
    gl: Gl,
}

impl Texture {
    pub fn new(kind: TextureKind, gl: &Gl) -> Texture {
        let mut id: GLuint = 0;
        unsafe {
            gl.GenTextures(1, &mut id);
        }

        Texture {
            id,
            kind,
            gl: gl.clone()
        }
    }

    pub fn id(&self) -> TextureId {
        self.id
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteTextures(1, &self.id)
        }
    }
}

/// OpenGL textures come in different kinds, each of which influence the types, number, and format
/// of images within the texture.
///
/// There are more types of textures than are currently present in this enum.  Add them as needed.
///
/// https://www.khronos.org/opengl/wiki/Texture
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextureKind {
    /// Images in this texture have two dimensions -- width and height (but no depth).
    TwoDimensional
}

impl From<TextureKind> for types::GLuint {
    fn from(texture_kind: TextureKind) -> Self {
        match texture_kind {
            TextureKind::TwoDimensional => TEXTURE_2D,
        }
    }
}
