use crate::layout::rect::{EdgeSizes, Rect};
use crate::layout::{BoxComponent, LogicalDirection};
use crate::style::values::computed::length::CSSPixelLength;
use crate::style::values::computed::{Direction, WritingMode};
use crate::style::values::CSSFloat;
use crate::Side;

#[derive(Debug, Clone, Copy)]
pub struct LogicalDimensions {
    writing_mode: WritingMode,
    direction: Direction,
    dimensions: PhysicalDimensions,
}

impl LogicalDimensions {
    pub fn new(writing_mode: WritingMode, direction: Direction) -> LogicalDimensions {
        Self {
            writing_mode,
            direction,
            dimensions: PhysicalDimensions::default(),
        }
    }

    pub fn replace_inner_physical(&mut self, new_dimensions: PhysicalDimensions) {
        self.dimensions = new_dimensions;
    }

    pub fn physical(&self) -> PhysicalDimensions {
        self.dimensions
    }

    pub fn border_size(self, side: Side) -> CSSPixelLength {
        self.dimensions.border_size(side)
    }

    pub fn padding_box(self) -> Rect {
        self.dimensions.content.expanded_by(self.dimensions.padding)
    }

    pub fn border_box(self) -> Rect {
        self.dimensions
            .padding_box()
            .expanded_by(self.dimensions.border)
    }

    pub fn scale_edges_by(&mut self, scale_factor: f32) {
        self.dimensions.scale_edges_by(scale_factor)
    }

    pub fn set_inline_start_coord(&mut self, val: CSSFloat) {
        match self.writing_mode {
            WritingMode::HorizontalTb => self.set_start_x(val),
            WritingMode::VerticalRl
            | WritingMode::SidewaysRl
            | WritingMode::VerticalLr
            | WritingMode::SidewaysLr => self.set_start_y(val),
        }
    }

    pub fn set_phys_height(&mut self, val: CSSPixelLength) {
        self.dimensions.content.height = val;
    }

    pub fn set_phys_width(&mut self, val: CSSPixelLength) {
        self.dimensions.content.width = val;
    }

    pub fn set_start_x(&mut self, val: CSSFloat) {
        self.dimensions.content.start_x = val;
    }

    pub fn set_block_start_coord(&mut self, val: CSSFloat) {
        match self.writing_mode {
            WritingMode::HorizontalTb => self.set_start_y(val),
            WritingMode::VerticalRl
            | WritingMode::SidewaysRl
            | WritingMode::VerticalLr
            | WritingMode::SidewaysLr => self.set_start_x(val),
        }
    }

    pub fn set_start_y(&mut self, val: CSSFloat) {
        self.dimensions.content.start_y = val;
    }

    pub fn get_content_block_size(&self) -> CSSPixelLength {
        self.get_block_size(None)
    }

    pub fn padding_box_block_size(&self) -> CSSPixelLength {
        self.get_block_size(Some(BoxComponent::Padding))
    }

    pub fn border_box_block_size(&self) -> CSSPixelLength {
        self.get_block_size(Some(BoxComponent::Border))
    }

