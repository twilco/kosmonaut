pub mod block;
pub mod inline;

use crate::apply_page_relative_properties_base_box_passthrough_impls;
use crate::dom::tree::NodeRef;
use crate::layout::behavior::{ApplyPageRelativeProperties, BaseLayoutBoxBehavior};
use crate::layout::containing_block::ContainingBlock;
use crate::layout::dimensions::Dimensions;
use crate::layout::formatting_context::FormattingContextRef;
use crate::layout::layout_box::{BaseBox, LayoutBox};
use crate::layout::DumpLayoutFormat;
use crate::layout_box_behavior_base_box_passthrough_impls;
use crate::style::values::computed::Direction;
use crate::style::values::computed::{ComputedValues, WritingMode};
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

impl BaseLayoutBoxBehavior for BlockContainer {
    layout_box_behavior_base_box_passthrough_impls!();
}

impl ApplyPageRelativeProperties for BlockContainer {
    apply_page_relative_properties_base_box_passthrough_impls!();
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

/// The direction in which block-level boxes and line boxes stack within a block container.
/// The block flow direction is determined by the `writing-mode` property.
///
/// https://drafts.csswg.org/css-writing-modes-4/#block-flow-direction
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BlockFlowDirection {
    TopToBottom,
    RightToLeft,
    LeftToRight,
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

/// Represents the direction boxes progress physically (relative to the origin).  For example,
/// in a `writing-mode: horizontal-tb; direction: ltr;` layout, inline progression is left-to-right,
/// moving `AwayFromOrigin` (which is an (x, y) of (0, 0)).  Conversely, a `writing-mode` and
/// `direction` resulting in right-to-left or bottom-to-top progression would be progression
/// `TowardsOrigin`.
///
/// This has implications in layout when calculating box (x, y) position.
///
/// This table showing `writing-mode` and `direction` to logical and physical direction is relevant:
/// https://drafts.csswg.org/css-writing-modes-4/#logical-to-physical
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OriginRelativeProgression {
    AwayFromOrigin,
    TowardsOrigin,
}

impl OriginRelativeProgression {
    /// Determine whether or not the given `writing-mode` and `direction` represents an
    /// `AwayFromOrigin` or `TowardsOrigin` progression for the inline-start direction.
    ///
    /// Maps to https://drafts.csswg.org/css-writing-modes-4/#logical-to-physical.
    pub fn inline_start_origin_relative_direction(
        writing_mode: WritingMode,
        direction: Direction,
    ) -> OriginRelativeProgression {
        match (writing_mode, direction) {
            // left-to-right and top-to-bottom start-to-end flows are `AwayFromOrigin` progressions.
            (
                WritingMode::HorizontalTb
                | WritingMode::VerticalRl
                | WritingMode::VerticalLr
                | WritingMode::SidewaysRl,
                Direction::Ltr,
            )
            | (WritingMode::SidewaysLr, Direction::Rtl) => {
                OriginRelativeProgression::AwayFromOrigin
            }
            // right-to-left and bottom-to-top start-to-end flows are `TowardsOrigin` progression.
            (
                WritingMode::HorizontalTb
                | WritingMode::VerticalRl
                | WritingMode::VerticalLr
                | WritingMode::SidewaysRl,
                Direction::Rtl,
            )
            | (WritingMode::SidewaysLr, Direction::Ltr) => OriginRelativeProgression::TowardsOrigin,
        }
    }

    /// Determine whether or not the given `writing-mode` represents an `AwayFromOrigin` or
    /// `TowardsOrigin` progression.  Determining progression for the block-start direction only requires
    /// the `writing-mode`, whereas determining progression for the inline direction requires both
    /// the `writing-mode` and the `direction`.
    ///
    /// Maps to https://drafts.csswg.org/css-writing-modes-4/#logical-to-physical.
    pub fn block_start_origin_relative_direction(
        writing_mode: WritingMode,
    ) -> OriginRelativeProgression {
        match writing_mode {
            // left-to-right and top-to-bottom start-to-end flows are `AwayFromOrigin` progressions.
            WritingMode::HorizontalTb | WritingMode::VerticalLr | WritingMode::SidewaysLr => {
                OriginRelativeProgression::AwayFromOrigin
            }
            // right-to-left start-to-end flows are `TowardsOrigin` progressions.
            WritingMode::VerticalRl | WritingMode::SidewaysRl => {
                OriginRelativeProgression::TowardsOrigin
            }
        }
    }
}
