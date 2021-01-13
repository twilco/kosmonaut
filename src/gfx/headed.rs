use crate::gfx::{
    load_and_config_gl, print_gl_info, resize_window, LogGlInfo, DEFAULT_INNER_WINDOW_HEIGHT_PX,
    DEFAULT_INNER_WINDOW_WIDTH_PX, TARGETED_GL_PROFILE,
};
use gl::Gl;
use glutin::dpi::PhysicalSize;
use glutin::event_loop::EventLoop;
use glutin::window::{Icon, WindowBuilder};
use glutin::ContextBuilder;
use glutin::{PossiblyCurrent, WindowedContext};
use image::ImageFormat;
use std::io::Cursor;

pub fn init_window_and_gl(
    inner_width_opt: Option<f32>,
    inner_height_opt: Option<f32>,
    log_gl_info: LogGlInfo,
) -> (WindowedContext<PossiblyCurrent>, EventLoop<()>, Gl) {
    let el = EventLoop::new();
    let initial_physical_size = PhysicalSize {
        width: inner_width_opt.unwrap_or(DEFAULT_INNER_WINDOW_WIDTH_PX) as u32,
        height: inner_height_opt.unwrap_or(DEFAULT_INNER_WINDOW_HEIGHT_PX) as u32,
    };
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
        .with_gl_profile(TARGETED_GL_PROFILE)
        .build_windowed(wb, &el)
        .unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };
    let gl = load_and_config_headed_gl(&windowed_context, log_gl_info);
    resize_window(&gl, &windowed_context, &initial_physical_size);
    (windowed_context, el, gl)
}

fn load_and_config_headed_gl(
    windowed_context: &WindowedContext<PossiblyCurrent>,
    log_gl_info: LogGlInfo,
) -> Gl {
    let gl = load_and_config_gl(windowed_context.context());
    if log_gl_info == LogGlInfo::Yes {
        print_gl_info(
            &windowed_context,
            Some(windowed_context.get_pixel_format()),
            &gl,
        );
    }
    gl
}