    pub fn margin_box_block_size(&self) -> CSSPixelLength {
        self.get_block_size(Some(BoxComponent::Margin))
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
    fn get_block_size(&self, expanded_by: Option<BoxComponent>) -> CSSPixelLength {
        match self.writing_mode {
            WritingMode::HorizontalTb => match expanded_by {
                None => self.dimensions.content.height,
                Some(BoxComponent::Padding) => self.dimensions.padding_box().height,
                Some(BoxComponent::Border) => self.dimensions.border_box().height,
                Some(BoxComponent::Margin) => self.dimensions.margin_box().height,
            },
            WritingMode::VerticalRl
            | WritingMode::SidewaysRl
            | WritingMode::VerticalLr
            | WritingMode::SidewaysLr => match expanded_by {
                None => self.dimensions.content.width,
                Some(BoxComponent::Padding) => self.dimensions.padding_box().width,
                Some(BoxComponent::Border) => self.dimensions.border_box().width,
                Some(BoxComponent::Margin) => self.dimensions.margin_box().width,
            },
        }
    }

    pub fn set_block_size(&mut self, val: CSSPixelLength) {
        match self.writing_mode {
            WritingMode::HorizontalTb => self.dimensions.content.height = val,
            WritingMode::VerticalRl
            | WritingMode::SidewaysRl
            | WritingMode::VerticalLr
            | WritingMode::SidewaysLr => self.dimensions.content.width = val,
        }
    }

    /// Note that the inline size is also known as the logical width.
    pub fn get_inline_size(&self) -> CSSPixelLength {
        match self.writing_mode {
            WritingMode::HorizontalTb => self.dimensions.content.width,
            WritingMode::VerticalRl
            | WritingMode::SidewaysRl
            | WritingMode::VerticalLr
            | WritingMode::SidewaysLr => self.dimensions.content.height,
        }
    }

    pub fn set_inline_size(&mut self, val: CSSPixelLength) {
        match self.writing_mode {
            WritingMode::HorizontalTb => self.dimensions.content.width = val,
            WritingMode::VerticalRl
            | WritingMode::SidewaysRl
            | WritingMode::VerticalLr
            | WritingMode::SidewaysLr => self.dimensions.content.height = val,
        }
    }

    // Make sure the `get` and `set` tables stay in sync.  There's probably a better way to share
    // the lookup logic between get and set operations, but copy-paste will do for now.
    pub fn get(&self, dir: LogicalDirection, box_component: BoxComponent) -> CSSPixelLength {
        // Maps to this table: https://drafts.csswg.org/css-writing-modes-4/#logical-to-physical
        match (self.writing_mode, self.direction, dir) {
            (
                WritingMode::VerticalRl | WritingMode::SidewaysRl | WritingMode::VerticalLr,
                Direction::Ltr,
                LogicalDirection::InlineStart,
            )
            | (
                WritingMode::VerticalRl | WritingMode::SidewaysRl | WritingMode::VerticalLr,
                Direction::Rtl,
                LogicalDirection::InlineEnd,
            )
            | (WritingMode::SidewaysLr, Direction::Ltr, LogicalDirection::InlineEnd)
            | (WritingMode::SidewaysLr, Direction::Rtl, LogicalDirection::InlineStart)
            | (WritingMode::HorizontalTb, _, LogicalDirection::BlockStart) => match box_component {
                BoxComponent::Border => self.dimensions.border.top,
                BoxComponent::Margin => self.dimensions.margin.top,
                BoxComponent::Padding => self.dimensions.padding.top,
            },
            (
                WritingMode::VerticalRl | WritingMode::SidewaysRl | WritingMode::VerticalLr,
                Direction::Ltr,
                LogicalDirection::InlineEnd,
            )
            | (
                WritingMode::VerticalRl | WritingMode::SidewaysRl | WritingMode::VerticalLr,
                Direction::Rtl,
                LogicalDirection::InlineStart,
            )
            | (WritingMode::SidewaysLr, Direction::Ltr, LogicalDirection::InlineStart)
            | (WritingMode::SidewaysLr, Direction::Rtl, LogicalDirection::InlineEnd)
            | (WritingMode::HorizontalTb, _, LogicalDirection::BlockEnd) => match box_component {
                BoxComponent::Border => self.dimensions.border.bottom,
                BoxComponent::Margin => self.dimensions.margin.bottom,
                BoxComponent::Padding => self.dimensions.padding.bottom,
            },
            (WritingMode::VerticalRl | WritingMode::SidewaysRl, _, LogicalDirection::BlockEnd)
            | (
                WritingMode::VerticalLr | WritingMode::SidewaysLr,
                _,
                LogicalDirection::BlockStart,
            )
            | (WritingMode::HorizontalTb, Direction::Ltr, LogicalDirection::InlineStart)
            | (WritingMode::HorizontalTb, Direction::Rtl, LogicalDirection::InlineEnd) => {
                match box_component {
                    BoxComponent::Border => self.dimensions.border.left,
                    BoxComponent::Margin => self.dimensions.margin.left,
                    BoxComponent::Padding => self.dimensions.padding.left,
                }
            }
            (
                WritingMode::VerticalRl | WritingMode::SidewaysRl,
                _,
                LogicalDirection::BlockStart,
            )
            | (WritingMode::VerticalLr | WritingMode::SidewaysLr, _, LogicalDirection::BlockEnd)
            | (WritingMode::HorizontalTb, Direction::Ltr, LogicalDirection::InlineEnd)
            | (WritingMode::HorizontalTb, Direction::Rtl, LogicalDirection::InlineStart) => {
                match box_component {
                    BoxComponent::Border => self.dimensions.border.right,
                    BoxComponent::Margin => self.dimensions.margin.right,
                    BoxComponent::Padding => self.dimensions.padding.right,
                }
            }
        }
    }

    pub fn set(&mut self, dir: LogicalDirection, box_component: BoxComponent, val: CSSPixelLength) {
        // Maps to this table: https://drafts.csswg.org/css-writing-modes-4/#logical-to-physical
        match (self.writing_mode, self.direction, dir) {
            (
                WritingMode::VerticalRl | WritingMode::SidewaysRl | WritingMode::VerticalLr,
                Direction::Ltr,
                LogicalDirection::InlineStart,
            )
            | (
                WritingMode::VerticalRl | WritingMode::SidewaysRl | WritingMode::VerticalLr,
                Direction::Rtl,
                LogicalDirection::InlineEnd,
            )
            | (WritingMode::SidewaysLr, Direction::Ltr, LogicalDirection::InlineEnd)
            | (WritingMode::SidewaysLr, Direction::Rtl, LogicalDirection::InlineStart)
            | (WritingMode::HorizontalTb, _, LogicalDirection::BlockStart) => match box_component {
                BoxComponent::Border => self.dimensions.border.top = val,
                BoxComponent::Margin => self.dimensions.margin.top = val,
                BoxComponent::Padding => self.dimensions.padding.top = val,
            },
            (
                WritingMode::VerticalRl | WritingMode::SidewaysRl | WritingMode::VerticalLr,
                Direction::Ltr,
                LogicalDirection::InlineEnd,
            )
            | (
                WritingMode::VerticalRl | WritingMode::SidewaysRl | WritingMode::VerticalLr,
                Direction::Rtl,
                LogicalDirection::InlineStart,
            )
            | (WritingMode::SidewaysLr, Direction::Ltr, LogicalDirection::InlineStart)
            | (WritingMode::SidewaysLr, Direction::Rtl, LogicalDirection::InlineEnd)
            | (WritingMode::HorizontalTb, _, LogicalDirection::BlockEnd) => match box_component {
                BoxComponent::Border => self.dimensions.border.bottom = val,
                BoxComponent::Margin => self.dimensions.margin.bottom = val,
                BoxComponent::Padding => self.dimensions.padding.bottom = val,
            },
            (WritingMode::VerticalRl | WritingMode::SidewaysRl, _, LogicalDirection::BlockEnd)
            | (
                WritingMode::VerticalLr | WritingMode::SidewaysLr,
                _,
                LogicalDirection::BlockStart,
            )
            | (WritingMode::HorizontalTb, Direction::Ltr, LogicalDirection::InlineStart)
            | (WritingMode::HorizontalTb, Direction::Rtl, LogicalDirection::InlineEnd) => {
                match box_component {
                    BoxComponent::Border => self.dimensions.border.left = val,
                    BoxComponent::Margin => self.dimensions.margin.left = val,
                    BoxComponent::Padding => self.dimensions.padding.left = val,
                }
            }
            (
                WritingMode::VerticalRl | WritingMode::SidewaysRl,
                _,
                LogicalDirection::BlockStart,
            )
            | (WritingMode::VerticalLr | WritingMode::SidewaysLr, _, LogicalDirection::BlockEnd)
            | (WritingMode::HorizontalTb, Direction::Ltr, LogicalDirection::InlineEnd)
            | (WritingMode::HorizontalTb, Direction::Rtl, LogicalDirection::InlineStart) => {
                match box_component {
                    BoxComponent::Border => self.dimensions.border.right = val,
                    BoxComponent::Margin => self.dimensions.margin.right = val,
                    BoxComponent::Padding => self.dimensions.padding.right = val,
                }
            }
        }
    }
}

/// https://www.w3.org/TR/2018/WD-css-box-3-20181218/#box-model
#[derive(Clone, Copy, Debug, Default)]
pub struct PhysicalDimensions {
    // Position of the content area relative to the document origin:
    pub content: Rect,

    // Surrounding edges:
    pub padding: EdgeSizes,
    pub border: EdgeSizes,
    pub margin: EdgeSizes,
}

impl PhysicalDimensions {
    pub fn border_size(self, side: Side) -> CSSPixelLength {
        match side {
            Side::Bottom => self.border.bottom,
            Side::Left => self.border.left,
            Side::Right => self.border.right,
            Side::Top => self.border.top,
        }
    }

    /// The area covered by the content area plus its padding.
    pub fn padding_box(self) -> Rect {
        self.content.expanded_by(self.padding)
    }

    /// The area covered by the content area plus padding and borders.
    pub fn border_box(self) -> Rect {
        self.padding_box().expanded_by(self.border)
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
}
