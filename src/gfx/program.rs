use crate::gfx::kgl::bool_from_glint;
use crate::gfx::kgl::info_log::{info_log_for, InfoLogKind};
use crate::gfx::shader::Shader;

pub type ProgramId = gl::types::GLuint;

/// A `Program` is the culmination of multiple shaders that can be used to render objects.  Programs
/// link the output of one shader to the next, making one cohesive pipeline.
///
/// https://www.khronos.org/opengl/wiki/GLSL_Object#Program_objects
#[derive(Clone, Debug)]
pub struct Program {
    id: ProgramId,
}

impl Program {
    pub fn id(&self) -> ProgramId {
        self.id
    }

    /// Creates a `Program` from a slice of shaders.
    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        let new_program = Program {
            id: unsafe { gl::CreateProgram() },
        };
        for shader in shaders {
            unsafe { gl::AttachShader(new_program.id(), shader.id()) }
        }
        unsafe { gl::LinkProgram(new_program.id()) };
        // Once a program has been linked, any shaders that are no longer needed should be detached
        // and deleted.  `Shader#drop` handles shader deletion, but it cannot drop a shader if it
        // is still attached to a program.  Let's detach these shaders.
        // https://gamedev.stackexchange.com/questions/47910/after-a-succesful-gllinkprogram-should-i-delete-detach-my-shaders
        for shader in shaders {
            unsafe { gl::DetachShader(new_program.id(), shader.id()) }
        }

        if !new_program.link_successful() {
            let error_msg = info_log_for(InfoLogKind::Program(new_program.id()));
            return Err(error_msg.to_string_lossy().into_owned());
        }
        Ok(new_program)
    }

    /// Determines whether or not the most link attempt was successful.
    pub fn link_successful(&self) -> bool {
        let mut link_successful: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(
                self.id(),
                ProgramParameter::LinkStatus.into(),
                &mut link_successful,
            )
        };
        bool_from_glint(link_successful)
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id()) }
    }
}

/// Possible parameters for an OpenGL program.  Many are currently not present in this enum — add as
/// needed.
///
/// http://docs.gl/gl3/glGetProgram
pub enum ProgramParameter {
    InfoLogLength,
    LinkStatus,
}

impl From<ProgramParameter> for gl::types::GLuint {
    fn from(program_parameter: ProgramParameter) -> Self {
        match program_parameter {
            ProgramParameter::InfoLogLength => gl::INFO_LOG_LENGTH,
            ProgramParameter::LinkStatus => gl::LINK_STATUS,
        }
    }
}
