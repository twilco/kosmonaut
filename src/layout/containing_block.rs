use crate::layout::rect::Rect;
use crate::style::values::computed::length::CSSPixelLength;
use crate::style::values::computed::{Direction, WritingMode};
use crate::style::values::CSSFloat;

/// A containing block is a pairing of a `Rect` and a writing mode that is associated with this
/// block, which is used in determining flow-relative directions for children of this containing
/// block.
///
/// https://www.w3.org/TR/2011/REC-CSS2-20110607/visudet.html#containing-block-details
/// https://drafts.csswg.org/css-writing-modes-4/#logical-direction-layout
#[derive(Copy, Clone, Debug)]
pub struct ContainingBlock {
    rect: Rect,
    direction: Direction,
    writing_mode: WritingMode,
}

impl ContainingBlock {
    pub fn new(rect: Rect, direction: Direction, writing_mode: WritingMode) -> Self {
        Self {
            rect,
            direction,
            writing_mode,
        }
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn self_relative_block_size(&self) -> CSSPixelLength {
        self.block_size(self.writing_mode)
    }

    /// Gets the logical height of this rectangle (also called the block-size).
    pub fn block_size(&self, writing_mode: WritingMode) -> CSSPixelLength {
        match writing_mode {
            WritingMode::HorizontalTb => self.rect.height,
            WritingMode::VerticalRl
            | WritingMode::SidewaysRl
            | WritingMode::VerticalLr
            | WritingMode::SidewaysLr => self.rect.width,
        }
    }

    pub fn self_relative_block_start_coord(&self) -> CSSFloat {
        self.block_start_coord(self.writing_mode)
    }

    /// Gets the block-direction start coordinate of this containing block.
    pub fn block_start_coord(&self, writing_mode: WritingMode) -> CSSFloat {
        match writing_mode {
            WritingMode::HorizontalTb => self.rect.start_y,
            WritingMode::VerticalRl
            | WritingMode::SidewaysRl
            | WritingMode::VerticalLr
            | WritingMode::SidewaysLr => self.rect.start_x,
        }
    }

    pub fn self_relative_inline_size(&self) -> CSSPixelLength {
        self.inline_size(self.writing_mode)
    }

    /// Gets the logical width of this rectangle (also called the inline-size).
    pub fn inline_size(&self, writing_mode: WritingMode) -> CSSPixelLength {
        match writing_mode {
            WritingMode::HorizontalTb => self.rect.width,
            WritingMode::VerticalRl
            | WritingMode::SidewaysRl
            | WritingMode::VerticalLr
            | WritingMode::SidewaysLr => self.rect.height,
        }
    }

    pub fn self_relative_inline_start_coord(&self) -> CSSFloat {
        self.inline_start_coord(self.writing_mode)
    }

    /// Gets the inline-direction start coordinate of this containing block.
    pub fn inline_start_coord(&self, writing_mode: WritingMode) -> CSSFloat {
        match writing_mode {
            WritingMode::HorizontalTb => self.rect.start_x,
            WritingMode::VerticalRl
            | WritingMode::SidewaysRl
            | WritingMode::VerticalLr
            | WritingMode::SidewaysLr => self.rect.start_y,
        }
    }

    pub fn rect(&self) -> &Rect {
        &self.rect
    }

    pub fn writing_mode(&self) -> WritingMode {
        self.writing_mode
    }
}
