use gl_generator::{Api, DebugStructGenerator, Fallbacks, Profile, Registry, StructGenerator};
use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").expect("should've been able to get OUT_DIR env var");
    let mut gl_bindings_file = File::create(&Path::new(&out_dir).join("gl_bindings.rs"))
        .expect("failed to create gl_bindings.rs file");

    let registry = Registry::new(
        Api::Gl,
        (3, 3),
        Profile::Core,
        Fallbacks::All,
        ["GL_NV_command_list"],
    );

    if env::var("CARGO_FEATURE_DEBUG").is_ok() {
        registry
            .write_bindings(DebugStructGenerator, &mut gl_bindings_file)
            .expect("failed to write gl_bindings");
    } else {
        registry
            .write_bindings(StructGenerator, &mut gl_bindings_file)
            .expect("failed to write gl_bindings");
    }
}
