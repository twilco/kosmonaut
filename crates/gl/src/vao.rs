use crate::bindings::types::GLuint;
use crate::buffer::vbo::VertexBufferObject;
use crate::buffer::{unbind_array_buffer_globally, Buffer};
use crate::Gl;

/// Represents an OpenGL vertex array object (VAO).
///
/// https://www.khronos.org/opengl/wiki/Vertex_Specification
pub struct VertexArrayObject {
    /// The buffer this VAO sources vertex data from.  `None` if no buffer is bound.
    bound_buffer: Option<VertexBufferObject>,
    /// The name of this VAO given by OpenGL upon creation.
    name: GLuint,
    /// The shared OpenGL instance.
    gl: Gl,
}

impl VertexArrayObject {
    /// Returns a new VAO bound to the input buffer.  Take special note of the `config_vao_fn`
    /// parameter.  You must pass this function in to make any calls to `EnableVertexAttribArray`,
    /// `VertexAttribPointer`, etc, as is contextually necessary.  This function does all the work
    /// it can to set you up for your configuration, such as binding and unbinding the new VAO
    /// before and after your config function.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it relies on you implementing `config_vao_fn` correctly to
    /// get proper functionality and safety.  It also uses some GL methods directly, which
    /// themselves require care to use correctly.
    pub unsafe fn new(
        vbo_to_bind: VertexBufferObject,
        config_vao_fn: impl Fn(&Gl),
        gl: &Gl,
    ) -> VertexArrayObject {
        let mut vao_name: GLuint = 0;
        gl.GenVertexArrays(1, &mut vao_name);
        gl.BindVertexArray(vao_name);
        vbo_to_bind.bind_to(gl);

        config_vao_fn(gl);

        unbind_array_buffer_globally(gl);
        gl.BindVertexArray(0);

        VertexArrayObject {
            bound_buffer: Some(vbo_to_bind),
            name: vao_name,
            gl: gl.clone(),
        }
    }

    pub fn name(&self) -> GLuint {
        self.name
    }

    /// Stores the input vertex data in the VBO associated with this VAO.
    ///
    /// If this VAO does not have a VBO bound (`None`), one is created and bound to this VAO, and
    /// the data is added into this new VBO.  Otherwise, if a VBO is already bound, the data is
    /// added to this existing VBO.
    pub fn store_vertex_data(&mut self, data: &[f32]) {
        match self.store_vertex_data_fallible(data) {
            Ok(()) => {}
            Err(err) => match err {
                VaoErr::NoVboBound => {
                    let mut vbo = VertexBufferObject::new(&self.gl);
                    vbo.store_vertex_data(data);
                    self.bound_buffer = Some(vbo);
                }
            },
        }
    }

    /// Stores the input vertex data in the VBO associated with this VAO.
    ///
    /// If this VAO does not have a VBO bound (`None`), an `Err` is returned.
    pub fn store_vertex_data_fallible(&mut self, data: &[f32]) -> Result<(), VaoErr> {
        match &mut self.bound_buffer {
            Some(ref mut vbo) => {
                vbo.store_vertex_data(data);
                Ok(())
            }
            None => Err(VaoErr::NoVboBound),
        }
    }
}

/// Errors related to VAO behavior.
pub enum VaoErr {
    /// Used when the VAO is asked to take an action that requires a bound VBO, but there is `None`.
    NoVboBound,
}

impl Drop for VertexArrayObject {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteVertexArrays(1, self.name as *const GLuint) }
    }
}
