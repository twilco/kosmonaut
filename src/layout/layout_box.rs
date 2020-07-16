use crate::dom::tree::{NodeData, NodeRef};
use crate::layout::{Dimensions, DumpLayout, DumpLayoutFormat};
use crate::style::values::computed::length::{
    CSSPixelLength, LengthPercentage, LengthPercentageOrAuto,
};
use crate::style::values::computed::ComputedValues;
use crate::style::values::used::ToPx;
use std::cell::Ref;
use std::io::Write;
use std::mem::discriminant;

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
    pub fn new(box_type: BoxType, node: NodeRef) -> LayoutBox {
        LayoutBox {
            box_type,
            dimensions: Default::default(), // initially set all fields to 0.0
            children: Vec::new(),
            node,
        }
    }

    pub fn dimensions(&self) -> Dimensions {
        self.dimensions
    }

    pub fn box_type(&self) -> BoxType {
        self.box_type
    }

    pub fn children(&self) -> &[LayoutBox] {
        &self.children
    }

    /// Retrieve the computed values of the node associated with this layout box.
    pub fn computed_values(&self) -> Ref<ComputedValues> {
        self.node.computed_values()
    }

    /// Determines if this layout box is associated with the root DOM node (<html>).
    pub fn is_root(&self) -> bool {
        match self.node.parent() {
            None => false,
            Some(parent) => matches!(*parent.data(), NodeData::Document(_)),
        }
    }

    /// Directly adds `new_child` to this layout box's children.
    pub fn add_child(&mut self, new_child: LayoutBox) {
        self.children.push(new_child)
    }

    /// Adds the `new_child` to the proper inline-container of `self`.
    pub fn add_child_inline(&mut self, new_child: LayoutBox) {
        self.get_root_inline_box().children.push(new_child)
    }

    /// Gets the proper `LayoutBox` container to insert inline-children in to, which is called
    /// the "root inline box".  The root inline box for a block container holds all of that block
    /// containers inline-level contents.
    ///
    /// If this box is already an inline or anonymous box, we can use ourselves to contain the
    /// inline children.  Otherwise, find or create an anonymous box.
    ///
    /// https://drafts.csswg.org/css-inline-3/#model
    fn get_root_inline_box(&mut self) -> &mut LayoutBox {
        match self.box_type {
            BoxType::Inline | BoxType::Anonymous => self,
            BoxType::Block => {
                let root_inline_box_idx_opt =
                    self.children.iter().enumerate().find_map(|(idx, child)| {
                        if discriminant(&child.box_type) == discriminant(&BoxType::Anonymous) {
                            return Some(idx);
                        }
                        return None;
                    });
                let idx = match root_inline_box_idx_opt {
                    Some(idx) => idx,
                    None => {
                        self.children
                            .push(LayoutBox::new(BoxType::Anonymous, self.node.clone()));
                        self.children.len() - 1
                    }
                };
                self.children.iter_mut().nth(idx).unwrap()
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
    pub fn layout(&mut self, containing_block: Dimensions, scale_factor: f32) {
        match self.box_type {
            BoxType::Block => self.layout_block(containing_block, scale_factor),
            BoxType::Inline => {
                // TODO: The root element is an inline box-type, so when we can actually layout
                // inline boxes, make sure to handle the root element.  This current implementation
                // is a quite a hack.
                if self.is_root() {
                    //                    println!("laying out root element, which is an inline box type");
                    // The root element takes the dimensions of the containing block, which is the viewport.
                    self.dimensions = containing_block;
                    for child in &mut self.children {
                        child.layout(self.dimensions, scale_factor);
                    }
                } else {
                    //                    println!("layout inline box types not implemented");
                    layout_non_block_because_only_block_is_impl(
                        self,
                        containing_block,
                        scale_factor,
                    );
                }
            }
            BoxType::Anonymous => {
                //                println!("layout anonymous box types not implemented");
                layout_non_block_because_only_block_is_impl(self, containing_block, scale_factor);
            }
        }

        // TODO: Implement inline and anonymous layout and then remove this function
        fn layout_non_block_because_only_block_is_impl(
            layout_box: &mut LayoutBox,
            containing_block: Dimensions,
            scale_factor: f32,
        ) {
            layout_box.calculate_block_width(containing_block, scale_factor);
            layout_box.layout_block_children(scale_factor);
        }
    }

    /// Custom debug implementation for when you don't want to print out information about the node,
    /// as that can be quite noisy.
    pub fn nodeless_dbg(&self) {
        //        if self.box_type == BoxType::Block {
        dbg!(&self.dimensions.content);
        dbg!(&self.box_type);
        //            println!("children: [");
        //        }

        self.children.iter().for_each(|child| child.nodeless_dbg());

        //        if self.box_type == BoxType::Block {
        //            println!("]");
        //        }
    }

    /// Assuming `self` is a block-box, calculate the dimensions of this box and any children.
    fn layout_block(&mut self, containing_block: Dimensions, scale_factor: f32) {
        // Child width can depend on parent width, so we need to calculate this box's width before
        // laying out its children.
        self.calculate_block_width(containing_block, scale_factor);

        // Determine where the box is located within its containing block.
        self.calculate_block_position(containing_block, scale_factor);

        // Recursively layout the children of this box.
        self.layout_block_children(scale_factor);

        // Parent height can depend on child height, so let's calculate our height now that we've
        // laid out our children.
        self.calculate_block_height(containing_block);
    }

    /// Calculate the width of a block-level non-replaced element in normal flow.
    ///
    /// https://www.w3.org/TR/CSS2/visudet.html#blockwidth
    ///
    /// Sets the horizontal margin/padding/border dimensions, and the `width`.
    fn calculate_block_width(&mut self, containing_block: Dimensions, scale_factor: f32) {
        let containing_width = containing_block.content.width;
        let cvs = self.node.computed_values();

        let mut width = cvs.width;
        let mut margin_left = cvs.margin_left;
        let mut margin_right = cvs.margin_right;

        let border_left = cvs.border_left_width;
        let border_right = cvs.border_right_width;
        let padding_left = cvs.padding_left;
        let padding_right = cvs.padding_right;

        // Run block layout _with_ the device scale factor applied to ensure the proper values are
        // computed.
        let block_width = (margin_left.size.to_px(containing_width)
            + margin_right.size.to_px(containing_width)
            + border_left.size
            + border_right.size
            + padding_left.size.to_px(containing_width)
            + padding_right.size.to_px(containing_width)
            + width.size.to_px(containing_width))
            * scale_factor;

        let auto = LengthPercentageOrAuto::Auto;
        // If 'width' is not 'auto' and 'border-left-width' + 'padding-left' + 'width' +
        // 'padding-right' + 'border-right-width' (plus any of 'margin-left' or 'margin-right'
        // that are not 'auto') is larger than the width of the containing block, then any 'auto'
        // values for 'margin-left' or 'margin-right' are, for the following rules, treated as zero.
        if width.size != auto && block_width > containing_width {
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
        let underflow = containing_width - block_width;
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
                margin_left.size = LengthPercentageOrAuto::new_len_px(underflow / 2.);
                margin_right.size = LengthPercentageOrAuto::new_len_px(underflow / 2.);
            }
            // If 'width' is set to 'auto', any other 'auto' values become '0' and 'width' follows
            // from the resulting equality.
            (true, _, _) => {
                if margin_left.size == auto {
                    margin_left.size = LengthPercentageOrAuto::new_len(0.)
                };
                if margin_right.size == auto {
                    margin_right.size = LengthPercentageOrAuto::new_len(0.)
                };

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
        // Now that we've calculated the horizontal used values, store them in this box's dimensions.
        let d = &mut self.dimensions;
        d.content.width = width.size.to_px(containing_width);

        d.padding.left = padding_left.size.to_px(containing_width);
        d.padding.right = padding_right.size.to_px(containing_width);

        d.border.left = border_left.size;
        d.border.right = border_right.size;

        d.margin.left = margin_left.size.to_px(containing_width);
        d.margin.right = margin_right.size.to_px(containing_width);
    }

    fn calculate_block_height(&mut self, containing_block: Dimensions) {
        // If the height is set to an explicit length, use that — otherwise keep the value
        // calculated by `layout_block_children`.
        if let LengthPercentageOrAuto::LengthPercentage(lp) =
            self.node.computed_values().height.size
        {
            self.dimensions.content.height = lp.to_px(containing_block.content.height);
        }
    }

    /// Calculates this boxes position (x, y) on the page.
    fn calculate_block_position(&mut self, containing_block: Dimensions, scale_factor: f32) {
        let cvs = self.node.computed_values();
        let containing_width = containing_block.content.width;
        let d = &mut self.dimensions;

        d.padding.bottom = cvs.padding_bottom.size.to_px(containing_width);
        d.padding.top = cvs.padding_top.size.to_px(containing_width);

        d.border.bottom = cvs.border_bottom_width.size;
        d.border.top = cvs.border_top_width.size;

        d.margin.bottom = cvs.margin_bottom.size.to_px(containing_width);
        d.margin.top = cvs.margin_top.size.to_px(containing_width);

        // Ensure window scale factor is applied before computing the start-{x, y} coordinates.
        d.scale_edges_by(scale_factor);

        d.content.start_x =
            (containing_block.content.start_x + d.margin.left + d.border.left + d.padding.left)
                .into();
        // Position the box below all the previous boxes in the container.
        // TODO: I think this is block-layout/context specific behavior.  We'll need to generalize this logic to handle other formatting contexts
        d.content.start_y = (containing_block.content.start_y
            + containing_block.content.height
            + d.margin.top
            + d.border.top
            + d.padding.top)
            .into();
    }

    fn layout_block_children(&mut self, scale_factor: f32) {
        let d = &mut self.dimensions;
        for child in &mut self.children {
            child.layout(*d, scale_factor);
            // Track the height so each child is laid out below the previous content.
            d.content.height += child.dimensions.margin_box().height;
        }
    }
}

/// RenderView at (0,0) size 1166x819 renderer->(0x3055f9250)
/// B-----L- --    HTML RenderBlock at (0,0) size 1166x6248.50 renderer->(0x3055f9700) node->(0x3055f9550)
/// B------- --      BODY RenderBody at (0,0) size 1166x6248.50 renderer->(0x3055f9830) node->(0x3055f9670)
/// B------- --        DIV RenderBlock at (0,0) size 1166x0 renderer->(0x306fcc900) node->(0x305a18480)
/// B------- --        DIV RenderBlock at (0,0) size 1166x0 renderer->(0x306fcca20) node->(0x305a18700)
/// B------- --        DIV RenderBlock at (260,0) size 906x6248.50 renderer->(0x306fccb40) node->(0x305a18800)
/// BX-O--LC --          NAV RenderFlexibleBox at (0,0) size 260x819 renderer->(0x30ddf2e20) node->(0x30dde41c0)
impl DumpLayout for LayoutBox {
    fn dump_layout<W: Write>(&self, write_to: &mut W, indent_spaces: usize) {
        let node_name = match self.box_type {
            BoxType::Anonymous => "".to_owned(),
            BoxType::Block | BoxType::Inline => self.node.data().dump_layout_format(),
        };
        writeln!(
            write_to,
            "{:indent_spaces$}{} {:?} LayoutBox at ({}, {}) size {}x{}",
            "",
            node_name,
            self.box_type,
            self.dimensions.content.start_x.dump_layout_format(),
            self.dimensions.content.start_y.dump_layout_format(),
            self.dimensions.content.width.dump_layout_format(),
            self.dimensions.content.height.dump_layout_format(),
            indent_spaces = indent_spaces,
        )
        .expect("error writing layout dump");

        self.children.iter().for_each(|child| {
            child.dump_layout(write_to, indent_spaces + 2);
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BoxType {
    Block,
    Inline,
    Anonymous,
}
