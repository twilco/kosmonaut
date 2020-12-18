use crate::layout::flow::FlowSide;
use crate::layout::rect::{EdgeSizes, Rect};
use crate::layout::BoxComponent;
use crate::style::values::computed::length::CSSPixelLength;
use crate::style::values::computed::{Direction, WritingMode};
use crate::style::values::CSSFloat;
use crate::Side;

/// https://www.w3.org/TR/2018/WD-css-box-3-20181218/#box-model
#[derive(Clone, Copy, Debug, Default)]
pub struct Dimensions {
    // Position of the content area relative to the document origin:
    pub content: Rect,

    // Surrounding edges:
    pub padding: EdgeSizes,
    pub border: EdgeSizes,
    pub margin: EdgeSizes,
}

impl Dimensions {
    pub fn border_size(self, side: Side) -> CSSPixelLength {
        match side {
            Side::Bottom => self.border.bottom,
            Side::Left => self.border.left,
            Side::Right => self.border.right,
            Side::Top => self.border.top,
        }
    }

    /// The area covered by the content area plus padding and borders.
    pub fn border_box(self) -> Rect {
        self.padding_box().expanded_by(self.border)
    }

    /// The area covered by the content area plus its padding.
    pub fn padding_box(self) -> Rect {
        self.content.expanded_by(self.padding)
    }

    /// The area covered by the content area plus padding, borders, and margin.
    // TODO: This will need to change when we implement margin collapsing: http://www.w3.org/TR/CSS2/box.html#collapsing-margins
    pub fn margin_box(self) -> Rect {
        self.border_box().expanded_by(self.margin)
    }

    pub fn scale_edges_by(&mut self, scale_factor: f32) {
        self.padding.scale_by(scale_factor);
        self.border.scale_by(scale_factor);
        self.margin.scale_by(scale_factor);
    }

    pub fn set_height(&mut self, val: CSSPixelLength) {
        self.content.height = val;
    }

    pub fn set_width(&mut self, val: CSSPixelLength) {
        self.content.width = val;
    }

    pub fn set_start_x(&mut self, val: CSSFloat) {
        self.content.start_x = val;
    }

    pub fn set_start_y(&mut self, val: CSSFloat) {
        self.content.start_y = val;
    }

    pub fn set_block_start_coord(&mut self, val: CSSFloat, writing_mode: WritingMode) {
        match writing_mode {
            WritingMode::HorizontalTb => self.set_start_y(val),
            WritingMode::VerticalRl
            | WritingMode::SidewaysRl
            | WritingMode::VerticalLr
            | WritingMode::SidewaysLr => self.set_start_x(val),
        }
    }

    /// The inline-directions for `horizontal-tb` are left-right, so set `start_x` for that
    /// `writing-mode`.  The inline-directions of the other `writing-mode`s are bottom-top, so
    /// set `start_y` for them here.
    ///
    /// https://github.com/twilco/kosmonaut/blob/master/src/layout/dimensions.rs
    pub fn set_inline_start_coord(&mut self, val: CSSFloat, writing_mode: WritingMode) {
        match writing_mode {
            WritingMode::HorizontalTb => self.set_start_x(val),
            WritingMode::VerticalRl
            | WritingMode::SidewaysRl
            | WritingMode::VerticalLr
            | WritingMode::SidewaysLr => self.set_start_y(val),
        }
    }

    pub fn get_content_block_size(&self, writing_mode: WritingMode) -> CSSPixelLength {
        self.get_block_size(None, writing_mode)
    }

    pub fn padding_box_block_size(&self, writing_mode: WritingMode) -> CSSPixelLength {
        self.get_block_size(Some(BoxComponent::Padding), writing_mode)
    }

    pub fn border_box_block_size(&self, writing_mode: WritingMode) -> CSSPixelLength {
        self.get_block_size(Some(BoxComponent::Border), writing_mode)
    }

    pub fn margin_box_block_size(&self, writing_mode: WritingMode) -> CSSPixelLength {
        self.get_block_size(Some(BoxComponent::Margin), writing_mode)
    }

    /// Gets the block size of these dimensions, optionally expanded by:
    ///
    ///  1. Padding only
    ///  2. Padding and borders
    ///  3. Padding, borders, and margins
    ///
    /// If `None`, only the block size of the box content will be returned.
    ///
    /// Note that the block size is also referred to as the logical height.
    fn get_block_size(
        &self,
        expanded_by: Option<BoxComponent>,
        writing_mode: WritingMode,
    ) -> CSSPixelLength {
        match writing_mode {
            WritingMode::HorizontalTb => match expanded_by {
                None => self.content.height,
                Some(BoxComponent::Padding) => self.padding_box().height,
                Some(BoxComponent::Border) => self.border_box().height,
                Some(BoxComponent::Margin) => self.margin_box().height,
            },
            WritingMode::VerticalRl
            | WritingMode::SidewaysRl
            | WritingMode::VerticalLr
            | WritingMode::SidewaysLr => match expanded_by {
                None => self.content.width,
                Some(BoxComponent::Padding) => self.padding_box().width,
                Some(BoxComponent::Border) => self.border_box().width,
                Some(BoxComponent::Margin) => self.margin_box().width,
            },
        }
    }

