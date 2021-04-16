use crate::{load_and_config_gl, print_gl_info, LogGlInfo, TARGETED_GL_PROFILE};
use gl::buffer::framebuffer::FrameBuffer;
use gl::buffer::renderbuffer::RenderBuffer;
use gl::buffer::Buffer;
use gl::pixels::{read_pixels, RgbaPixel};
use gl::types::GLint;
use gl::viewport::resize_viewport;
use gl::{Gl, COLOR_ATTACHMENT0};
use glutin::dpi::PhysicalSize;
use glutin::event_loop::EventLoop;
use glutin::{
    Context, ContextBuilder, ContextCurrentState, CreationError, GlRequest, NotCurrent,
    PossiblyCurrent,
};

pub fn init_framebuffer_and_gl(
    width_px: f32,
    height_px: f32,
    log_gl_info: LogGlInfo,
) -> HeadlessGfxContent<PossiblyCurrent> {
    let cb = ContextBuilder::new()
        .with_gl_profile(TARGETED_GL_PROFILE)
        .with_gl(GlRequest::Latest);
    let size = PhysicalSize::new(width_px, height_px);
    let (headless_context, _el) = build_context(cb, log_gl_info).unwrap();
    let headless_context = unsafe { headless_context.make_current().unwrap() };
    let gl = load_and_config_headless_gl(&headless_context, log_gl_info);
    let framebuffer = FrameBuffer::new(
        RenderBuffer::new(&gl, size.width as _, size.height as _),
        &gl,
    );
    resize_viewport(&gl, size.width as _, size.height as _);
    HeadlessGfxContent::new(headless_context, framebuffer, gl)
}

fn load_and_config_headless_gl(context: &Context<PossiblyCurrent>, log_gl_info: LogGlInfo) -> Gl {
    let gl = load_and_config_gl(context);
    if log_gl_info == LogGlInfo::Yes {
        print_gl_info(context, None, &gl);
    }
    gl
}

pub struct HeadlessGfxContent<T: ContextCurrentState> {
    /// The OpenGL context.
    _context: Context<T>,
    /// The framebuffer to render to.
    framebuffer: FrameBuffer,
    /// The handle to the OpenGL API associated with `context`.
    gl: Gl,
}

impl<T: ContextCurrentState> HeadlessGfxContent<T> {
    fn new(context: Context<T>, framebuffer: FrameBuffer, gl: Gl) -> HeadlessGfxContent<T> {
        HeadlessGfxContent {
            _context: context,
            framebuffer,
            gl,
        }
    }

    pub fn gl(&self) -> &Gl {
        &self.gl
    }

    pub fn read_pixels(&self, viewport_width: f32, viewport_height: f32) -> Vec<RgbaPixel> {
        // All framebuffers in Kosmonaut are currently attached to COLOR_ATTACHMENT0.
        // https://docs.gl/gl3/glReadBuffer
        unsafe {
            self.gl.ReadBuffer(COLOR_ATTACHMENT0);
        }
        read_pixels(&self.gl, viewport_width as GLint, viewport_height as GLint)
    }

    /// Binds the framebuffer associated with this context to the GL handle associated with this
    /// context.  Note this is a GL-handle-global operation.
    pub fn bind_framebuffer(&self) {
        self.framebuffer.bind_to(&self.gl);
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

// This function was taken from here and modified slightly for our needs (Glutin is Apache 2.0 licensed):
// https://github.com/rust-windowing/glutin/blob/bab33a84dfb094ff65c059400bed7993434638e2/glutin_examples/examples/headless.rs#L38
#[cfg(target_os = "linux")]
fn build_context<T1: ContextCurrentState>(
    cb: ContextBuilder<T1>,
    log_gl_info: LogGlInfo,
) -> Result<(Context<NotCurrent>, EventLoop<()>), [CreationError; 3]> {
    // On Unix operating systems, you should always try for surfaceless first,
    // and if that does not work, headless (pbuffers), and if that too fails,
    // finally osmesa.
    //
    // If willing, you could attempt to use hidden windows instead of os mesa,
    // but note that you must handle events for the window that come on the
    // events loop.
    let el = EventLoop::new();

    if log_gl_info == LogGlInfo::Yes {
        println!("Trying surfaceless");
    }
    let err1 = match build_context_surfaceless(cb.clone(), &el) {
        Ok(ctx) => return Ok((ctx, el)),
        Err(err) => err,
    };

    if log_gl_info == LogGlInfo::Yes {
        println!("Trying headless");
    }
    let err2 = match build_context_headless(cb.clone(), &el) {
        Ok(ctx) => return Ok((ctx, el)),
        Err(err) => err,
    };

    if log_gl_info == LogGlInfo::Yes {
        println!("Trying osmesa");
    }
    let err3 = match build_context_osmesa(cb) {
        Ok(ctx) => return Ok((ctx, el)),
        Err(err) => err,
    };

    Err([err1, err2, err3])
}

#[cfg(not(target_os = "linux"))]
fn build_context<T1: ContextCurrentState>(
    cb: ContextBuilder<T1>,
    log_gl_info: LogGlInfo,
) -> Result<(Context<NotCurrent>, EventLoop<()>), CreationError> {
    if log_gl_info == LogGlInfo::Yes {
        println!("Trying to build platform-default headless context")
    }
    let el = EventLoop::new();
    build_context_headless(cb.clone(), &el).map(|ctx| (ctx, el))
}
