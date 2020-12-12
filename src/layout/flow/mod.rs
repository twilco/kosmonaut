pub mod block;
pub mod inline;

use crate::base_box_passthrough_impls;
use crate::dom::tree::NodeRef;
use crate::layout::containing_block::ContainingBlock;
use crate::layout::dimensions::Dimensions;
use crate::layout::flow::block::AnonymousBlockBox;
use crate::layout::flow::inline::{AnonymousInlineBox, InlineBox, TextRun};
use crate::layout::formatting_context::FormattingContextRef;
use crate::layout::layout_box::{BaseBox, LayoutBox};
use crate::layout::DumpLayoutFormat;
use crate::style::values::computed::length::{
    CSSPixelLength, LengthPercentage, LengthPercentageOrAuto,
};
use crate::style::values::computed::ComputedValues;
use crate::style::values::computed::Direction;
use crate::style::values::used::ToPx;
use accountable_refcell::Ref;

/// A box that contains either contains only inline-level boxes participating in an inline
/// formatting context, or contains only block-level boxes participating in a block formatting
/// context (possibly generating anonymous block boxes to ensure this constraint, as defined in
/// [CSS2§9.2.1.1](https://www.w3.org/TR/CSS2/visuren.html#anonymous-block-level).
///
/// https://drafts.csswg.org/css-display-3/#block-container
#[derive(Clone, Debug)]
pub struct BlockContainer {
    base: BaseBox,
    children: Vec<LayoutBox>,
}

impl BlockContainer {
    base_box_passthrough_impls!();

    pub fn new(node: NodeRef, fc: FormattingContextRef) -> Self {
        BlockContainer {
            base: BaseBox::new(node, fc),
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, new_child: LayoutBox) {
        self.children.push(new_child)
    }

    pub fn children(&self) -> &Vec<LayoutBox> {
        &self.children
    }

    pub fn children_mut(&mut self) -> &mut Vec<LayoutBox> {
        &mut self.children
    }
}

impl DumpLayoutFormat for BlockContainer {
    fn dump_layout_format(&self) -> String {
        let node_data = self.node().data().dump_layout_format();
        if node_data.is_empty() {
            "BlockContainer".to_string()
        } else {
            format!("{} {}", node_data, "BlockContainer")
        }
    }
}

/// Sides relative to the flow of the page, rather than physical sides (e.g. left, top, ...).
///
/// https://drafts.csswg.org/css-writing-modes-4/#logical-directions
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FlowSide {
    BlockStart,
    BlockEnd,
    InlineStart,
    InlineEnd,
}
