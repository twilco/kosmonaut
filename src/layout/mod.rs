pub mod behavior;
pub mod box_tree;
pub mod containing_block;
pub mod dimensions;
pub mod flow;
pub mod formatting_context;
pub mod layout_box;
pub mod rect;
pub mod values;

use crate::cli::DumpLayoutVerbosity;
use crate::dom::tree::NodeData;
use crate::layout::behavior::BaseLayoutBoxBehavior;
use crate::layout::containing_block::ContainingBlock;
use crate::layout::flow::block::BlockLevelBox;
use crate::layout::flow::inline::InlineLevelBox;
use crate::layout::flow::inline::InlineLevelContent;
use crate::layout::flow::OriginRelativeProgression;
use crate::layout::layout_box::LayoutBox;
use crate::layout::rect::Rect;
use crate::style::values::computed::length::CSSPixelLength;
use crate::style::values::CSSFloat;
use enum_dispatch::enum_dispatch;
use glutin::dpi::PhysicalSize;
use std::io::Write;

/// Given a `window` and a `layout_root_box`, perform a layout with the dimensions of the `window`.
pub fn global_layout(
    layout_root_box: &mut LayoutBox,
    viewport: LayoutViewportDimensions,
    scale_factor: f32,
) {
    let writing_mode = layout_root_box.computed_values().writing_mode;
    let direction = layout_root_box.computed_values().direction;
    layout_root_box.layout(LayoutContext::new(ContainingBlock::new(
        Rect {
            start_x: 0.0,
            start_y: 0.0,
            width: CSSPixelLength::new(viewport.width.px() / scale_factor),
            height: CSSPixelLength::new(viewport.height.px() / scale_factor),
        },
        direction,
        writing_mode,
    )));
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
}

impl LayoutContext {
    pub fn new(containing_block: ContainingBlock) -> Self {
        LayoutContext { containing_block }
    }

    pub fn inline_start_origin_relative_progression(&self) -> OriginRelativeProgression {
        self.containing_block
            .inline_start_origin_relative_progression()
    }

    pub fn block_start_origin_relative_progression(&self) -> OriginRelativeProgression {
        self.containing_block
            .block_start_origin_relative_progression()
    }
}

/// The dimensions of the viewport into which content is laid out.  This differs from the visual
/// viewport, which changes when users perform certain actions (e.g. pinch to zoom).
///
/// https://wicg.github.io/visual-viewport/#the-visualviewport-interface
/// https://developer.mozilla.org/en-US/docs/Web/CSS/Viewport_concepts
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LayoutViewportDimensions {
    width: CSSPixelLength,
    height: CSSPixelLength,
}

impl LayoutViewportDimensions {
    pub fn new(width: CSSPixelLength, height: CSSPixelLength) -> Self {
        LayoutViewportDimensions { width, height }
    }

    pub fn new_px(width: CSSFloat, height: CSSFloat) -> Self {
        LayoutViewportDimensions {
            width: CSSPixelLength::new(width),
            height: CSSPixelLength::new(height),
        }
    }

    pub fn width(&self) -> CSSPixelLength {
        self.width
    }

    pub fn height(&self) -> CSSPixelLength {
        self.height
    }

    pub fn width_height_px(&self) -> (CSSFloat, CSSFloat) {
        (self.width.px(), self.height.px())
    }
}

impl From<PhysicalSize<u32>> for LayoutViewportDimensions {
    fn from(size: PhysicalSize<u32>) -> Self {
        LayoutViewportDimensions::new_px(size.width as f32, size.height as f32)
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
