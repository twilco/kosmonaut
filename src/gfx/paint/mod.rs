use crate::gfx::display::DisplayCommand;
use crate::gfx::ndc::{ndc_x, ndc_y};
use crate::gfx::paint::rect::RectPainter;
use gl::Gl;
use glutin::{PossiblyCurrent, WindowedContext};

pub mod rect;

pub fn paint(
    windowed_context: &WindowedContext<PossiblyCurrent>,
    gl: &Gl,
    display_list: &[DisplayCommand],
    rect_painter: &mut RectPainter,
) {
    // TODO: Do we need to set gl.Viewport() size here?
    let viewport_width = windowed_context.window().inner_size().width as f32;
    let viewport_height = windowed_context.window().inner_size().height as f32;
    let mut vertex_data = Vec::new();

    for command in display_list {
        process_display_command(
            command,
            &mut vertex_data,
            viewport_width,
            viewport_height,
            gl,
        );
    }
    rect_painter.paint(vertex_data.as_slice());
    windowed_context.swap_buffers().unwrap();
}

fn process_display_command(
    command: &DisplayCommand,
    vertex_data: &mut Vec<f32>,
    viewport_width: f32,
    viewport_height: f32,
    gl: &Gl,
) {
    match command {
        DisplayCommand::RectSolidColor(rgba, rect) => {
            // impl of command::to_vertices should probably be removed and moved into here
            //                vertex_data.extend(command.to_vertices(viewport));
            let rect_colors = &[
                rgba.red_f32(),
                rgba.green_f32(),
                rgba.blue_f32(),
                rgba.alpha_f32(),
            ];
            // TODO: Should there be a trait called `ToVertices` that Rect and other things impl?

            // Top-left vertex.
            vertex_data.extend_from_slice(&[
                ndc_x(rect.start_x, viewport_width),
                ndc_y(rect.start_y, viewport_height),
                // TODO: Implement z-indexing.
                0.0,
            ]);
            vertex_data.extend_from_slice(rect_colors);

            let top_right_vertex = &[
                ndc_x((rect.start_x + rect.width).px(), viewport_width),
                ndc_y(rect.start_y, viewport_height),
                0.0,
            ];
            let bottom_left_vertex = &[
                ndc_x(rect.start_x, viewport_width),
                ndc_y((rect.start_y + rect.height).px(), viewport_height),
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
                ndc_x((rect.start_x + rect.width).px(), viewport_width),
                ndc_y((rect.start_y + rect.height).px(), viewport_height),
                0.0,
            ]);
            vertex_data.extend_from_slice(rect_colors);
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
