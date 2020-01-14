use crate::info_log::{info_log_for, InfoLogKind};
use crate::util::bool_from_glint;
use crate::{
    types, Gl, COMPILE_STATUS, DELETE_STATUS, FRAGMENT_SHADER, GEOMETRY_SHADER, INFO_LOG_LENGTH,
    SHADER_SOURCE_LENGTH, SHADER_TYPE, VERTEX_SHADER,
};
use std::ffi::CStr;

pub type ShaderId = types::GLuint;

/// A `Shader` is a program that allows manipulation of the OpenGL graphics pipeline via arbitrary
/// GLSL code.  There are several different `kind`s of shaders, each with their own well-defined
/// purpose.
///
/// https://www.khronos.org/opengl/wiki/Shader
#[derive(Clone, Debug)]
pub struct Shader {
    id: ShaderId,
    kind: ShaderKind,
    gl: Gl,
}

impl Shader {
    pub fn id(&self) -> ShaderId {
        self.id
    }

    /// Create `Shader` from a literal `CStr` of the sourcecode.
    pub fn from_source(source: &CStr, kind: ShaderKind, gl: &Gl) -> Result<Shader, String> {
        let new_shader = Shader {
            id: unsafe { gl.CreateShader(kind.into()) },
            kind,
            gl: gl.clone(),
        };
        unsafe {
            gl.ShaderSource(new_shader.id(), 1, &source.as_ptr(), std::ptr::null());
            gl.CompileShader(new_shader.id());
        }

        if new_shader.compilation_successful() {
            Ok(new_shader)
        } else {
            Err(info_log_for(InfoLogKind::Shader(new_shader.id()), &gl)
                .to_string_lossy()
                .into_owned())
        }
    }

    /// Uses the OpenGL state machine to determine if this shader compiled successfully.
    fn compilation_successful(&self) -> bool {
        let mut compilation_successful: types::GLint = 1;
        unsafe {
            self.gl.GetShaderiv(
                self.id(),
                ShaderParameter::CompileStatus.into(),
                &mut compilation_successful,
            );
        }
        bool_from_glint(compilation_successful)
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteShader(self.id);
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ShaderKind {
    /// Processes a fragment (sort of like a pixel) produced by rasterization into a set of colors
    /// and a single depth value.
    ///
    /// https://www.khronos.org/opengl/wiki/Fragment_Shader
    Fragment,
    /// https://www.khronos.org/opengl/wiki/Geometry_Shader
    Geometry,
    /// Calculates the `gl_PerVertex` values one-by-one for each vertex.  These are `gl_Postion`,
    /// `gl_PointSize`, and `gl_ClipDistance`.
    ///
    /// https://www.khronos.org/opengl/wiki/Vertex_Shader
    Vertex,
}

impl From<ShaderKind> for types::GLuint {
    fn from(shader_kind: ShaderKind) -> Self {
        match shader_kind {
            ShaderKind::Fragment => FRAGMENT_SHADER,
            ShaderKind::Geometry => GEOMETRY_SHADER,
            ShaderKind::Vertex => VERTEX_SHADER,
        }
    }
}

/// http://docs.gl/gl3/glGetShader
pub enum ShaderParameter {
    CompileStatus,
    DeleteStatus,
    InfoLogLength,
    ShaderSourceLength,
    ShaderType,
}

impl From<ShaderParameter> for types::GLuint {
    fn from(shader_parameter: ShaderParameter) -> Self {
        match shader_parameter {
            ShaderParameter::CompileStatus => COMPILE_STATUS,
            ShaderParameter::DeleteStatus => DELETE_STATUS,
            ShaderParameter::InfoLogLength => INFO_LOG_LENGTH,
            ShaderParameter::ShaderSourceLength => SHADER_SOURCE_LENGTH,
            ShaderParameter::ShaderType => SHADER_TYPE,
        }
    }
}
