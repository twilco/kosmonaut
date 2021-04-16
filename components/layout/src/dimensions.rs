use crate::BoxComponent;
use primitives::rect::{EdgeSizes, PositionedRect};
use primitives::sides::{FlowSide, PhysicalSide};
use primitives::units::CSSFloat;
use primitives::units::CSSPixelLength;
use style::values::computed::{Direction, WritingMode};

/// https://www.w3.org/TR/2018/WD-css-box-3-20181218/#box-model
#[derive(Clone, Copy, Debug, Default)]
pub struct Dimensions {
    /// Position and size of these dimensions.  This position is of the content area (i.e. not
    /// including margin, border, or padding) relative to the document origin.
    pub(super) content: PositionedRect,
    pub(super) padding: EdgeSizes,
    pub(super) border: EdgeSizes,
    pub(super) margin: EdgeSizes,
}

impl Dimensions {
    fn expanded_by(self, other: Dimensions) -> Dimensions {
        Dimensions {
            content: self.content.expanded_by_rect(other.content),
            padding: self.padding.expanded_by_edges(other.padding),
            border: self.border.expanded_by_edges(other.border),
            margin: self.margin.expanded_by_edges(other.margin),
        }
    }

    pub fn border_size(self, side: PhysicalSide) -> CSSPixelLength {
        match side {
            PhysicalSide::Bottom => self.border.bottom,
            PhysicalSide::Left => self.border.left,
            PhysicalSide::Right => self.border.right,
            PhysicalSide::Top => self.border.top,
        }
    }

    /// The area covered by the content area plus padding and borders.
    pub fn border_box(self) -> PositionedRect {
        self.padding_box().expanded_by_edges(self.border)
    }

    /// The area covered by the content area plus its padding.
    fn padding_box(self) -> PositionedRect {
        self.content.expanded_by_edges(self.padding)
    }

    /// The area covered by the content area plus padding, borders, and margin.
    // TODO: This will need to change when we implement margin collapsing: http://www.w3.org/TR/CSS2/box.html#collapsing-margins
    fn margin_box(self) -> PositionedRect {
        self.border_box().expanded_by_edges(self.margin)
    }

    pub(super) fn set_height(&mut self, val: CSSPixelLength) {
        self.content.set_height(val);
    }

    pub(super) fn set_width(&mut self, val: CSSPixelLength) {
        self.content.set_width(val);
    }

    fn start_x(&self) -> CSSFloat {
        self.content.start_x
    }

    fn set_start_x(&mut self, val: CSSFloat) {
        self.content.start_x = val;
    }

    fn start_y(&self) -> CSSFloat {
        self.content.start_y
    }

    fn set_start_y(&mut self, val: CSSFloat) {
        self.content.start_y = val;
    }

    pub(super) fn get_block_start_coord(&mut self, writing_mode: WritingMode) -> CSSFloat {
        match writing_mode {
            WritingMode::HorizontalTb => self.start_y(),
            WritingMode::VerticalRl
            | WritingMode::SidewaysRl
            | WritingMode::VerticalLr
            | WritingMode::SidewaysLr => self.start_x(),
        }
    }

    pub(super) fn set_block_start_coord(&mut self, val: CSSFloat, writing_mode: WritingMode) {
        match writing_mode {
            WritingMode::HorizontalTb => self.set_start_y(val),
            WritingMode::VerticalRl
            | WritingMode::SidewaysRl
            | WritingMode::VerticalLr
            | WritingMode::SidewaysLr => self.set_start_x(val),
        }
    }

    fn add_to_block_start_coord(&mut self, val: CSSFloat, writing_mode: WritingMode) {
        let block_start_coord = self.get_block_start_coord(writing_mode);
        self.set_block_start_coord(val + block_start_coord, writing_mode)
    }

    /// The inline-directions for `horizontal-tb` are left-right, so set `start_x` for that
    /// `writing-mode`.  The inline-directions of the other `writing-mode`s are bottom-top, so
    /// set `start_y` for them here.
    ///
    /// https://github.com/twilco/kosmonaut/blob/master/src/layout/dimensions.rs
    pub(super) fn set_inline_start_coord(&mut self, val: CSSFloat, writing_mode: WritingMode) {
        match writing_mode {
            WritingMode::HorizontalTb => self.set_start_x(val),
            WritingMode::VerticalRl
            | WritingMode::SidewaysRl
            | WritingMode::VerticalLr
            | WritingMode::SidewaysLr => self.set_start_y(val),
        }
    }

