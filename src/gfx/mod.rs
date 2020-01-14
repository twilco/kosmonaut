use gl::program::Program;
use gl::shader::{Shader, ShaderKind};
use gl::types::{GLint, GLsizeiptr, GLuint, GLvoid};
use gl::util::opengl_version;
use gl::Gl;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{ContextBuilder, GlProfile, PossiblyCurrent, WindowedContext};
use std::ffi::CString;

pub fn init_main_window_and_gl() -> (WindowedContext<PossiblyCurrent>, EventLoop<()>, Gl) {
    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("Kosmonaut");
    let windowed_context = ContextBuilder::new()
        .with_gl_profile(GlProfile::Core)
        .build_windowed(wb, &el)
        .unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };
    // TODO: Do we need to set viewport size here?
    let gl_context = windowed_context.context();
    let gl = Gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);
    (windowed_context, el, gl)
}

pub fn run_event_loop(
    windowed_context: WindowedContext<PossiblyCurrent>,
    event_loop: EventLoop<()>,
    gl: Gl,
) {
    #[rustfmt::skip]
    static VERTEX_DATA: [f32; 9] = [
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0,  0.5, 0.0
    ];

    event_loop.run(move |event, _, control_flow| {
        //        println!("{:?}", event);
        *control_flow = ControlFlow::Wait;
        match event {
            Event::LoopDestroyed => {}
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(logical_size) => {
                    let dpi_factor = windowed_context.window().hidpi_factor();
                    windowed_context.resize(logical_size.to_physical(dpi_factor));
                }
                WindowEvent::RedrawRequested => {
                    // TODO: Do we need to set viewport size here?
                    unsafe {
                        gl.ClearColor(0.1, 0.9, 0.3, 1.0);
                        gl.Clear(gl::COLOR_BUFFER_BIT);
                    }
                    let mut vbo: GLuint = 0;
                    unsafe {
                        gl.GenBuffers(1, &mut vbo);
                        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
                        gl.BufferData(
                            gl::ARRAY_BUFFER,                                               // target
                            (VERTEX_DATA.len() * std::mem::size_of::<f32>()) as GLsizeiptr, // size of data in bytes
                            VERTEX_DATA.as_ptr() as *const GLvoid, // pointer to data
                            gl::STATIC_DRAW,                       // usage
                        );
                        gl.BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
                    }
                    let mut vao: GLuint = 0;
                    unsafe {
                        gl.GenVertexArrays(1, &mut vao);
                        gl.BindVertexArray(vao);
                        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
                        gl.EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
                        gl.VertexAttribPointer(
                            0,         // index of the generic vertex attribute ("layout (location = 0)")
                            3,         // the number of components per generic vertex attribute
                            gl::FLOAT, // data type
                            gl::FALSE, // normalized (int-to-float conversion)
                            (3 * std::mem::size_of::<f32>()) as GLint, // stride (byte offset between consecutive attributes)
                            std::ptr::null(), // offset of the first component
                        );
                        gl.BindBuffer(gl::ARRAY_BUFFER, 0);
                        gl.BindVertexArray(0);
                    }
                    let p = build_program(&gl).expect("should've been able to build program");
                    p.use_globally();
                    unsafe {
                        gl.BindVertexArray(vao);
                        gl.DrawArrays(
                            gl::TRIANGLES, // mode
                            0,             // starting index in the enabled arrays
                            3,             // number of indices to be rendered
                        );
                    }
                    windowed_context.swap_buffers().unwrap();
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            _ => (),
        }
    });
}

fn build_program(gl: &Gl) -> Result<Program, String> {
    let vertex_shader = Shader::from_source(
        &CString::new(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/shader_src/triangle.vert"
        )))
        .expect("could not create cstring for triangle"),
        ShaderKind::Vertex,
        gl,
    )?;
    let fragment_shader = Shader::from_source(
        &CString::new(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/shader_src/triangle.frag"
        )))
        .expect("could not create cstring for triangle"),
        ShaderKind::Fragment,
        gl,
    )?;
    let program = Program::from_shaders(&[vertex_shader, fragment_shader], &gl)?;
    Ok(program)
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
