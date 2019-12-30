// Useful links:
//  * https://www.w3.org/TR/css-display-3/#css-box
//  * https://www.w3.org/TR/2018/WD-css-box-3-20181218/#intro

use crate::dom::tree::NodeRef;
use crate::layout::BoxType::Anonymous;
use crate::style::values::computed::length::CSSPixelLength;
use crate::style::values::computed::length::{LengthPercentage, LengthPercentageOrAuto};
use crate::style::values::computed::width::Width;
use crate::style::values::computed::Display;
use crate::style::values::used::ToPx;
use crate::style::values::CSSFloat;
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

#[derive(Clone, Copy, Debug, Default)]
pub struct Rect {
    pub x: CSSFloat,
    pub y: CSSFloat,
    pub width: CSSPixelLength,
    pub height: CSSPixelLength,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct EdgeSizes {
    pub left: CSSPixelLength,
    pub right: CSSPixelLength,
    pub top: CSSPixelLength,
    pub bottom: CSSPixelLength,
}

#[derive(Clone, Debug)]
pub struct LayoutBox {
    dimensions: Dimensions,
    box_type: BoxType,
    children: Vec<LayoutBox>,
    /// Reference to the closest non-anonymous node.  This distinction only matters for anonymous
    /// boxes, since anonymous boxes are by definition not associated with a node, but need access
    /// to a node to get computed values during layout.  If the box is a block, inline, or any other
    /// non-anonymous box, this field is simply the actual DOM node associated with this box.
    node: NodeRef,
}

impl LayoutBox {
    /// Creates a new layout box.  The passed in node should be the DOM node associated with
    /// the box, assuming it is a non-anonymous box.  If creating an anonymous box, `node`
    /// should be the DOM node associated with the closest non-anonymous box.
    fn new(box_type: BoxType, node: NodeRef) -> LayoutBox {
        LayoutBox {
            box_type,
            dimensions: Default::default(), // initially set all fields to 0.0
            children: Vec::new(),
            node,
        }
    }

    /// Gets the proper `LayoutBox` container to insert inline-children in to.
    ///
    /// If a block box contains inline-children, an anonymous box must be used to contain them.
    ///
    /// If this box is already an inline or anonymous box, we can use ourself to contain the inline
    /// children.  Otherwise, find or create an anonymous box.
    fn get_inline_container(&mut self) -> &mut LayoutBox {
        match self.box_type {
            BoxType::Inline | BoxType::Anonymous => self,
            BoxType::Block => {
                match self.children.last() {
                    Some(last_child)
                        if discriminant(&last_child.box_type)
                            == discriminant(&BoxType::Anonymous) => {}
                    _ => self
                        .children
                        .push(LayoutBox::new(BoxType::Anonymous, self.node.clone())),
                }
                self.children
                    .last_mut()
                    .expect("there should've been at least one child")
            }
        }
    }

    /// Calculates the dimensions of this box, and any child boxes.
    ///
    /// A block's width depends on that of its parent (called "containing block" in the spec), while
    /// a block's height depends on that of its children.  This is important to know in layout.
    ///
    /// In this step, we will be taking computed values and calculating actual, used values
    /// based on the constraint of our environment.
    fn layout(&mut self, containing_block: Dimensions) {
        match self.box_type {
            BoxType::Block => self.layout_block(containing_block),
            BoxType::Inline => unimplemented!("layout inline box types"),
            BoxType::Anonymous => unimplemented!("layout anonymous box types"),
        }
    }

    /// Assuming `self` is a block-box, calculate the dimensions of this box and any children.
    fn layout_block(&mut self, containing_block: Dimensions) {
        // Child width can depend on parent width, so we need to calculate this box's width before
        // laying out its children.
        self.calculate_block_width(containing_block);

        // Determine where the box is located within its containing block.
        self.calculate_block_position(containing_block);

        // Recursively layout the children of this box.
        self.layout_block_children();

        // Parent height can depend on child height, so let's `calculate_height` now that we've
        // laid out our children.
        self.calculate_block_height();
    }

