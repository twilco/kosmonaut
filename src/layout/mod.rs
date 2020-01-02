// Useful links:
//  * https://www.w3.org/TR/css-display-3/#css-box
//  * https://www.w3.org/TR/2018/WD-css-box-3-20181218/#intro
pub mod layout_box;

use crate::dom::tree::NodeRef;
use crate::layout::layout_box::{BoxType, LayoutBox};
use crate::layout::BoxType::Anonymous;
use crate::style::values::computed::length::CSSPixelLength;
use crate::style::values::computed::length::{LengthPercentage, LengthPercentageOrAuto};
use crate::style::values::computed::width::Width;
use crate::style::values::computed::Display;
use crate::style::values::used::ToPx;
use crate::style::values::CSSFloat;
use crate::Side;
use std::mem::discriminant;

/// Takes a DOM node and builds the corresponding layout tree of it and its children.  Returns
/// `None` if `node` is a `Display::None`.
pub fn build_layout_tree(node: NodeRef) -> Option<LayoutBox> {
    let computed_values = &*node.computed_values();
    // TODO: We need to think about the validity of making strong-ref clones to nodes here (and elsewhere).
    // Will things get properly dropped?  Maybe LayoutBox should store a `Weak` ref?
    let mut layout_box = match computed_values.display {
        Display::Block => LayoutBox::new(BoxType::Block, node.clone()),
        Display::Inline => LayoutBox::new(BoxType::Inline, node.clone()),
        Display::None => {
            return None;
        }
    };

    for child in node.children() {
        let child_computed_values = &*child.computed_values();
        match child_computed_values.display {
            Display::Block => {
                if let Some(child_box) = build_layout_tree(child.clone()) {
                    // TODO: We don't handle the case where a block-flow child box is added to an inline box.
                    // This current behavior is wrong — we should be checking if `node` is an `Display::Inline` and
                    // doing something different here.  To fix, see: https://www.w3.org/TR/CSS2/visuren.html#box-gen
                    // Namely, the paragraph that begins with "When an inline box contains an in-flow block-level box"
                    // This concept _might_ be called "fragmenting".
                    layout_box.children.push(child_box)
                }
            }
            Display::Inline => {
                if let Some(child_box) = build_layout_tree(child.clone()) {
                    layout_box.get_inline_container().children.push(child_box)
                }
            }
            Display::None => {}
        }
    }
    Some(layout_box)
}

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
}

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
    fn expanded_by(self, edge: EdgeSizes) -> Rect {
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
