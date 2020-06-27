use crate::gfx::display::DisplayCommand;
use crate::gfx::paint::rect::RectPainter;
use crate::gfx::paint::text::TextPainter;
use crate::style::values::CSSFloat;
use cssparser::RGBA;
use gl::program::Program;
use gl::shader::{Shader, ShaderKind};
use gl::texture::TextureId;
use gl::Gl;
use glutin::{PossiblyCurrent, WindowedContext};
use std::ffi::CString;

pub mod rect;
pub mod text;

/// Wraps other painters to ensure they are only painting OpenGL vertex data (paint) that
/// corresponds to their "bucket".  This is necessary because vertex data for a rectangle needs to
/// be painted differently (namely, different OpenGL drawing sequences) than vertex data for text,
/// as an example.
pub struct MasterPainter {
    rect_painter: RectPainter,
    rect_vertices: Vec<f32>,
    text_painter: TextPainter,
    text_vertices: Vec<CharPaintData>,
    /// The OpenGL instance to paint to.
    gl: Gl,
}

/// Data necessary to paint a character with OpenGL.
#[derive(Clone, Debug)]
pub struct CharPaintData {
    // TODO: It would probably be cleaner to handle colors like RectPainter does vs. passing it here
    pub color: RGBA,
    pub texture_id: TextureId,
    pub vertices: Vec<f32>,
}

impl CharPaintData {
    pub fn new(color: RGBA, texture_id: TextureId, vertices: Vec<f32>) -> Self {
        CharPaintData {
            color,
            texture_id,
            vertices,
        }
    }
}

impl MasterPainter {
    pub fn new(gl: &Gl) -> Result<MasterPainter, String> {
        Ok(MasterPainter {
            rect_painter: RectPainter::new(gl)?,
            rect_vertices: Vec::new(),
            text_painter: TextPainter::new(gl)?,
            text_vertices: Vec::new(),
            gl: gl.clone(),
        })
    }

    pub fn paint(
        &mut self,
        windowed_context: &WindowedContext<PossiblyCurrent>,
        display_list: &[DisplayCommand],
    ) {
        // Note: For semantic correctness, the OpenGL instance (the `gl` member on `self`) must
        // also have its viewport specified to the below dimensions, presumably done outside this
        // function when the window is resized.
        // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glViewport.xhtml
        let viewport_width = windowed_context.window().inner_size().width;
        let viewport_height = windowed_context.window().inner_size().height;

        for command in display_list {
            self.process_display_command(
                command,
                viewport_width as CSSFloat,
                viewport_height as CSSFloat,
            );
        }
        self.rect_painter.paint(self.rect_vertices.as_slice());
        self.text_painter.paint(self.text_vertices.as_slice(), viewport_width, viewport_height);
        // Now that we've painted, let's dump the paint buckets so they're clean for the next paint.
        self.rect_vertices.clear();
        self.text_vertices.clear();
        windowed_context
            .swap_buffers()
            .expect("couldn't swap window buffers");
    }

    fn process_display_command(
        &mut self,
        command: &DisplayCommand,
        viewport_width: CSSFloat,
        viewport_height: CSSFloat,
    ) {
        match command {
            DisplayCommand::Char(char_command) => {
                self.text_vertices.push(CharPaintData::new(
                    char_command.color(),
                    char_command.texture_id(),
                    char_command.to_vertices(viewport_width, viewport_height),
                ));
            }
            DisplayCommand::RectSolidColor(rgba, rect) => self
                .rect_vertices
                .extend((rect, rgba).to_vertices(viewport_width, viewport_height)),
            DisplayCommand::ViewportBackground(rgba) => unsafe {
                self.gl.ClearColor(
                    rgba.red_f32(),
                    rgba.green_f32(),
                    rgba.blue_f32(),
                    rgba.alpha_f32(),
                );
                self.gl.Clear(gl::COLOR_BUFFER_BIT);
            },
        }
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

pub fn build_program(
    vertex_shader_src: &CString,
    fragment_shader_src: &CString,
    gl: &Gl,
) -> Result<Program, String> {
    let vertex_shader = Shader::from_source(vertex_shader_src, ShaderKind::Vertex, gl)?;
    let fragment_shader = Shader::from_source(fragment_shader_src, ShaderKind::Fragment, gl)?;
    let program = Program::from_shaders(&[vertex_shader, fragment_shader], &gl)?;
    Ok(program)
}