    pub(super) fn content_box_block_size(&self, writing_mode: WritingMode) -> CSSPixelLength {
        self.get_block_size(None, writing_mode)
    }

    fn padding_box_block_size(&self, writing_mode: WritingMode) -> CSSPixelLength {
        self.get_block_size(Some(BoxComponent::Padding), writing_mode)
    }

    fn padding_box_inline_size(&self, writing_mode: WritingMode) -> CSSPixelLength {
        self.get_inline_size(Some(BoxComponent::Padding), writing_mode)
    }

    fn border_box_block_size(&self, writing_mode: WritingMode) -> CSSPixelLength {
        self.get_block_size(Some(BoxComponent::Border), writing_mode)
    }

    pub(super) fn border_box_inline_size(&self, writing_mode: WritingMode) -> CSSPixelLength {
        self.get_inline_size(Some(BoxComponent::Border), writing_mode)
    }

    pub(super) fn margin_box_block_size(&self, writing_mode: WritingMode) -> CSSPixelLength {
        self.get_block_size(Some(BoxComponent::Margin), writing_mode)
    }

    fn margin_box_inline_size(&self, writing_mode: WritingMode) -> CSSPixelLength {
        self.get_inline_size(Some(BoxComponent::Margin), writing_mode)
    }

    /// Gets the block-size of these dimensions, optionally expanded by:
    ///
    ///  1. Padding only
    ///  2. Padding and borders
    ///  3. Padding, borders, and margins
    ///
    /// If `None`, only the block-size of the box content will be returned.
    ///
    /// Note that the block-size is also referred to as the logical height.
    fn get_block_size(
        &self,
        expanded_by: Option<BoxComponent>,
        writing_mode: WritingMode,
    ) -> CSSPixelLength {
        match writing_mode {
            WritingMode::HorizontalTb => match expanded_by {
                None => self.content.height(),
                Some(BoxComponent::Padding) => self.padding_box().height(),
                Some(BoxComponent::Border) => self.border_box().height(),
                Some(BoxComponent::Margin) => self.margin_box().height(),
            },
            WritingMode::VerticalRl
            | WritingMode::SidewaysRl
            | WritingMode::VerticalLr
            | WritingMode::SidewaysLr => match expanded_by {
                None => self.content.width(),
                Some(BoxComponent::Padding) => self.padding_box().width(),
                Some(BoxComponent::Border) => self.border_box().width(),
                Some(BoxComponent::Margin) => self.margin_box().width(),
            },
        }
    }

    pub(super) fn add_to_block_size(&mut self, val: CSSPixelLength, writing_mode: WritingMode) {
        let current_block_size = self.get_block_size(None, writing_mode);
        self.set_block_size(current_block_size + val, writing_mode)
    }

    fn set_block_size(&mut self, val: CSSPixelLength, writing_mode: WritingMode) {
        match writing_mode {
            WritingMode::HorizontalTb => self.content.set_height(val),
            WritingMode::VerticalRl
            | WritingMode::SidewaysRl
            | WritingMode::VerticalLr
            | WritingMode::SidewaysLr => self.content.set_width(val),
        }
    }

    /// Gets the inline-size of these dimensions, optionally expanded by:
    ///
    ///  1. Padding only
    ///  2. Padding and borders
    ///  3. Padding, borders, and margins
    ///
    /// If `None`, only the inline-size of the box content will be returned.
    ///
    /// Note that the inline-size is also referred to as the logical width.
    fn get_inline_size(
        &self,
        expanded_by: Option<BoxComponent>,
        writing_mode: WritingMode,
    ) -> CSSPixelLength {
        match writing_mode {
            WritingMode::HorizontalTb => match expanded_by {
                None => self.content.width(),
                Some(BoxComponent::Padding) => self.padding_box().width(),
                Some(BoxComponent::Border) => self.border_box().width(),
                Some(BoxComponent::Margin) => self.margin_box().width(),
            },
            WritingMode::VerticalRl
            | WritingMode::SidewaysRl
            | WritingMode::VerticalLr
            | WritingMode::SidewaysLr => match expanded_by {
                None => self.content.height(),
                Some(BoxComponent::Padding) => self.padding_box().height(),
                Some(BoxComponent::Border) => self.border_box().height(),
                Some(BoxComponent::Margin) => self.margin_box().height(),
            },
        }
    }

