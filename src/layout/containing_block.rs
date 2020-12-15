use crate::layout::rect::Rect;
use crate::style::values::computed::length::CSSPixelLength;
use crate::style::values::computed::{Direction, WritingMode};

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

    /// Gets the logical height of this rectangle (also called the block-size).
    pub fn block_size(&self) -> CSSPixelLength {
        if self.writing_mode.is_horizontal() {
            self.rect.height
        } else {
            self.rect.width
        }
    }

    /// Gets the logical width of this rectangle (also called the inline-size).
    pub fn inline_size(&self) -> CSSPixelLength {
        if self.writing_mode.is_horizontal() {
            self.rect.width
        } else {
            self.rect.height
        }
    }

    pub fn rect(&self) -> &Rect {
        &self.rect
    }

    pub fn writing_mode(&self) -> WritingMode {
        self.writing_mode
    }
}
