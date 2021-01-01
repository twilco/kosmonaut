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
use crate::layout::containing_block::ContainingBlock;
use crate::layout::flow::OriginRelativeProgression;
use crate::layout::layout_box::LayoutBox;
use crate::layout::rect::Rect;
use crate::style::values::computed::length::CSSPixelLength;
use crate::style::values::CSSFloat;
use std::io::Write;

/// Given a `window` and a `layout_root_box`, perform a layout with the dimensions of the `window`.
pub fn global_layout(
    layout_root_box: &mut LayoutBox,
    inner_window_width: f32,
    inner_window_height: f32,
    scale_factor: f32,
) {
    let writing_mode = layout_root_box.computed_values().writing_mode;
    let direction = layout_root_box.computed_values().direction;
    layout_root_box.layout(LayoutContext::new(ContainingBlock::new(
        Rect {
            start_x: 0.0,
            start_y: 0.0,
            width: CSSPixelLength::new(inner_window_width / scale_factor),
            height: CSSPixelLength::new(inner_window_height / scale_factor),
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
/// TODO: Write a custom derive for this.  A bunch of impls of this are just enums calling
/// `dump_layout_format` on their variants.  This trait will still need to be implemented by hand
/// at the leaves, though (sort of like the Debug trait)
/// https://doc.rust-lang.org/book/ch19-06-macros.html#how-to-write-a-custom-derive-macrok
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
