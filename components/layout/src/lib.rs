pub mod behavior;
pub mod box_tree;
pub mod containing_block;
pub mod dimensions;
pub mod flow;
pub mod formatting_context;
pub mod layout_box;
pub mod values;

use crate::behavior::BaseLayoutBoxBehavior;
use crate::containing_block::ContainingBlock;
use crate::flow::block::BlockLevelBox;
use crate::flow::inline::{InlineLevelBox, InlineLevelContent};
use crate::flow::OriginRelativeProgression;
use crate::layout_box::LayoutBox;
use cli::DumpLayoutVerbosity;
use dom::tree::NodeData;
use enum_dispatch::enum_dispatch;
use glutin::dpi::PhysicalSize;
use primitives::rect::{PositionedRect, Rect};
use primitives::units::{CSSFloat, CSSPixelLength};
use std::io::Write;
use style::values::computed::WritingMode;

#[macro_use]
extern crate html5ever;
#[macro_use]
extern crate strum_macros;

/// Given a `window` and a `layout_root_box`, perform a layout with the dimensions of the `window`.
pub fn global_layout(
    layout_root_box: &mut LayoutBox,
    viewport: LayoutViewportDimensions,
    scale_factor: f32,
) {
    let writing_mode = layout_root_box.computed_values().writing_mode;
    let direction = layout_root_box.computed_values().direction;
    let layout_viewport_rect = Rect {
        width: CSSPixelLength::new(viewport.width().px() / scale_factor),
        height: CSSPixelLength::new(viewport.height().px() / scale_factor),
    };
    layout_root_box.layout(LayoutContext::new(
        ContainingBlock::new(
            PositionedRect {
                start_x: 0.0,
                start_y: 0.0,
                rect: layout_viewport_rect,
            },
            direction,
            writing_mode,
        ),
        LayoutViewportDimensions::new(layout_viewport_rect),
    ));
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoxComponent {
    Border,
    Margin,
    Padding,
}

#[derive(Copy, Clone, Debug)]
pub struct LayoutContext {
    containing_block: ContainingBlock,
    layout_viewport: LayoutViewportDimensions,
}

impl LayoutContext {
    pub fn new(
        containing_block: ContainingBlock,
        layout_viewport: LayoutViewportDimensions,
    ) -> Self {
        LayoutContext {
            containing_block,
            layout_viewport,
        }
    }

    pub fn block_start_origin_relative_progression(&self) -> OriginRelativeProgression {
        self.containing_block
            .block_start_origin_relative_progression()
    }

    pub fn inline_start_origin_relative_progression(&self) -> OriginRelativeProgression {
        self.containing_block
            .inline_start_origin_relative_progression()
    }

    pub fn layout_viewport_block_size(
        &self,
        relative_to_writing_mode: WritingMode,
    ) -> CSSPixelLength {
        if relative_to_writing_mode.is_horizontal() {
            self.layout_viewport_height()
        } else {
            self.layout_viewport_width()
        }
    }

    #[inline(always)]
    pub fn layout_viewport_width(&self) -> CSSPixelLength {
        self.layout_viewport.width()
    }

    #[inline(always)]
    pub fn layout_viewport_height(&self) -> CSSPixelLength {
        self.layout_viewport.height()
    }
}

/// The dimensions of the viewport into which content is laid out.  This differs from the visual
/// viewport, which changes when users perform certain actions (e.g. pinch to zoom).
///
/// https://wicg.github.io/visual-viewport/#the-visualviewport-interface
/// https://developer.mozilla.org/en-US/docs/Web/CSS/Viewport_concepts
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LayoutViewportDimensions {
    rect: Rect,
}

impl LayoutViewportDimensions {
    pub fn new(rect: Rect) -> Self {
        LayoutViewportDimensions { rect }
    }

    pub fn from_px(width: CSSFloat, height: CSSFloat) -> Self {
        LayoutViewportDimensions {
            rect: Rect {
                width: CSSPixelLength::new(width),
                height: CSSPixelLength::new(height),
            },
        }
    }

    pub fn width(&self) -> CSSPixelLength {
        self.rect.width
    }

    pub fn height(&self) -> CSSPixelLength {
        self.rect.height
    }

    pub fn width_height_px(&self) -> (CSSFloat, CSSFloat) {
        (self.rect.width.px(), self.rect.height.px())
    }
}

impl From<PhysicalSize<u32>> for LayoutViewportDimensions {
    fn from(size: PhysicalSize<u32>) -> Self {
        LayoutViewportDimensions::from_px(size.width as f32, size.height as f32)
    }
}

pub trait Layout {
    fn layout(&mut self, context: LayoutContext);
}

/// Trait describing behavior necessary for dumping the layout tree, used in the `dump-layout`
/// tests and debugging.  Should be implemented by "container"-style entities, such as members
/// of the layout tree, formatting individual struct members via the `DumpLayoutFormat` trait.
pub trait DumpLayout {
    fn dump_layout<W: Write>(
        &self,
        write_to: &mut W,
        indent_spaces: usize,
        verbosity: DumpLayoutVerbosity,
    );
}

/// Trait describing behavior necessary for formatting ones data in preparation for a layout tree
/// dump.
#[enum_dispatch(BlockLevelBox, InlineLevelBox, InlineLevelContent)]
pub trait DumpLayoutFormat {
    fn dump_layout_format(&self) -> String;
}

impl DumpLayoutFormat for CSSFloat {
    fn dump_layout_format(&self) -> String {
        let px = format!("{:.2}", self);
        let mut px_trimmed = px.trim_end_matches('0');
        px_trimmed = px_trimmed.trim_end_matches('.');
        px_trimmed.to_owned()
    }
}

impl DumpLayoutFormat for CSSPixelLength {
    fn dump_layout_format(&self) -> String {
        self.px().dump_layout_format()
    }
}

impl DumpLayoutFormat for NodeData {
    fn dump_layout_format(&self) -> String {
        match self {
            NodeData::Comment(_) => "COMMENT".to_owned(),
            NodeData::Document(_) => "DOCUMENT".to_owned(),
            NodeData::Doctype(_) => "DOCTYPE".to_owned(),
            NodeData::DocumentFragment => "DOCUMENT_FRAGMENT".to_owned(),
            NodeData::Element(element_data) => element_data.name.local.to_uppercase(),
            NodeData::Text(text) => format!("TEXT \"{}\"", text.clone().take().trim()),
            NodeData::ProcessingInstruction(_) => "PROCESSING_INSTRUCTION".to_owned(),
        }
    }
}
