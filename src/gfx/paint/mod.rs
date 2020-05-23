use crate::gfx::display::DisplayCommand;
use crate::gfx::paint::rect::RectPainter;
use crate::style::values::CSSFloat;
use cssparser::RGBA;
use gl::Gl;
use glutin::{PossiblyCurrent, WindowedContext};

pub mod rect;

pub fn paint(
    windowed_context: &WindowedContext<PossiblyCurrent>,
    gl: &Gl,
    display_list: &[DisplayCommand],
    rect_painter: &mut RectPainter,
) {
    // Note: For semantic correctness, the OpenGL instance (represented by the `gl` parameter) must
    // also have its viewport specified to the below dimensions, presumably done outside this
    // function when the window is resized.
    // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glViewport.xhtml
    let viewport_width = windowed_context.window().inner_size().width;
    let viewport_height = windowed_context.window().inner_size().height;
    let mut vertex_data = Vec::new();

    for command in display_list {
        process_display_command(
            command,
            &mut vertex_data,
            viewport_width as f32,
            viewport_height as f32,
            gl,
        );
    }
    rect_painter.paint(vertex_data.as_slice());
    windowed_context.swap_buffers().unwrap();
}

fn process_display_command(
    command: &DisplayCommand,
    vertex_data: &mut Vec<f32>,
    viewport_width: CSSFloat,
    viewport_height: CSSFloat,
    gl: &Gl,
) {
    match command {
        DisplayCommand::RectSolidColor(rgba, rect) => {
            vertex_data.extend((rect, rgba).to_vertices(viewport_width, viewport_height))
        }
        DisplayCommand::ViewportBackground(rgba) => unsafe {
            gl.ClearColor(
                rgba.red_f32(),
                rgba.green_f32(),
                rgba.blue_f32(),
                rgba.alpha_f32(),
            );
            gl.Clear(gl::COLOR_BUFFER_BIT);
        },
    }
}

/// Represents the conversion from some entity to OpenGL vertex data.
pub trait ToVertices {
    fn to_vertices(&self, viewport_width: CSSFloat, viewport_height: CSSFloat) -> Vec<f32>;
}

impl ToVertices for RGBA {
    fn to_vertices(&self, _viewport_width: f32, _viewport_height: f32) -> Vec<f32> {
        let mut vertices = Vec::new();
        vertices.extend_from_slice(&[
            self.red_f32(),
            self.green_f32(),
            self.blue_f32(),
            self.alpha_f32(),
        ]);
        vertices
    }
}
