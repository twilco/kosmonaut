use gl::util::opengl_version;
use gl::viewport::resize_viewport;
use gl::{Gl, BLEND, ONE_MINUS_SRC_ALPHA, SRC_ALPHA};
use glutin::dpi::PhysicalSize;
use glutin::event_loop::EventLoop;
use glutin::window::{Icon, WindowBuilder};
use glutin::{ContextBuilder, GlProfile, PossiblyCurrent, WindowedContext};
use image::ImageFormat;
use std::io::Cursor;

pub mod char;
pub mod display;
pub mod font;
pub mod ndc;
pub mod paint;

static DEFAULT_INNER_WINDOW_WIDTH_PX: f32 = 1920.;
static DEFAULT_INNER_WINDOW_HEIGHT_PX: f32 = 1080.;

pub fn init_main_window_and_gl(
    inner_width_opt: Option<f32>,
    inner_height_opt: Option<f32>,
) -> (WindowedContext<PossiblyCurrent>, EventLoop<()>, Gl) {
    let el = EventLoop::new();
    // This was an arbitrary choice in size.  We can revisit this later.
    let initial_physical_size = PhysicalSize {
        width: inner_width_opt.unwrap_or(DEFAULT_INNER_WINDOW_WIDTH_PX) as u32,
        height: inner_height_opt.unwrap_or(DEFAULT_INNER_WINDOW_HEIGHT_PX) as u32,
    };
    // TODO: `image` is an extremely heavy dependency to use just for the icon...we could probably
    // reduce compile time if we just DYI'd this bit.
    let icon = image::load(
        Cursor::new(&include_bytes!("../../img/Kosmonaut_Logo_164x164-01.png")[..]),
        ImageFormat::Png,
    )
    .unwrap()
    .to_rgba();
    let icon_dimensions = icon.dimensions();
    let wb = WindowBuilder::new()
        .with_title("Kosmonaut")
        .with_inner_size(initial_physical_size)
        .with_window_icon(Some(
            Icon::from_rgba(icon.to_vec(), icon_dimensions.0, icon_dimensions.1).unwrap(),
        ));
    let windowed_context = ContextBuilder::new()
        .with_gl_profile(GlProfile::Core)
        .build_windowed(wb, &el)
        .unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };
    let gl_context = windowed_context.context();
    let gl = Gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);
    configure_gl_blend(&gl);
    resize_window(&gl, &windowed_context, &initial_physical_size);
    (windowed_context, el, gl)
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

pub fn print_gl_info(windowed_context: &WindowedContext<PossiblyCurrent>, gl: &Gl) {
    println!("-------------------------------------------------");
    println!(
        "Pixel format of the window's GL context: {:?}",
        windowed_context.get_pixel_format()
    );
    println!("OpenGL API in use: {:?}", windowed_context.get_api());
    println!("OpenGL version {}", opengl_version(gl));
    println!("-------------------------------------------------");
}