    pub fn set_block_size(&mut self, val: CSSPixelLength, writing_mode: WritingMode) {
        match writing_mode {
            WritingMode::HorizontalTb => self.content.height = val,
            WritingMode::VerticalRl
            | WritingMode::SidewaysRl
            | WritingMode::VerticalLr
            | WritingMode::SidewaysLr => self.content.width = val,
        }
    }

    /// Note that the inline size is also known as the logical width.
    pub fn get_inline_size(&self, writing_mode: WritingMode) -> CSSPixelLength {
        match writing_mode {
            WritingMode::HorizontalTb => self.content.width,
            WritingMode::VerticalRl
            | WritingMode::SidewaysRl
            | WritingMode::VerticalLr
            | WritingMode::SidewaysLr => self.content.height,
        }
    }

    pub fn set_inline_size(&mut self, val: CSSPixelLength, writing_mode: WritingMode) {
        match writing_mode {
            WritingMode::HorizontalTb => self.content.width = val,
            WritingMode::VerticalRl
            | WritingMode::SidewaysRl
            | WritingMode::VerticalLr
            | WritingMode::SidewaysLr => self.content.height = val,
        }
    }

    // Make sure the `get` and `set` tables stay in sync.  There's probably a better way to share
    // the lookup logic between get and set operations, but copy-paste will do for now.
    pub fn get(
        &self,
        side: FlowSide,
        box_component: BoxComponent,
        writing_mode: WritingMode,
        direction: Direction,
    ) -> CSSPixelLength {
        // Maps to this table: https://drafts.csswg.org/css-writing-modes-4/#logical-to-physical
        match (writing_mode, direction, side) {
            (
                WritingMode::VerticalRl | WritingMode::SidewaysRl | WritingMode::VerticalLr,
                Direction::Ltr,
                FlowSide::InlineStart,
            )
            | (
                WritingMode::VerticalRl | WritingMode::SidewaysRl | WritingMode::VerticalLr,
                Direction::Rtl,
                FlowSide::InlineEnd,
            )
            | (WritingMode::SidewaysLr, Direction::Ltr, FlowSide::InlineEnd)
            | (WritingMode::SidewaysLr, Direction::Rtl, FlowSide::InlineStart)
            | (WritingMode::HorizontalTb, _, FlowSide::BlockStart) => match box_component {
                BoxComponent::Border => self.border.top,
                BoxComponent::Margin => self.margin.top,
                BoxComponent::Padding => self.padding.top,
            },
            (
                WritingMode::VerticalRl | WritingMode::SidewaysRl | WritingMode::VerticalLr,
                Direction::Ltr,
                FlowSide::InlineEnd,
            )
            | (
                WritingMode::VerticalRl | WritingMode::SidewaysRl | WritingMode::VerticalLr,
                Direction::Rtl,
                FlowSide::InlineStart,
            )
            | (WritingMode::SidewaysLr, Direction::Ltr, FlowSide::InlineStart)
            | (WritingMode::SidewaysLr, Direction::Rtl, FlowSide::InlineEnd)
            | (WritingMode::HorizontalTb, _, FlowSide::BlockEnd) => match box_component {
                BoxComponent::Border => self.border.bottom,
                BoxComponent::Margin => self.margin.bottom,
                BoxComponent::Padding => self.padding.bottom,
            },
            (WritingMode::VerticalRl | WritingMode::SidewaysRl, _, FlowSide::BlockEnd)
            | (WritingMode::VerticalLr | WritingMode::SidewaysLr, _, FlowSide::BlockStart)
            | (WritingMode::HorizontalTb, Direction::Ltr, FlowSide::InlineStart)
            | (WritingMode::HorizontalTb, Direction::Rtl, FlowSide::InlineEnd) => {
                match box_component {
                    BoxComponent::Border => self.border.left,
                    BoxComponent::Margin => self.margin.left,
                    BoxComponent::Padding => self.padding.left,
                }
            }
            (WritingMode::VerticalRl | WritingMode::SidewaysRl, _, FlowSide::BlockStart)
            | (WritingMode::VerticalLr | WritingMode::SidewaysLr, _, FlowSide::BlockEnd)
            | (WritingMode::HorizontalTb, Direction::Ltr, FlowSide::InlineEnd)
            | (WritingMode::HorizontalTb, Direction::Rtl, FlowSide::InlineStart) => {
                match box_component {
                    BoxComponent::Border => self.border.right,
                    BoxComponent::Margin => self.margin.right,
                    BoxComponent::Padding => self.padding.right,
                }
            }
        }
    }

