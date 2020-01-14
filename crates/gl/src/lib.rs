use crate::util::opengl_version;
pub use bindings::*;
use std::fmt::{Debug, Error, Formatter};
use std::ops::Deref;
use std::rc::Rc;

#[allow(clippy::too_many_arguments, clippy::unused_unit, clippy::unreadable_literal)]
mod bindings;
pub mod info_log;
pub mod program;
pub mod shader;
pub mod util;

// This crate generates `gl_bindings.rs` in the `target` folder.  I have copy-pasted this file
// to `bindings.rs` in the local `src` directory to get autocomplete working, since my editor
// does not currently source autocomplete via `/target` generated files.
// The below bindings module and `include!` is normally all you would need.
// TODO: Automate the copy-paste of this file in a way that is conducive to autocomplete, or setup editor to source generated file.

// mod bindings {
// include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
// }

#[derive(Clone)]
pub struct Gl {
    inner: Rc<bindings::Gl>,
}

impl Gl {
    pub fn load_with<F>(load_fn: F) -> Self
    where
        F: FnMut(&'static str) -> *const types::GLvoid,
    {
        Gl {
            inner: Rc::new(bindings::Gl::load_with(load_fn)),
        }
    }
}

impl Deref for Gl {
    type Target = bindings::Gl;

    fn deref(&self) -> &bindings::Gl {
        &self.inner
    }
}

impl Debug for Gl {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "Gl: strong_count: {:?}, version: {:?}",
            Rc::strong_count(&self.inner),
            opengl_version(self)
        )
    }
}
