use crate::bindings::types::GLuint;
use crate::buffer::{unbind_array_buffer_globally, Buffer};
use crate::types::{GLsizeiptr, GLvoid};
use crate::{Gl, ARRAY_BUFFER, STATIC_DRAW};

/// Represents an OpenGL vertex buffer object (VBO).
///
/// https://www.khronos.org/opengl/wiki/Vertex_Specification
pub struct VertexBufferObject {
    /// The buffer name given by OpenGL for this VBO upon creation.
    name: GLuint,
    /// The OpenGL context this VBO belongs to.
    gl: Gl,
}

impl VertexBufferObject {
    pub fn new(gl: &Gl) -> VertexBufferObject {
        let mut vbo_name: GLuint = 0;
        unsafe {
            gl.GenBuffers(1, &mut vbo_name);
        }

        VertexBufferObject {
            name: vbo_name,
            gl: gl.clone(),
        }
    }

    pub fn name(&self) -> GLuint {
        self.name
    }

    /// Stores new vertex data, overwriting any that might already exist in this VBO.
    pub fn store_vertex_data(&mut self, data: &[f32]) {
        self.bind_to(&self.gl);
        unsafe {
            self.gl.BufferData(
                ARRAY_BUFFER,                                            // target
                (data.len() * std::mem::size_of::<f32>()) as GLsizeiptr, // size of data in bytes
                data.as_ptr() as *const GLvoid,                          // pointer to data
                STATIC_DRAW,                                             // usage
            );
        }
        unbind_array_buffer_globally(&self.gl);
    }
}

impl Buffer for VertexBufferObject {
    fn bind_to(&self, gl: &Gl) {
        unsafe { gl.BindBuffer(ARRAY_BUFFER, self.name()) }
    }
}

impl Drop for VertexBufferObject {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteBuffers(1, &self.name) }
    }
}
