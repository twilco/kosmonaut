use crate::flow::OriginRelativeProgression;
use primitives::rect::PositionedRect;
use primitives::units::CSSFloat;
use primitives::units::CSSPixelLength;
use style::values::computed::{Direction, WritingMode};

/// A containing block is a pairing of a `Rect` and a writing mode that is associated with this
/// block, which is used in determining flow-relative directions for children of this containing
/// block.
///
/// https://www.w3.org/TR/2011/REC-CSS2-20110607/visudet.html#containing-block-details
/// https://drafts.csswg.org/css-writing-modes-4/#logical-direction-layout
#[derive(Copy, Clone, Debug)]
pub struct ContainingBlock {
    positioned_rect: PositionedRect,
    direction: Direction,
    writing_mode: WritingMode,
}

impl ContainingBlock {
    pub fn new(
        positioned_rect: PositionedRect,
        direction: Direction,
        writing_mode: WritingMode,
    ) -> Self {
        Self {
            positioned_rect,
            direction,
            writing_mode,
        }
    }

    pub fn positioned_rect(&self) -> &PositionedRect {
        &self.positioned_rect
    }

    pub fn width(&self) -> CSSPixelLength {
        self.positioned_rect.width()
    }

    pub fn height(&self) -> CSSPixelLength {
        self.positioned_rect.height()
    }

    pub fn writing_mode(&self) -> WritingMode {
        self.writing_mode
    }

    pub fn self_relative_block_size(&self) -> CSSPixelLength {
        self.block_size(self.writing_mode)
    }

    /// Gets the logical height of this rectangle (also called the block-size).
    pub fn block_size(&self, writing_mode: WritingMode) -> CSSPixelLength {
        match writing_mode {
            WritingMode::HorizontalTb => self.positioned_rect.height(),
            WritingMode::VerticalRl
            | WritingMode::SidewaysRl
            | WritingMode::VerticalLr
            | WritingMode::SidewaysLr => self.positioned_rect.width(),
        }
    }

    pub fn self_relative_block_start_coord(&self) -> CSSFloat {
        self.block_start_coord(self.writing_mode)
    }

    /// Gets the block-direction start coordinate of this containing block.
    pub fn block_start_coord(&self, writing_mode: WritingMode) -> CSSFloat {
        match writing_mode {
            WritingMode::HorizontalTb => self.positioned_rect.start_y,
            WritingMode::VerticalRl
            | WritingMode::SidewaysRl
            | WritingMode::VerticalLr
            | WritingMode::SidewaysLr => self.positioned_rect.start_x,
        }
    }

    pub fn block_start_origin_relative_progression(&self) -> OriginRelativeProgression {
        OriginRelativeProgression::block_start_origin_relative_direction(self.writing_mode())
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn self_relative_inline_size(&self) -> CSSPixelLength {
        self.inline_size(self.writing_mode)
    }

    /// Gets the logical width of this rectangle (also called the inline-size).
    pub fn inline_size(&self, writing_mode: WritingMode) -> CSSPixelLength {
        match writing_mode {
            WritingMode::HorizontalTb => self.positioned_rect.width(),
            WritingMode::VerticalRl
            | WritingMode::SidewaysRl
            | WritingMode::VerticalLr
            | WritingMode::SidewaysLr => self.positioned_rect.height(),
        }
    }

    pub fn self_relative_inline_start_coord(&self) -> CSSFloat {
        self.inline_start_coord(self.writing_mode)
    }

    /// Gets the inline-direction start coordinate of this containing block.
    pub fn inline_start_coord(&self, writing_mode: WritingMode) -> CSSFloat {
        match writing_mode {
            WritingMode::HorizontalTb => self.positioned_rect.start_x,
            WritingMode::VerticalRl
            | WritingMode::SidewaysRl
            | WritingMode::VerticalLr
            | WritingMode::SidewaysLr => self.positioned_rect.start_y,
        }
    }

    pub fn inline_start_origin_relative_progression(&self) -> OriginRelativeProgression {
        OriginRelativeProgression::inline_start_origin_relative_direction(
            self.writing_mode(),
            self.direction(),
        )
    }
}
