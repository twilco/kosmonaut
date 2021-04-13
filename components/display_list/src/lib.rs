use crate::char::CharCommand;
use cssparser::RGBA;
use layout::behavior::BaseLayoutBoxBehavior;
use layout::layout_box::LayoutBox;
use primitives::rect::{PositionedRect, Rect};
use primitives::sides::PhysicalSide;
use style::values::computed::LineStyle;

pub mod char;

/// Builds list of display commands that should be used to paint the output.
pub fn build_display_list(layout_box: &LayoutBox, _scale_factor: f32) -> DisplayList {
    let mut display_list = Vec::new();
    // TODO: This function used to take a CharHandle, but that is OpenGL-specific, and the
    // display_list module shouldn't depend on anything OpenGL-specific.
    // TODO: Remove the three preceeding statements once text rendering is fixed.
    // let font_handle = FontHandle::new();
    // let font = font_handle.get_font("Helvetica").unwrap();
    // char_handle
    //     .prepare_char(
    //         &mut display_list,
    //         'A',
    //         RGBA::new(0, 0, 0, 1),
    //         &font,
    //         Au::from_f32_px(12.),
    //         scale_factor,
    //     )
    //     .unwrap();
    prepare_layout_box(&mut display_list, &layout_box);
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
/// For now, our display list is OpenGL specific, but this could be made more dynamic should we
/// implement other types of rendering backends.
///
/// https://en.wikipedia.org/wiki/Display_list
pub type DisplayList = Vec<DisplayCommand>;

/// A command to perform a graphics operation.
#[derive(Clone, Debug)]
pub enum DisplayCommand {
    Char(CharCommand),
    RectSolidColor(RGBA, PositionedRect),
    /// This _could_ be represented as [`RectSolidColor`], but graphics APIs sometimes have a
    /// special background painting capabilities that are more idiomatic, such as OpenGL's
    /// `Clear(COLOR_BUFFER_BIT)` and `ClearColor(r, g, b, a)` APIs.
    ViewportBackground(RGBA),
}

/// Prepares a layout box for display in the correct order.  The order in which each part of a box
/// is painted is defined here: https://www.w3.org/TR/CSS22/zindex.html
fn prepare_layout_box(display_list: &mut DisplayList, layout_box: &LayoutBox) {
    // Step 1 of painting order
    if layout_box.is_root() {
        // Step 1.1
        let cvs = layout_box.computed_values();
        display_list.push(DisplayCommand::ViewportBackground(
            cvs.background_color.rgba(),
        ));
        // TODO: Step 1.2, painting background images
    }

    match layout_box {
        LayoutBox::BlockLevel(_) => prepare_block_listitem_block_equiv(display_list, layout_box),
        LayoutBox::InlineLevel(_) => {
            // TODO: Implement other steps of painting order, 3 -> 10
            // println!("skipping render of non-block box")
        }
    }

    if let Some(children) = layout_box.children() {
        for child in children {
            prepare_layout_box(display_list, child);
        }
    }
}

/// Preparation for step 2 from: https://www.w3.org/TR/CSS22/zindex.html
fn prepare_block_listitem_block_equiv(display_list: &mut DisplayList, layout_box: &LayoutBox) {
    prepare_background(display_list, layout_box);
    prepare_borders(display_list, layout_box);
    // TODO: Render text - https://learnopengl.com/In-Practice/Text-Rendering
}

/// Prepares the background of `layout_box` for display by converting it to display command(s).
fn prepare_background(display_list: &mut DisplayList, layout_box: &LayoutBox) {
    let bg_color = layout_box.computed_values().background_color.rgba();
    if bg_color != RGBA::transparent() {
        display_list.push(DisplayCommand::RectSolidColor(
            bg_color,
            layout_box.dimensions().border_box(),
        ))
    }
}

/// Prepares the borders of `layout_box` for display by converting them to display commands.
fn prepare_borders(display_list: &mut DisplayList, layout_box: &LayoutBox) {
    prepare_border(display_list, layout_box, PhysicalSide::Bottom);
    prepare_border(display_list, layout_box, PhysicalSide::Left);
    prepare_border(display_list, layout_box, PhysicalSide::Right);
    prepare_border(display_list, layout_box, PhysicalSide::Top);
}

/// Prepares the border `side` of `layout_box` for display by converting it to a display command.
fn prepare_border(display_list: &mut DisplayList, layout_box: &LayoutBox, side: PhysicalSide) {
    let cvs = layout_box.computed_values();
    let d = layout_box.dimensions();
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
    let rect = match side {
        PhysicalSide::Bottom => PositionedRect {
            start_x: border_box.start_x,
            start_y: (border_box.start_y + border_box.height() - border_size_px).px(),
            rect: Rect {
                width: border_box.width(),
                height: border_size_px,
            },
        },
        PhysicalSide::Left => PositionedRect {
            start_x: border_box.start_x,
            start_y: border_box.start_y,
            rect: Rect {
                width: border_size_px,
                height: border_box.height(),
            },
        },
        PhysicalSide::Right => PositionedRect {
            start_x: (border_box.start_x + border_box.width() - border_size_px).px(),
            start_y: border_box.start_y,
            rect: Rect {
                width: border_size_px,
                height: border_box.height(),
            },
        },
        PhysicalSide::Top => PositionedRect {
            start_x: border_box.start_x,
            start_y: border_box.start_y,
            rect: Rect {
                width: border_box.width(),
                height: border_size_px,
            },
        },
    };
    display_list.push(DisplayCommand::RectSolidColor(border_color_rgba, rect));
}
