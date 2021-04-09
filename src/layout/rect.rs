use crate::style::values::computed::length::CSSPixelLength;
use crate::style::values::CSSFloat;

#[derive(Clone, Copy, Debug, Default)]
pub struct PositionedRect {
    /// The exact point where the rectangle begins on the x-axis.
    pub start_x: CSSFloat,
    /// The exact point where the rectangle begins on the y-axis.
    pub start_y: CSSFloat,
    pub rect: Rect,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rect {
    pub width: CSSPixelLength,
    pub height: CSSPixelLength,
}

impl Rect {
    pub fn expanded_by_edges(self, edge: EdgeSizes) -> Rect {
        Rect {
            width: self.width + edge.left + edge.right,
            height: self.height + edge.top + edge.bottom,
        }
    }

    pub fn expanded_by_rect(self, other: Rect) -> Rect {
        Rect {
            width: self.width + other.width,
            height: self.height + other.height,
        }
    }

    pub fn scaled_by(&self, scale_factor: f32) -> Rect {
        Rect {
            width: self.width * scale_factor,
            height: self.height * scale_factor,
        }
    }
}

impl PositionedRect {
    pub fn width(&self) -> CSSPixelLength {
        self.rect.width
    }

    pub fn height(&self) -> CSSPixelLength {
        self.rect.height
    }

    pub fn set_width(&mut self, val: CSSPixelLength) {
        self.rect.width = val;
    }

    pub fn set_height(&mut self, val: CSSPixelLength) {
        self.rect.height = val;
    }

    pub fn expanded_by_edges(self, edge: EdgeSizes) -> PositionedRect {
        PositionedRect {
            start_x: (self.start_x - edge.left).px(),
            start_y: (self.start_y - edge.top).px(),
            rect: self.rect.expanded_by_edges(edge),
        }
    }

    /// Expands this rect by another.  The resulting rect has the same `start_x` and `start_y` as
    /// the `self` rect, but with width and height expanded by other rect.
    pub fn expanded_by_rect(self, other: PositionedRect) -> PositionedRect {
        PositionedRect {
            start_x: self.start_x,
            start_y: self.start_y,
            rect: self.rect.expanded_by_rect(other.rect),
        }
    }

    pub fn scaled_by(&self, scale_factor: f32) -> PositionedRect {
        PositionedRect {
            start_x: self.start_x * scale_factor,
            start_y: self.start_y * scale_factor,
            rect: self.rect.scaled_by(scale_factor),
        }
    }
}

/// A collection of edges, e.g. borders, margins, padding.
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

    pub fn expanded_by_edges(self, edges: EdgeSizes) -> EdgeSizes {
        EdgeSizes {
            left: self.left + edges.left,
            right: self.right + edges.right,
            bottom: self.bottom + edges.bottom,
            top: self.top + edges.top,
        }
    }
}
