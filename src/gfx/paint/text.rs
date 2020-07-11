use crate::gfx::display::CharCommand;
use crate::gfx::ndc::{ndc_x, ndc_y};
use crate::gfx::paint::{build_program, CharPaintData, ToVertices};
use cssparser::RGBA;
use gl::program::Program;
use gl::types::{GLint, GLsizeiptr};
use gl::vao::VertexArrayObject;
use gl::vbo::VertexBufferObject;
use gl::{Gl, ARRAY_BUFFER, DYNAMIC_DRAW, FALSE, FLOAT, TEXTURE0, TEXTURE_2D, TRIANGLES};
use pathfinder_geometry::transform3d::Transform4F;
use std::ffi::CString;

/// Uses given OpenGL instance to paint arbitrary text.
pub struct TextPainter {
    /// An instance of OpenGL.
    gl: Gl,
    /// The OpenGL program that will be used to paint text.
    program: Program,
    /// The VAO to use to paint text.
    vao: VertexArrayObject,
}

impl TextPainter {
    pub fn new(gl: &Gl) -> Result<TextPainter, String> {
        let vbo = VertexBufferObject::new(gl);
        let config_vao = |gl: &Gl| {
            unsafe {
                // Our 2D quad requires 6 vertices of 4 floats each, so pass that as the size of the buffer.
                // Pass a null pointer because we don't want to initialize the buffer with any data right now.
                // DYNAMIC_DRAW because this buffer will be updated very often (once for each character drawn).
                gl.BufferData(
                    ARRAY_BUFFER,
                    (6 * 4 * std::mem::size_of::<f32>()) as GLsizeiptr,
                    std::ptr::null(),
                    DYNAMIC_DRAW,
                );
                // Enable use of `layout (location=0)` data in our vertex shader.
                gl.EnableVertexAttribArray(0);
                // Location index 0 data has a size of four floats and is not normalized, where normalization
                // is the process of OpenGL mapping the values to a [-1,1] range for signed values or a range
                // of [0,1] for unsigned values.
                gl.VertexAttribPointer(
                    0,
                    4,
                    FLOAT,
                    FALSE,
                    (4 * std::mem::size_of::<f32>()) as GLint,
                    std::ptr::null(),
                );
            }
        };
        let vao = unsafe { VertexArrayObject::new(vbo, config_vao, gl) };

        Ok(TextPainter {
            program: build_text_program(gl)?,
            vao,
            gl: gl.clone(),
        })
    }

    pub fn paint(
        &mut self,
        paintable_chars: &[CharPaintData],
        viewport_width: u32,
        viewport_height: u32,
    ) {
        self.program.use_globally();

        let text_color_str =
            CString::new("textColor").expect("couldn't create `textColor` cstring");
        let projection_str =
            CString::new("projection").expect("couldn't create `projection` cstring");
        let black = RGBA::new(0, 0, 0, 1);
        unsafe {
            // TODO: Clean this up.  Should be using the color provided by the character, not hardcoded black.
            self.gl.Uniform3f(
                self.gl
                    .GetUniformLocation(self.program.id(), text_color_str.as_ptr()),
                black.red_f32(),
                black.green_f32(),
                black.blue_f32(),
            );
            // TODO: This isn't correct.
            let projection = Transform4F::from_ortho(
                0.,
                viewport_width as f32,
                viewport_height as f32,
                0.,
                0.0,
                100.0,
            );
            self.gl.UniformMatrix4fv(
                self.gl
                    .GetUniformLocation(self.program.id(), projection_str.as_ptr()),
                1,
                FALSE,
                projection.as_ptr(),
            );
            self.gl.ActiveTexture(TEXTURE0);
            self.gl.BindVertexArray(self.vao.name());
        }
        for ch in paintable_chars {
            // Panic rather than truncate data.
            assert!(ch.vertices.len() <= i32::max_value() as usize);

            unsafe {
                self.gl.BindTexture(TEXTURE_2D, ch.texture_id);
                self.vao.store_vertex_data(&ch.vertices);
                // Casting the `usize` to `GLint` will not truncate due to the above assert!().
                self.gl.DrawArrays(TRIANGLES, 0, ch.vertices.len() as i32);
                // TODO:  We need to increment the `x_pos` (which is calculated somewhere else) with the advance of this character.
                // This should probably happen somewhere else (layout?  display command processing?)
                // Here is some C++ code that does this work:
                //     now advance cursors for next glyph (note that advance is number of 1/64 pixels)
                //     x += (ch.Advance >> 6) * scale; // bitshift by 6 to get value in pixels (2^6 = 64)
            }
        }

        unsafe {
            self.gl.BindVertexArray(0);
            self.gl.BindTexture(TEXTURE_2D, 0);
        }
    }
}

fn build_text_program(gl: &Gl) -> Result<Program, String> {
    let vertex_shader_src = &CString::new(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/shader_src/text.vert"
    )))
    .expect("could not create cstring for text program");

    let frag_shader_src = &CString::new(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/shader_src/text.frag"
    )))
    .expect("could not create cstring for text program");

    build_program(vertex_shader_src, frag_shader_src, gl)
}

impl ToVertices for CharCommand {
    fn to_vertices(&self, viewport_width: f32, viewport_height: f32) -> Vec<f32> {
        let x_pos = self.start_coords().x() + self.bearing().x();
        let y_pos = self.start_coords().y() - (self.size().y() - self.bearing().y());

        // Transpose the quad width and height values (which are the second half of each vertex)
        // relative to what https://learnopengl.com/In-Practice/Text-Rendering has because of the
        // way we flip `ndc_y` values to render them at the top of the screen.
        let mut vertices = Vec::new();
        vertices.extend_from_slice(&[
            ndc_x(x_pos, viewport_width),
            ndc_y(y_pos + self.size().y(), viewport_height),
            1.0,
            1.0,
        ]);
        vertices.extend_from_slice(&[
            ndc_x(x_pos, viewport_width),
            ndc_y(y_pos, viewport_height),
            1.0,
            0.0,
        ]);
        vertices.extend_from_slice(&[
            ndc_x(x_pos + self.size().x(), viewport_width),
            ndc_y(y_pos, viewport_height),
            0.0,
            0.0,
        ]);

        vertices.extend_from_slice(&[
            ndc_x(x_pos, viewport_width),
            ndc_y(y_pos + self.size().y(), viewport_height),
            1.0,
            1.0,
        ]);
        vertices.extend_from_slice(&[
            ndc_x(x_pos + self.size().x(), viewport_width),
            ndc_y(y_pos, viewport_height),
            0.0,
            0.0,
        ]);
        vertices.extend_from_slice(&[
            ndc_x(x_pos + self.size().x(), viewport_width),
            ndc_y(y_pos + self.size().y(), viewport_height),
            0.0,
            1.0,
        ]);

        vertices
    }
}
