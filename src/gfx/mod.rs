use gl::util::opengl_version;
use gl::viewport::resize_viewport;
use gl::Gl;
use glutin::dpi::PhysicalSize;
use glutin::event_loop::EventLoop;
use glutin::window::WindowBuilder;
use glutin::{ContextBuilder, GlProfile, PossiblyCurrent, WindowedContext};

pub mod display;
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
    // TODO: Add Ksomonaut icon.  See here for Glutin icon usage example:
    // https://github.com/RyanChristian4427/parametric_equations/blob/afa7436f70ec2209154255debe7925f9b1c35347/src/lib.rs
    let wb = WindowBuilder::new()
        .with_title("Kosmonaut")
        .with_inner_size(initial_physical_size);
    let windowed_context = ContextBuilder::new()
        .with_gl_profile(GlProfile::Core)
        .build_windowed(wb, &el)
        .unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };
    let gl_context = windowed_context.context();
    let gl = Gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);
    resize_window(&gl, &windowed_context, &initial_physical_size);
    (windowed_context, el, gl)
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
