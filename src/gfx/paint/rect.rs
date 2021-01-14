use crate::gfx::ndc::{ndc_x, ndc_y};
use crate::gfx::paint::{build_program, ToVertices};
use crate::layout::rect::Rect;
use cssparser::RGBA;
use gl::buffer::vbo::VertexBufferObject;
use gl::program::Program;
use gl::types::{GLint, GLvoid};
use gl::vao::VertexArrayObject;
use gl::Gl;
use std::ffi::CString;

/// Uses given OpenGL context handle to paint arbitrary rectangles.
pub struct RectPainter {
    /// The OpenGL program that will be used to paint rectangles.
    program: Program,
    /// The VAO to use to paint rectangles.
    vao: VertexArrayObject,
    /// An instance of OpenGL.
    gl: Gl,
}

impl RectPainter {
    pub fn new(gl: &Gl) -> Result<RectPainter, String> {
        let vbo = VertexBufferObject::new(gl);
        let config_rect_vao = |gl: &Gl| {
            unsafe {
                gl.EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
                gl.VertexAttribPointer(
                    0,         // index of the generic vertex attribute ("layout (location = 0)")
                    3,         // the number of components per generic vertex attribute
                    gl::FLOAT, // data type
                    gl::FALSE, // normalized (int-to-float conversion)
                    (7 * std::mem::size_of::<f32>()) as GLint, // stride (byte offset between consecutive attributes)
                    std::ptr::null(),                          // offset of the first component
                );

                gl.EnableVertexAttribArray(1); // this is "layout (location = 1)" in vertex shader
                gl.VertexAttribPointer(
                    1,         // index of the generic vertex attribute ("layout (location = 1)")
                    4,         // the number of components per generic vertex attribute
                    gl::FLOAT, // data type
                    gl::FALSE, // normalized (int-to-float conversion)
                    (7 * std::mem::size_of::<f32>()) as GLint, // stride (byte offset between consecutive attributes)
                    (3 * std::mem::size_of::<f32>()) as *const GLvoid, // offset of the first component
                );
            }
        };
        let vao = unsafe { VertexArrayObject::new(vbo, config_rect_vao, gl) };

        Ok(RectPainter {
            program: build_triangle_program(gl)?,
            gl: gl.clone(),
            vao,
        })
    }

    pub fn paint(&mut self, vertices: &[f32]) {
        // Panic rather than truncate data.
        assert!(vertices.len() <= i32::max_value() as usize);

        self.program.use_globally();
        self.vao.store_vertex_data(&vertices[..]);
        unsafe {
            self.gl.BindVertexArray(self.vao.name());
            self.gl.DrawArrays(
                gl::TRIANGLES,
                0,
                // Safe because of the assert above.
                vertices.len() as i32,
            );
            self.gl.BindVertexArray(0);
        }
    }
}

fn build_triangle_program(gl: &Gl) -> Result<Program, String> {
    let vertex_shader_src = &CString::new(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/shader_src/triangle.vert"
    )))
    .expect("could not create cstring for triangle program");

    let frag_shader_src = &CString::new(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/shader_src/triangle.frag"
    )))
    .expect("could not create cstring for triangle program");

    build_program(vertex_shader_src, frag_shader_src, gl)
}

impl ToVertices for (&RGBA, &Rect) {
    fn to_vertices(
        &self,
        scaled_viewport_width: f32,
        scaled_viewport_height: f32,
        scale_factor: f32,
    ) -> Vec<f32> {
        (self.1, self.0).to_vertices(scaled_viewport_width, scaled_viewport_height, scale_factor)
    }
}

impl ToVertices for (&Rect, &RGBA) {
    fn to_vertices(
        &self,
        scaled_viewport_width: f32,
        scaled_viewport_height: f32,
        scale_factor: f32,
    ) -> Vec<f32> {
        let rect = self.0.scaled_by(scale_factor);
        let rgba_vec =
            self.1
                .to_vertices(scaled_viewport_width, scaled_viewport_height, scale_factor);
        let rect_colors = rgba_vec.as_slice();

        let mut vertex_data = Vec::new();
        // Top-left vertex.
        vertex_data.extend_from_slice(&[
            ndc_x(rect.start_x, scaled_viewport_width),
            ndc_y(rect.start_y, scaled_viewport_height),
            // TODO: Implement z-indexing.
            0.0,
        ]);
        vertex_data.extend_from_slice(rect_colors);

        let top_right_vertex = &[
            ndc_x((rect.start_x + rect.width).px(), scaled_viewport_width),
            ndc_y(rect.start_y, scaled_viewport_height),
            0.0,
        ];
        let bottom_left_vertex = &[
            ndc_x(rect.start_x, scaled_viewport_width),
            ndc_y((rect.start_y + rect.height).px(), scaled_viewport_height),
            0.0,
        ];
        vertex_data.extend_from_slice(top_right_vertex);
        vertex_data.extend_from_slice(rect_colors);
        vertex_data.extend_from_slice(bottom_left_vertex);
        vertex_data.extend_from_slice(rect_colors);

        // Second triangle.
        vertex_data.extend_from_slice(bottom_left_vertex);
        vertex_data.extend_from_slice(rect_colors);
        vertex_data.extend_from_slice(top_right_vertex);
        vertex_data.extend_from_slice(rect_colors);
        // Bottom-right vertex.
        vertex_data.extend_from_slice(&[
            ndc_x((rect.start_x + rect.width).px(), scaled_viewport_width),
            ndc_y((rect.start_y + rect.height).px(), scaled_viewport_height),
            0.0,
        ]);
        vertex_data.extend_from_slice(rect_colors);
        vertex_data
    }
}