    pub(super) fn set_inline_size(&mut self, val: CSSPixelLength, writing_mode: WritingMode) {
        match writing_mode {
            WritingMode::HorizontalTb => self.content.set_width(val),
            WritingMode::VerticalRl
            | WritingMode::SidewaysRl
            | WritingMode::VerticalLr
            | WritingMode::SidewaysLr => self.content.set_height(val),
        }
    }

    // Make sure the `get` and `set` tables stay in sync.  There's probably a better way to share
    // the lookup logic between get and set operations, but copy-paste will do for now.
    pub(super) fn get(
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

    /// Get the sum of the margin, border, and padding box components for the given `FlowSide`.
    pub(super) fn get_mbp(
        &self,
        side: FlowSide,
        writing_mode: WritingMode,
        direction: Direction,
    ) -> CSSPixelLength {
        self.get(side, BoxComponent::Margin, writing_mode, direction)
            + self.get(side, BoxComponent::Border, writing_mode, direction)
            + self.get(side, BoxComponent::Padding, writing_mode, direction)
    }

    #[inline(always)]
    pub(super) fn set_margin(
        &mut self,
        side: FlowSide,
        val: CSSPixelLength,
        writing_mode: WritingMode,
        direction: Direction,
    ) {
        self.set(side, BoxComponent::Margin, val, writing_mode, direction)
    }

    fn get_margin_physical(&self, side: PhysicalSide) -> CSSPixelLength {
        match side {
            PhysicalSide::Bottom => self.margin.bottom,
            PhysicalSide::Left => self.margin.left,
            PhysicalSide::Right => self.margin.right,
            PhysicalSide::Top => self.margin.top,
        }
    }

    #[inline(always)]
    pub(super) fn set_margin_physical(&mut self, side: PhysicalSide, val: CSSPixelLength) {
        match side {
            PhysicalSide::Bottom => self.margin.bottom = val,
            PhysicalSide::Left => self.margin.left = val,
            PhysicalSide::Right => self.margin.right = val,
            PhysicalSide::Top => self.margin.top = val,
        }
    }

    fn get_border_physical(&self, side: PhysicalSide) -> CSSPixelLength {
        match side {
            PhysicalSide::Bottom => self.border.bottom,
            PhysicalSide::Left => self.border.left,
            PhysicalSide::Right => self.border.right,
            PhysicalSide::Top => self.border.top,
        }
    }

    #[inline(always)]
    pub(super) fn set_border(
        &mut self,
        side: FlowSide,
        val: CSSPixelLength,
        writing_mode: WritingMode,
        direction: Direction,
    ) {
        self.set(side, BoxComponent::Border, val, writing_mode, direction)
    }

    #[inline(always)]
    fn set_border_physical(&mut self, side: PhysicalSide, val: CSSPixelLength) {
        match side {
            PhysicalSide::Bottom => self.border.bottom = val,
            PhysicalSide::Left => self.border.left = val,
            PhysicalSide::Right => self.border.right = val,
            PhysicalSide::Top => self.border.top = val,
        }
    }

    fn get_padding_physical(&self, side: PhysicalSide) -> CSSPixelLength {
        match side {
            PhysicalSide::Bottom => self.padding.bottom,
            PhysicalSide::Left => self.padding.left,
            PhysicalSide::Right => self.padding.right,
            PhysicalSide::Top => self.padding.top,
        }
    }

    #[inline(always)]
    pub(super) fn set_padding(
        &mut self,
        side: FlowSide,
        val: CSSPixelLength,
        writing_mode: WritingMode,
        direction: Direction,
    ) {
        self.set(side, BoxComponent::Padding, val, writing_mode, direction);
    }

    #[inline(always)]
    fn set_padding_physical(&mut self, side: PhysicalSide, val: CSSPixelLength) {
        match side {
            PhysicalSide::Bottom => self.padding.bottom = val,
            PhysicalSide::Left => self.padding.left = val,
            PhysicalSide::Right => self.padding.right = val,
            PhysicalSide::Top => self.padding.top = val,
        }
    }

    fn set(
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