    /// https://www.w3.org/TR/CSS2/visudet.html#blockwidth
    fn calculate_block_width(&mut self, containing_block: Dimensions) {
        let containing_width = containing_block.content.width;
        let cvs = self.node.computed_values();

        let mut width = cvs.width;
        let mut margin_left = cvs.margin_left;
        let mut margin_right = cvs.margin_right;

        let border_left = cvs.border_left_width;
        let border_right = cvs.border_right_width;
        let padding_left = cvs.padding_left;
        let padding_right = cvs.padding_right;

        let total_size = margin_left.size.to_px(containing_width)
            + margin_right.size.to_px(containing_width)
            + border_left.size
            + border_right.size
            + padding_left.size.to_px(containing_width)
            + padding_right.size.to_px(containing_width);

        let auto = LengthPercentageOrAuto::Auto;
        // If 'width' is not 'auto' and 'border-left-width' + 'padding-left' + 'width' +
        // 'padding-right' + 'border-right-width' (plus any of 'margin-left' or 'margin-right'
        // that are not 'auto') is larger than the width of the containing block, then any 'auto'
        // values for 'margin-left' or 'margin-right' are, for the following rules, treated as zero.
        if width.size != auto && total_size > containing_width {
            if margin_left.size == auto {
                margin_left.size =
                    LengthPercentageOrAuto::LengthPercentage(LengthPercentage::new_len(0.));
            }
            if margin_right.size == auto {
                margin_right.size =
                    LengthPercentageOrAuto::LengthPercentage(LengthPercentage::new_len(0.));
            }
        }
        // This value can be negative, indicating an overflow or "overconstraint", if the width of
        // this box is greater than that of the containing one.
        let underflow = containing_width - total_size;

        match (
            width.size == auto,
            margin_left.size == auto,
            margin_right.size == auto,
        ) {
            // If all of the above have a computed value other than 'auto', the values are said to be
            // "over-constrained" and one of the used values will have to be different from its computed
            // value. If the 'direction' property of the containing block has the value 'ltr', the
            // specified value of 'margin-right' is ignored and the value is calculated so as to make
            // the equality true. If the value of 'direction' is 'rtl', this happens to 'margin-left' instead.
            (false, false, false) => {
                // TODO: Support `direction: rtl` property/value
                margin_right.size = LengthPercentageOrAuto::new_len_px(
                    margin_right.size.to_px(containing_width) + underflow,
                )
            }
            // If there is exactly one margin value specified as 'auto', its used value follows
            // from the equality.
            (false, true, false) => {
                margin_left.size = LengthPercentageOrAuto::new_len_px(underflow)
            }
            (false, false, true) => {
                margin_right.size = LengthPercentageOrAuto::new_len_px(underflow)
            }
            // If both 'margin-left' and 'margin-right' are 'auto', their used values are equal.
            // This horizontally centers the element with respect to the edges of the containing block.
            (false, true, true) => {
                let new_size = underflow / 2.;
                margin_left.size = LengthPercentageOrAuto::new_len_px(new_size);
                margin_right.size = LengthPercentageOrAuto::new_len_px(new_size);
            }
            // If 'width' is set to 'auto', any other 'auto' values become '0' and 'width' follows
            // from the resulting equality.
            (true, _, _) => {
                margin_left.size = auto;
                margin_right.size = auto;
                if underflow >= CSSPixelLength::new(0.) {
                    width.size = LengthPercentageOrAuto::new_len_px(underflow)
                } else {
                    // Width cannot be negative, adjust `margin-right` instead
                    // TODO: Support `direction: rtl` property/value
                    width.size = LengthPercentageOrAuto::new_len(0.);
                    margin_right.size = LengthPercentageOrAuto::new_len_px(
                        margin_right.size.to_px(containing_width) + underflow,
                    );
                }
            }
        }
    }

    fn calculate_block_height(&mut self) {}

    fn calculate_block_position(&mut self, _containing_block: Dimensions) {}

    fn layout_block_children(&mut self) {}
}

#[derive(Clone, Debug)]
pub enum BoxType {
    Block,
    Inline,
    Anonymous,
}
