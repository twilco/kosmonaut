use crate::layout::layout_box::{BoxType, LayoutBox};
use crate::layout::Rect;
use crate::style::values::computed::LineStyle;
use crate::Side;
use cssparser::RGBA;

/// Builds list of display commands that should be used to paint the output.
pub fn build_display_list(layout_box: &LayoutBox) -> DisplayList {
    let mut display_list = Vec::new();
    render_layout_box(&mut display_list, &layout_box);
    display_list
}

/// Represents a display list, which is a list of graphics operations Kosmonaut should perform to
/// paint output to the screen.
///
/// Display lists are useful because painting is an expensive
/// operation, and display lists allow you to search the list for work that would covered up by
/// other things later in the list.
///
/// It also generalizes the input for various different types of
/// output, such as making pixels for display on a screen or vector graphics for sending to a
/// printer.
///
/// https://en.wikipedia.org/wiki/Display_list
type DisplayList = Vec<DisplayCommand>;

/// A command to perform a graphics operation.
#[derive(Clone, Copy, Debug)]
pub enum DisplayCommand {
    SolidColor(RGBA, Rect),
}

/// Renders a layout box in the correct order.  The order in which each part of a box is painted is
/// defined here: https://www.w3.org/TR/CSS22/zindex.html
fn render_layout_box(display_list: &mut Vec<DisplayCommand>, layout_box: &LayoutBox) {
    // TODO: Implement step 1 of painting order for root element

    match layout_box.box_type {
        BoxType::Block => {
            // Step 2 of painting order
            render_block_listitem_block_equiv(display_list, layout_box)
        }
        _ => {
            // TODO: Implement other steps of paining order, 3 -> 10
            println!("skipping render of non-block box")
        }
    }

    for child in &layout_box.children {
        render_layout_box(display_list, child);
    }
}

fn render_block_listitem_block_equiv(
    display_list: &mut Vec<DisplayCommand>,
    layout_box: &LayoutBox,
) {
    render_background(display_list, layout_box);
    render_borders(display_list, layout_box);
    // TODO: Render text - https://learnopengl.com/In-Practice/Text-Rendering
}

/// Renders the background as display command(s) for the `layout_box`.
fn render_background(display_list: &mut Vec<DisplayCommand>, layout_box: &LayoutBox) {
    let bg_color = layout_box.computed_values().background_color.rgba();
    if bg_color != RGBA::transparent() {
        display_list.push(DisplayCommand::SolidColor(
            bg_color,
            layout_box.dimensions.border_box(),
        ))
    }
}

/// Renders the borders as display commands for the `layout_box`.
fn render_borders(display_list: &mut Vec<DisplayCommand>, layout_box: &LayoutBox) {
    render_border(display_list, layout_box, Side::Bottom);
    render_border(display_list, layout_box, Side::Left);
    render_border(display_list, layout_box, Side::Right);
    render_border(display_list, layout_box, Side::Top);
}

/// Renders the border `side` as a display command for the `layout_box`.
fn render_border(display_list: &mut Vec<DisplayCommand>, layout_box: &LayoutBox, side: Side) {
    let cvs = layout_box.computed_values();
    let d = layout_box.dimensions;
    let border_style = cvs.border_style(side);
    let border_color_rgba = cvs.border_color_rgba(side);
    // The border size has already been calculated during layout, so we don't need to get it from
    // the computed values here.
    let border_size_px = d.border_size(side);
    if border_style == LineStyle::None
        || border_style == LineStyle::Hidden
        || border_color_rgba == RGBA::transparent()
        || border_size_px == 0.
    {
        return;
    }

    let border_box = d.border_box();
    match side {
        Side::Bottom => display_list.push(DisplayCommand::SolidColor(
            border_color_rgba,
            Rect {
                start_x: border_box.start_x,
                start_y: (border_box.start_y + border_box.height - border_size_px).px(),
                width: border_box.width,
                height: border_size_px,
            },
        )),
        Side::Left => display_list.push(DisplayCommand::SolidColor(
            border_color_rgba,
            Rect {
                start_x: border_box.start_x,
                start_y: border_box.start_y,
                width: border_size_px,
                height: border_box.height,
            },
        )),
        Side::Right => display_list.push(DisplayCommand::SolidColor(
            border_color_rgba,
            Rect {
                start_x: (border_box.start_x + border_box.width - border_size_px).px(),
                start_y: border_box.start_y,
                width: border_size_px,
                height: border_box.height,
            },
        )),
        Side::Top => display_list.push(DisplayCommand::SolidColor(
            border_color_rgba,
            Rect {
                start_x: border_box.start_x,
                start_y: border_box.start_y,
                width: border_box.width,
                height: border_size_px,
            },
        )),
    }
}
