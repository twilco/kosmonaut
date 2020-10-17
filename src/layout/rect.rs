use crate::style::values::computed::length::CSSPixelLength;
use crate::style::values::computed::{Direction, WritingMode};
use crate::style::values::CSSFloat;

#[derive(Clone, Copy, Debug, Default)]
pub struct Rect {
    /// The exact point Where the rectangle begins on the x-axis.
    pub start_x: CSSFloat,
    /// The exact point Where the rectangle begins on the y-axis.
    pub start_y: CSSFloat,
    pub width: CSSPixelLength,
    pub height: CSSPixelLength,
}

impl Rect {
    pub fn expanded_by(self, edge: EdgeSizes) -> Rect {
        Rect {
            start_x: (self.start_x - edge.left).px(),
            start_y: (self.start_y - edge.top).px(),
            width: self.width + edge.left + edge.right,
            height: self.height + edge.top + edge.bottom,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct EdgeSizes {
    pub left: CSSPixelLength,
    pub right: CSSPixelLength,
    pub top: CSSPixelLength,
    pub bottom: CSSPixelLength,
}

impl EdgeSizes {
    pub fn scale_by(&mut self, scale_factor: f32) {
        self.left *= scale_factor;
        self.right *= scale_factor;
        self.top *= scale_factor;
        self.bottom *= scale_factor;
    }
}
