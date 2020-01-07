use crate::gfx::kgl::bool_from_glint;
use crate::gfx::kgl::info_log::{info_log_for, InfoLogKind};
use std::ffi::CStr;

pub type ShaderId = gl::types::GLuint;

/// A `Shader` is a program that allows manipulation of the OpenGL graphics pipeline via arbitrary
/// GLSL code.  There are several different `kind`s of shaders, each with their own well-defined
/// purpose.
///
/// https://www.khronos.org/opengl/wiki/Shader
#[derive(Clone, Debug)]
pub struct Shader {
    id: ShaderId,
    kind: ShaderKind,
}

impl Shader {
    pub fn id(&self) -> ShaderId {
        self.id
    }

    /// Create `Shader` from a literal `CStr` of the sourcecode.
    pub fn from_source(source: &CStr, kind: ShaderKind) -> Result<Shader, String> {
        let new_shader = Shader {
            id: unsafe { gl::CreateShader(kind.into()) },
            kind,
        };
        unsafe {
            gl::ShaderSource(new_shader.id(), 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(new_shader.id());
        }

        if new_shader.compilation_successful() {
            Ok(new_shader)
        } else {
            Err(info_log_for(InfoLogKind::Shader(new_shader.id()))
                .to_string_lossy()
                .into_owned())
        }
    }

    /// Uses the OpenGL state machine to determine if this shader compiled successfully.
    fn compilation_successful(&self) -> bool {
        let mut compilation_successful: gl::types::GLint = 1;
        unsafe {
            gl::GetShaderiv(
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
            gl::DeleteShader(self.id);
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ShaderKind {
    /// https://www.khronos.org/opengl/wiki/Compute_Shader
    Compute,
    /// Processes a fragment (sort of like a pixel) produced by rasterization into a set of colors
    /// and a single depth value.
    ///
    /// https://www.khronos.org/opengl/wiki/Fragment_Shader
    Fragment,
    /// https://www.khronos.org/opengl/wiki/Geometry_Shader
    Geometry,
    /// https://www.khronos.org/opengl/wiki/Tessellation_Control_Shader
    TessControl,
    /// https://www.khronos.org/opengl/wiki/Tessellation_Evaluation_Shader
    TessEvaluation,
    /// Calculates the `gl_PerVertex` values one-by-one for each vertex.  These are `gl_Postion`,
    /// `gl_PointSize`, and `gl_ClipDistance`.
    ///
    /// https://www.khronos.org/opengl/wiki/Vertex_Shader
    Vertex,
}

impl From<ShaderKind> for gl::types::GLuint {
    fn from(shader_kind: ShaderKind) -> Self {
        match shader_kind {
            ShaderKind::Compute => gl::COMPUTE_SHADER,
            ShaderKind::Fragment => gl::FRAGMENT_SHADER,
            ShaderKind::Geometry => gl::GEOMETRY_SHADER,
            ShaderKind::TessControl => gl::TESS_CONTROL_SHADER,
            ShaderKind::TessEvaluation => gl::TESS_EVALUATION_SHADER,
            ShaderKind::Vertex => gl::VERTEX_SHADER,
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

impl From<ShaderParameter> for gl::types::GLuint {
    fn from(shader_parameter: ShaderParameter) -> Self {
        match shader_parameter {
            ShaderParameter::CompileStatus => gl::COMPILE_STATUS,
            ShaderParameter::DeleteStatus => gl::DELETE_STATUS,
            ShaderParameter::InfoLogLength => gl::INFO_LOG_LENGTH,
            ShaderParameter::ShaderSourceLength => gl::SHADER_SOURCE_LENGTH,
            ShaderParameter::ShaderType => gl::SHADER_TYPE,
        }
    }
}
