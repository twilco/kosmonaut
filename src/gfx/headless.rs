use crate::gfx::{
    load_and_config_gl, print_gl_info, DEFAULT_INNER_WINDOW_HEIGHT_PX,
    DEFAULT_INNER_WINDOW_WIDTH_PX, TARGETED_GL_PROFILE,
};
use gl::buffer::framebuffer::FrameBuffer;
use gl::buffer::renderbuffer::RenderBuffer;
use gl::viewport::resize_viewport;
use gl::Gl;
use glutin::dpi::PhysicalSize;
use glutin::event_loop::EventLoop;
use glutin::{
    Context, ContextBuilder, ContextCurrentState, CreationError, GlRequest, NotCurrent,
    PossiblyCurrent,
};

pub fn init_framebuffer_and_gl(
    width_px_opt: Option<f32>,
    height_px_opt: Option<f32>,
) -> HeadlessGfxContent<PossiblyCurrent> {
    let cb = ContextBuilder::new()
        .with_gl_profile(TARGETED_GL_PROFILE)
        .with_gl(GlRequest::Latest);
    let size = PhysicalSize::new(
        width_px_opt.unwrap_or(DEFAULT_INNER_WINDOW_WIDTH_PX),
        height_px_opt.unwrap_or(DEFAULT_INNER_WINDOW_HEIGHT_PX),
    );
    let (headless_context, _el) = build_context(cb).unwrap();
    let headless_context = unsafe { headless_context.make_current().unwrap() };
    let gl = load_and_config_headless_gl(&headless_context);
    let framebuffer = FrameBuffer::new(
        RenderBuffer::new(&gl, size.width as _, size.height as _),
        &gl,
    );
    resize_viewport(&gl, size.width as _, size.height as _);
    HeadlessGfxContent::new(headless_context, framebuffer, gl)
}

pub fn load_and_config_headless_gl(context: &Context<PossiblyCurrent>) -> Gl {
    let gl = load_and_config_gl(context);
    print_gl_info(context, None, &gl);
    gl
}

pub struct HeadlessGfxContent<T: ContextCurrentState> {
    /// The OpenGL context.
    context: Context<T>,
    /// The framebuffer to render to.
    framebuffer: FrameBuffer,
    /// The handle to the OpenGL API associated with `context`.
    gl: Gl,
}

impl<T: ContextCurrentState> HeadlessGfxContent<T> {
    pub fn new(context: Context<T>, framebuffer: FrameBuffer, gl: Gl) -> HeadlessGfxContent<T> {
        HeadlessGfxContent {
            context,
            framebuffer,
            gl,
        }
    }
}

#[cfg(target_os = "linux")]
fn build_context_surfaceless<T1: ContextCurrentState>(
    cb: ContextBuilder<T1>,
    el: &EventLoop<()>,
) -> Result<Context<NotCurrent>, CreationError> {
    use glutin::platform::unix::HeadlessContextExt;
    cb.build_surfaceless(&el)
}

fn build_context_headless<T1: ContextCurrentState>(
    cb: ContextBuilder<T1>,
    el: &EventLoop<()>,
) -> Result<Context<NotCurrent>, CreationError> {
    let size_one = PhysicalSize::new(1, 1);
    cb.build_headless(&el, size_one)
}

#[cfg(target_os = "linux")]
fn build_context_osmesa<T1: ContextCurrentState>(
    cb: ContextBuilder<T1>,
) -> Result<Context<NotCurrent>, CreationError> {
    use glutin::platform::unix::HeadlessContextExt;
    let size_one = PhysicalSize::new(1, 1);
    cb.build_osmesa(size_one)
}

#[cfg(target_os = "linux")]
fn build_context<T1: ContextCurrentState>(
    cb: ContextBuilder<T1>,
) -> Result<(Context<NotCurrent>, EventLoop<()>), [CreationError; 3]> {
    // On unix operating systems, you should always try for surfaceless first,
    // and if that does not work, headless (pbuffers), and if that too fails,
    // finally osmesa.
    //
    // If willing, you could attempt to use hidden windows instead of os mesa,
    // but note that you must handle events for the window that come on the
    // events loop.
    let el = EventLoop::new();

    println!("Trying surfaceless");
    let err1 = match build_context_surfaceless(cb.clone(), &el) {
        Ok(ctx) => return Ok((ctx, el)),
        Err(err) => err,
    };

    println!("Trying headless");
    let err2 = match build_context_headless(cb.clone(), &el) {
        Ok(ctx) => return Ok((ctx, el)),
        Err(err) => err,
    };

    println!("Trying osmesa");
    let err3 = match build_context_osmesa(cb) {
        Ok(ctx) => return Ok((ctx, el)),
        Err(err) => err,
    };

    Err([err1, err2, err3])
}

#[cfg(not(target_os = "linux"))]
fn build_context<T1: ContextCurrentState>(
    cb: ContextBuilder<T1>,
) -> Result<(Context<NotCurrent>, EventLoop<()>), CreationError> {
    let el = EventLoop::new();
    build_context_headless(cb.clone(), &el).map(|ctx| (ctx, el))
}
