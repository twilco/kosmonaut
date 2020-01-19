use crate::gfx::rect::RectPainter;
use crate::layout::Rect;
use crate::paint::DisplayCommand;
use crate::style::values::computed::length::CSSPixelLength;
use gl::util::opengl_version;
use gl::Gl;
use glutin::event_loop::EventLoop;
use glutin::window::WindowBuilder;
use glutin::{ContextBuilder, GlProfile, PossiblyCurrent, WindowedContext};

pub mod ndc;
pub mod rect;

pub fn init_main_window_and_gl() -> (WindowedContext<PossiblyCurrent>, EventLoop<()>, Gl) {
    let el = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_title("Kosmonaut")
        .with_maximized(true);
    let windowed_context = ContextBuilder::new()
        .with_gl_profile(GlProfile::Core)
        .build_windowed(wb, &el)
        .unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };
    // TODO: Do we need to set gl.Viewport() size here?
    let gl_context = windowed_context.context();
    let gl = Gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);
    (windowed_context, el, gl)
}

pub fn redraw(
    windowed_context: &WindowedContext<PossiblyCurrent>,
    gl: &Gl,
    display_list: &[DisplayCommand],
    rect_painter: &mut RectPainter,
) {
    let viewport = Rect {
        start_x: 0.0,
        start_y: 0.0,
        width: CSSPixelLength::new(windowed_context.window().inner_size().width as f32),
        height: CSSPixelLength::new(windowed_context.window().inner_size().height as f32),
    };
    let mut vertex_data = Vec::new();
    for command in display_list {
        vertex_data.extend(command.to_vertices(viewport));
    }
    // TODO: Do we need to set gl.Viewport() size here?
    unsafe {
        gl.ClearColor(0.1, 0.9, 0.3, 1.0);
        gl.Clear(gl::COLOR_BUFFER_BIT);
    }
    rect_painter.paint(vertex_data.as_slice());
    windowed_context.swap_buffers().unwrap();
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