    #[inline(always)]
    pub fn set_margin(
        &mut self,
        side: FlowSide,
        val: CSSPixelLength,
        writing_mode: WritingMode,
        direction: Direction,
    ) {
        self.set(side, BoxComponent::Margin, val, writing_mode, direction)
    }

    #[inline(always)]
    pub fn set_border(
        &mut self,
        side: FlowSide,
        val: CSSPixelLength,
        writing_mode: WritingMode,
        direction: Direction,
    ) {
        self.set(side, BoxComponent::Border, val, writing_mode, direction)
    }

    #[inline(always)]
    pub fn set_padding(
        &mut self,
        side: FlowSide,
        val: CSSPixelLength,
        writing_mode: WritingMode,
        direction: Direction,
    ) {
        self.set(side, BoxComponent::Padding, val, writing_mode, direction);
    }

    pub fn set(
        &mut self,
        side: FlowSide,
        box_component: BoxComponent,
        val: CSSPixelLength,
        writing_mode: WritingMode,
        direction: Direction,
    ) {
        // Maps to this table: https://drafts.csswg.org/css-writing-modes-4/#logical-to-physical
        match (writing_mode, direction, side) {
            (
                WritingMode::VerticalRl | WritingMode::SidewaysRl | WritingMode::VerticalLr,
                Direction::Ltr,
                FlowSide::InlineStart,
            )
            | (
                WritingMode::VerticalRl | WritingMode::SidewaysRl | WritingMode::VerticalLr,
                Direction::Rtl,
                FlowSide::InlineEnd,
            )
            | (WritingMode::SidewaysLr, Direction::Ltr, FlowSide::InlineEnd)
            | (WritingMode::SidewaysLr, Direction::Rtl, FlowSide::InlineStart)
            | (WritingMode::HorizontalTb, _, FlowSide::BlockStart) => match box_component {
                BoxComponent::Border => self.border.top = val,
                BoxComponent::Margin => self.margin.top = val,
                BoxComponent::Padding => self.padding.top = val,
            },
            (
                WritingMode::VerticalRl | WritingMode::SidewaysRl | WritingMode::VerticalLr,
                Direction::Ltr,
                FlowSide::InlineEnd,
            )
            | (
                WritingMode::VerticalRl | WritingMode::SidewaysRl | WritingMode::VerticalLr,
                Direction::Rtl,
                FlowSide::InlineStart,
            )
            | (WritingMode::SidewaysLr, Direction::Ltr, FlowSide::InlineStart)
            | (WritingMode::SidewaysLr, Direction::Rtl, FlowSide::InlineEnd)
            | (WritingMode::HorizontalTb, _, FlowSide::BlockEnd) => match box_component {
                BoxComponent::Border => self.border.bottom = val,
                BoxComponent::Margin => self.margin.bottom = val,
                BoxComponent::Padding => self.padding.bottom = val,
            },
            (WritingMode::VerticalRl | WritingMode::SidewaysRl, _, FlowSide::BlockEnd)
            | (WritingMode::VerticalLr | WritingMode::SidewaysLr, _, FlowSide::BlockStart)
            | (WritingMode::HorizontalTb, Direction::Ltr, FlowSide::InlineStart)
            | (WritingMode::HorizontalTb, Direction::Rtl, FlowSide::InlineEnd) => {
                match box_component {
                    BoxComponent::Border => self.border.left = val,
                    BoxComponent::Margin => self.margin.left = val,
                    BoxComponent::Padding => self.padding.left = val,
                }
            }
            (WritingMode::VerticalRl | WritingMode::SidewaysRl, _, FlowSide::BlockStart)
            | (WritingMode::VerticalLr | WritingMode::SidewaysLr, _, FlowSide::BlockEnd)
            | (WritingMode::HorizontalTb, Direction::Ltr, FlowSide::InlineEnd)
            | (WritingMode::HorizontalTb, Direction::Rtl, FlowSide::InlineStart) => {
                match box_component {
                    BoxComponent::Border => self.border.right = val,
                    BoxComponent::Margin => self.margin.right = val,
                    BoxComponent::Padding => self.padding.right = val,
                }
            }
        }
    }
}
