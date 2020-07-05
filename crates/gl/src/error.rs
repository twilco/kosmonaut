use crate::{Gl, NO_ERROR, INVALID_ENUM, INVALID_VALUE, INVALID_OPERATION, INVALID_FRAMEBUFFER_OPERATION, OUT_OF_MEMORY};
use crate::bindings::types::GLenum;

/// Returns all errors that have been recorded since the last call to this function.  The act of
/// getting these flags clears them from the internal OpenGL state machine, and thus they will not
/// be returned again.
pub fn get_error_flags(gl: &Gl) -> Vec<ErrorFlag> {
    let mut flags = Vec::new();
    unsafe {
        loop {
            let flag = ErrorFlag::from(gl.GetError());
            if flag == ErrorFlag::NoError {
                break;
            }
            flags.push(flag);
        }
    }
    flags
}

impl From<GLenum> for ErrorFlag {
    fn from(enum_val: u32) -> Self {
        match enum_val {
            NO_ERROR => ErrorFlag::NoError,
            INVALID_ENUM => ErrorFlag::InvalidEnum,
            INVALID_VALUE => ErrorFlag::InvalidValue,
            INVALID_OPERATION => ErrorFlag::InvalidOperation,
            INVALID_FRAMEBUFFER_OPERATION => ErrorFlag::InvalidFramebufferOperation,
            OUT_OF_MEMORY => ErrorFlag::OutOfMemory,
            _ => panic!("unexpected opengl error of value: {}", enum_val)
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ErrorFlag {
    /// No error has been recorded.
    NoError,
    /// An unacceptable value is specified for an enumerated argument. The offending command is
    /// ignored and has no other side effect than to set the error flag.
    InvalidEnum,
    /// A numeric argument is out of range. The offending command is ignored and has no other side
    /// effect than to set the error flag.
    InvalidValue,
    /// The specified operation is not allowed in the current state. The offending command is
    /// ignored and has no other side effect than to set the error flag.
    InvalidOperation,
    /// The framebuffer object is not complete. The offending command is ignored and has no other
    /// side effect than to set the error flag.
    InvalidFramebufferOperation,
    /// There is not enough memory left to execute the command. The state of the GL is undefined,
    /// except for the state of the error flags, after this error is recorded.
    OutOfMemory,
    /// An attempt has been made to perform an operation that would cause an internal stack to
    /// underflow.
    StackUnderflow,
    ///  An attempt has been made to perform an operation that would cause an internal stack to
    /// overflow.
    StackOverflow
}