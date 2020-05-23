use crate::resize_window;
use gl::util::opengl_version;
use gl::Gl;
use glutin::dpi::PhysicalSize;
use glutin::event_loop::EventLoop;
use glutin::window::WindowBuilder;
use glutin::{ContextBuilder, GlProfile, PossiblyCurrent, WindowedContext};

pub mod display;
pub mod ndc;
pub mod paint;

pub fn init_main_window_and_gl() -> (WindowedContext<PossiblyCurrent>, EventLoop<()>, Gl) {
    let el = EventLoop::new();
    // This was an arbitrary choice in size.  We can revisit this later.
    let initial_physical_size = PhysicalSize {
        width: 1920,
        height: 1080,
    };
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
