use gl::util::opengl_version;
use gl::viewport::resize_viewport;
use gl::{Gl, BLEND, ONE_MINUS_SRC_ALPHA, SRC_ALPHA};
use glutin::dpi::PhysicalSize;
use glutin::{Context, GlProfile, PixelFormat, PossiblyCurrent, WindowedContext};

pub mod char;
pub mod display;
pub mod font;
pub mod headed;
pub mod headless;
pub mod ndc;
pub mod paint;

pub const DEFAULT_INNER_WINDOW_WIDTH_PX: f32 = 1920.;
pub const DEFAULT_INNER_WINDOW_HEIGHT_PX: f32 = 1080.;
pub const TARGETED_GL_PROFILE: GlProfile = GlProfile::Core;

pub fn load_and_config_gl(context: &Context<PossiblyCurrent>) -> Gl {
    let gl = Gl::load_with(|ptr| context.get_proc_address(ptr) as *const _);
    configure_gl_blend(&gl);
    gl
}

/// Enables and configures blending for the entire OpenGL instance.  This blending configuration is
/// required to support text rendering.  If we require other blending configurations, this function
/// probably shouldn't set this blending configuration instance-wide here.
///
/// https://learnopengl.com/In-Practice/Text-Rendering
fn configure_gl_blend(gl: &Gl) {
    unsafe {
        gl.Enable(BLEND);
        gl.BlendFunc(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
    }
}

pub fn resize_window(
    gl: &Gl,
    windowed_context: &WindowedContext<PossiblyCurrent>,
    new_size: &PhysicalSize<u32>,
) {
    resize_viewport(gl, new_size.width, new_size.height);
    windowed_context.resize(*new_size);
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LogGlInfo {
    Yes,
    No,
}

/// Print's basic diagnostic information about the given OpenGL context and GL instance.
///
/// If you have a WindowedContext, you can also pass the the pixel format of the underlying frame
/// buffer(s).
pub fn print_gl_info(
    context: &Context<PossiblyCurrent>,
    pixel_format: Option<PixelFormat>,
    gl: &Gl,
) {
    println!("-------------------------------------------------");
    if let Some(pixel_format) = pixel_format {
        println!(
            "Pixel format of the window's GL context: {:?}",
            pixel_format
        );
    }
    println!("OpenGL API in use: {:?}", context.get_api());
    println!("OpenGL version {}", opengl_version(gl));
    println!("-------------------------------------------------");
}
