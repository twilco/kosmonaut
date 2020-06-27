use crate::layout::layout_box::{BoxType, LayoutBox};
use crate::layout::Rect;
use crate::style::values::computed::LineStyle;
use crate::Side;
use cssparser::RGBA;
use gl::texture::TextureId;
use pathfinder_geometry::vector::Vector2F;
use crate::gfx::char::CharHandle;
use crate::gfx::font::FontHandle;
use app_units::Au;

/// Builds list of display commands that should be used to paint the output.
pub fn build_display_list(layout_box: &LayoutBox, char_handle: &CharHandle) -> DisplayList {
    let mut display_list = Vec::new();
    // TODO: Remove the three preceeding lines once text rendering is fixed.
    let font_handle = FontHandle::new();
    let font = font_handle.get_font("Liberation Mono").unwrap();
    char_handle.prepare_char(&mut display_list, 'a', RGBA::new(0, 0, 0 ,1), &font, Au::from_f32_px(12.)).unwrap();
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
    RectSolidColor(RGBA, Rect),
    /// This _could_ be represented as [`RectSolidColor`], but graphics APIs sometimes have a
    /// special background painting capabilities that are more idiomatic, such as OpenGL's
    /// `Clear(COLOR_BUFFER_BIT)` and `ClearColor(r, g, b, a)` APIs.
    ViewportBackground(RGBA),
}

#[derive(Clone, Debug)]
pub struct CharCommand {
    /// The horizontal and vertical distance to the next glyph.
    advance: Vector2F,
    /// Offset from baseline to left/top of glyph.
    bearing: Vector2F,
    /// The char to be rendered.
    ch: char,
    /// The color to render the char as.
    color: RGBA,
    /// The size to render the char as.
    size: Vector2F,
    /// The x and y coordinates of where the glyph origin should be placed on the viewport.
    start_coords: Vector2F,
    // This is the only OpenGL-specific part of state associated with the command.  Kosmonaut is
    // pretty hardcoded to OpenGL, but I _really_ wanted to keep the DisplayCommands renderer-agnostic...
    /// The OpenGL texture associated with this character.
    texture_id: TextureId,
}

impl CharCommand {
    pub fn new(
        advance: Vector2F,
        bearing: Vector2F,
        ch: char,
        color: RGBA,
        size: Vector2F,
        start_coords: Vector2F,
        texture_id: TextureId,
    ) -> Self {
        CharCommand {
            advance,
            bearing,
            ch,
            color,
            size,
            start_coords,
            texture_id,
        }
    }

    pub fn advance(&self) -> Vector2F {
        self.advance
    }

    pub fn bearing(&self) -> Vector2F {
        self.bearing
    }

    pub fn ch(&self) -> char {
        self.ch
    }

    pub fn color(&self) -> RGBA {
        self.color
    }

    pub fn size(&self) -> Vector2F {
        self.size
    }

    pub fn start_coords(&self) -> Vector2F {
        self.start_coords
    }

    pub fn texture_id(&self) -> TextureId {
        self.texture_id
    }
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

    #[allow(clippy::single_match)]
    match layout_box.box_type() {
        BoxType::Block => {
            // Step 2 of painting order
            prepare_block_listitem_block_equiv(display_list, layout_box)
        }
        BoxType::Anonymous | BoxType::Inline => {
            // TODO: Implement other steps of painting order, 3 -> 10
            // println!("skipping render of non-block box")
        }
    }

    for child in layout_box.children() {
        prepare_layout_box(display_list, child);
    }
}

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
    prepare_border(display_list, layout_box, Side::Bottom);
    prepare_border(display_list, layout_box, Side::Left);
    prepare_border(display_list, layout_box, Side::Right);
    prepare_border(display_list, layout_box, Side::Top);
}

/// Prepares the border `side` of `layout_box` for display by converting it to a display command.
fn prepare_border(display_list: &mut DisplayList, layout_box: &LayoutBox, side: Side) {
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
    match side {
        Side::Bottom => display_list.push(DisplayCommand::RectSolidColor(
            border_color_rgba,
            Rect {
                start_x: border_box.start_x,
                start_y: (border_box.start_y + border_box.height - border_size_px).px(),
                width: border_box.width,
                height: border_size_px,
            },
        )),
        Side::Left => display_list.push(DisplayCommand::RectSolidColor(
            border_color_rgba,
            Rect {
                start_x: border_box.start_x,
                start_y: border_box.start_y,
                width: border_size_px,
                height: border_box.height,
            },
        )),
        Side::Right => display_list.push(DisplayCommand::RectSolidColor(
            border_color_rgba,
            Rect {
                start_x: (border_box.start_x + border_box.width - border_size_px).px(),
                start_y: border_box.start_y,
                width: border_size_px,
                height: border_box.height,
            },
        )),
        Side::Top => display_list.push(DisplayCommand::RectSolidColor(
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
