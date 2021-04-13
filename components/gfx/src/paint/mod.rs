use crate::paint::rect::RectPainter;
use crate::paint::text::TextPainter;
use cssparser::RGBA;
use display_list::DisplayCommand;
use gl::program::Program;
use gl::shader::{Shader, ShaderKind};
use gl::texture::TextureId;
use gl::Gl;
use glutin::{PossiblyCurrent, WindowedContext};
use layout::LayoutViewportDimensions;
use std::ffi::CString;

pub mod rect;
pub mod text;

/// Wraps other painters to ensure they are only painting OpenGL vertex data (paint) that
/// corresponds to their "bucket".  This is necessary because vertex data for a rectangle needs to
/// be painted differently (namely, different OpenGL drawing sequences) than vertex data for text,
/// as an example.
pub struct MasterPainter {
    /// The OpenGL context to paint to.
    gl: Gl,
    rect_painter: RectPainter,
    rect_vertices: Vec<f32>,
    scale_factor: f32,
    text_painter: TextPainter,
    text_vertices: Vec<CharPaintData>,
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
    pub fn new(gl: &Gl, scale_factor: f32) -> Result<MasterPainter, String> {
        Ok(MasterPainter {
            gl: gl.clone(),
            rect_painter: RectPainter::new(gl)?,
            rect_vertices: Vec::new(),
            scale_factor,
            text_painter: TextPainter::new(gl)?,
            text_vertices: Vec::new(),
        })
    }

    pub fn paint_headed(
        &mut self,
        windowed_context: &WindowedContext<PossiblyCurrent>,
        display_list: &[DisplayCommand],
    ) {
        // Note: For semantic correctness, the OpenGL context (the `gl` member on `self`) must also have its viewport
        // set to the below dimensions, presumably done outside this function when the window is resized.
        // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glViewport.xhtml
        let inner_window_size = windowed_context.window().inner_size();

        self.paint_inner(
            LayoutViewportDimensions::from_px(
                inner_window_size.width as f32,
                inner_window_size.height as f32,
            ),
            display_list,
        );
        windowed_context
            .swap_buffers()
            .expect("couldn't swap window buffers");
    }

    pub fn paint_headless(
        &mut self,
        viewport: LayoutViewportDimensions,
        display_list: &[DisplayCommand],
    ) {
        self.paint_inner(viewport, display_list);
    }

    fn paint_inner(&mut self, viewport: LayoutViewportDimensions, display_list: &[DisplayCommand]) {
        for command in display_list {
            self.process_display_command(command, viewport);
        }
        self.rect_painter.paint(self.rect_vertices.as_slice());
        self.text_painter.paint(self.text_vertices.as_slice());
        // Now that we've painted, let's dump the paint buckets so they're clean for the next paint.
        self.rect_vertices.clear();
        self.text_vertices.clear();
    }

    fn process_display_command(
        &mut self,
        command: &DisplayCommand,
        viewport: LayoutViewportDimensions,
    ) {
        match command {
            DisplayCommand::Char(char_command) => {
                self.text_vertices.push(CharPaintData::new(
                    char_command.color(),
                    char_command.texture_id(),
                    char_command.to_vertices(viewport, self.scale_factor),
                ));
            }
            DisplayCommand::RectSolidColor(rgba, rect) => self
                .rect_vertices
                .extend((rect, rgba).to_vertices(viewport, self.scale_factor)),
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
    fn to_vertices(&self, viewport: LayoutViewportDimensions, scale_factor: f32) -> Vec<f32>;
}

impl ToVertices for RGBA {
    fn to_vertices(&self, _viewport: LayoutViewportDimensions, _scale_factor: f32) -> Vec<f32> {
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
