use crate::dom::tree::{NodeData, NodeRef};
use crate::layout::dimensions::{LogicalDimensions, PhysicalDimensions};
use crate::layout::{BoxComponent, DumpLayout, DumpLayoutFormat, LogicalDirection};
use crate::style::values::computed::length::{
    CSSPixelLength, LengthPercentage, LengthPercentageOrAuto,
};
use crate::style::values::computed::{ComputedValues, Direction, WritingMode};
use crate::style::values::used::ToPx;
use crate::Side;
use std::cell::Ref;
use std::io::Write;
use std::mem::discriminant;

#[derive(Clone, Debug)]
pub struct LayoutBox {
    box_type: BoxType,
    children: Vec<LayoutBox>,
    dimensions: LogicalDimensions,
    direction: Direction,
    /// Reference to the closest non-anonymous node.  This distinction only matters for anonymous
    /// boxes, since anonymous boxes are by definition not associated with a node, but need access
    /// to a node to get computed values during layout.  If the box is a block, inline, or any other
    /// non-anonymous box, this field is simply the actual DOM node associated with this box.
    node: NodeRef,
    writing_mode: WritingMode,
}

impl LayoutBox {
    /// Creates a new layout box.  The passed in node should be the DOM node associated with
    /// the box, assuming it is a non-anonymous box.  If creating an anonymous box, `node`
    /// should be the DOM node associated with the closest non-anonymous box.
    pub fn new(
        box_type: BoxType,
        node: NodeRef,
        direction: Direction,
        writing_mode: WritingMode,
    ) -> LayoutBox {
        LayoutBox {
            box_type,
            dimensions: LogicalDimensions::new(writing_mode, direction),
            direction,
            children: Vec::new(),
            node,
            writing_mode,
        }
    }

    pub fn physical_dimensions(&self) -> PhysicalDimensions {
        self.dimensions.physical()
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
            BoxType::Anonymous | BoxType::AnonymousInline | BoxType::Inline => self,
            BoxType::Block => {
                let root_inline_box_idx_opt =
                    self.children.iter().enumerate().find_map(|(idx, child)| {
                        if discriminant(&child.box_type) == discriminant(&BoxType::AnonymousInline)
                        {
                            return Some(idx);
                        }
                        None
                    });
                let idx = match root_inline_box_idx_opt {
                    Some(idx) => idx,
                    None => {
                        self.children.push(LayoutBox::new(
                            BoxType::AnonymousInline,
                            self.node.clone(),
                            self.direction,
                            self.writing_mode,
                        ));
                        self.children.len() - 1
                    }
                };
                self.children.get_mut(idx).unwrap()
            }
        }
    }

    /// Calculates the dimensions of this box, and any child boxes.
    ///
    /// A block's logical width depends on that of its parent (called "containing block" in the spec),
    /// while a block's logical height depends on that of its children.  This is important to know
    /// in layout.
    ///
    /// In this step, we will be taking computed values and calculating actual, used values
    /// based on the constraint of our environment.
    pub fn layout(&mut self, containing_block: PhysicalDimensions, scale_factor: f32) {
        match self.box_type {
            BoxType::Anonymous | BoxType::AnonymousInline => {
                //                println!("layout anonymous box types not implemented");
                layout_non_block_because_only_block_is_impl(self, containing_block, scale_factor);
            }
            BoxType::Block => self.layout_block(containing_block, scale_factor),
            BoxType::Inline => {
                // TODO: The root element is an inline box-type, so when we can actually layout
                // inline boxes, make sure to handle the root element.  This current implementation
                // is a quite a hack.
                if self.is_root() {
                    //                    println!("laying out root element, which is an inline box type");
                    // The root element takes the dimensions of the containing block, which is the viewport.
                    self.dimensions.replace_inner_physical(containing_block);
                    for child in &mut self.children {
                        child.layout(self.dimensions.physical(), scale_factor);
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
        }

        // TODO: Implement inline and anonymous layout and then remove this function
        fn layout_non_block_because_only_block_is_impl(
            layout_box: &mut LayoutBox,
            containing_block: PhysicalDimensions,
            scale_factor: f32,
        ) {
            layout_box.calculate_block_logical_width(containing_block, scale_factor);
            layout_box.layout_block_children(scale_factor);
        }
    }

    /// Assuming `self` is a block-box, calculate the dimensions of this box and any children.
    fn layout_block(&mut self, containing_block: PhysicalDimensions, scale_factor: f32) {
        // Child logical width (inline size) can depend on parent logical width, so we need to
        // calculate it for this box before laying out its children.
        self.calculate_block_logical_width(containing_block, scale_factor);

        // Determine where the box is located within its containing block.
        self.calculate_block_position(containing_block, scale_factor);

        // Recursively layout the children of this box, which also determines this block's logical
        // height (block size).
        self.layout_block_children(scale_factor);

        // Now that we've performed a layout with logical properties, let's apply any physical
        // properties explicitly given for this block (e.g. `width`, `height`, bottom/left/right/top
        // properties).
        self.apply_physical_properties(containing_block, scale_factor);
    }

    /// Calculate the logical width (inline size) of a block-level non-replaced element in normal
    /// flow.
    ///
    /// https://www.w3.org/TR/CSS2/visudet.html#blockwidth
    /// https://drafts.csswg.org/css-writing-modes-4/#vertical-layout
    ///
    /// Sets the inline margin/padding/border dimensions, and the inline size.
    fn calculate_block_logical_width(
        &mut self,
        containing_block: PhysicalDimensions,
        scale_factor: f32,
    ) {
        // FIXME: In all of our abstract layout code, we use self.writing_mode to determine logical
        // dimensions, but I believe we should instead be looking at the writing mode of the
        // containing block.  https://drafts.csswg.org/css-writing-modes-4/#logical-direction-layout

        // FIXME: Always using the physical containing block width here is almost certainly wrong, but
        // works...sometimes.  This should probably be the containing block `width` or `height`
        // depending on the self.writing mode (or containing block writing mode?)...but this didn't
        // work when I tried it, likely due to other bugs, such as the FIXME directly above.
        // https://drafts.csswg.org/css-writing-modes-4/#logical-direction-layout
        let containing_width = containing_block.content.width;
        let cvs = self.node.computed_values();

        let mut logical_width = cvs.logical_width();
        let mut logical_margin_left = cvs.logical_margin(Side::Left);
        let mut logical_margin_right = cvs.logical_margin(Side::Right);

        let logical_border_left = cvs.logical_border_width(Side::Left);
        let logical_border_right = cvs.logical_border_width(Side::Right);

        let logical_padding_left = cvs.logical_padding(Side::Left);
        let logical_padding_right = cvs.logical_padding(Side::Right);

        // Run block layout _with_ the device scale factor applied to ensure the proper values are
        // computed.
        let block_width = (logical_margin_left.to_px(containing_width)
            + logical_margin_right.to_px(containing_width)
            + logical_border_left
            + logical_border_right
            + logical_padding_left.to_px(containing_width)
            + logical_padding_right.to_px(containing_width)
            + logical_width.to_px(containing_width))
            * scale_factor;

        let auto = LengthPercentageOrAuto::Auto;
        // If 'width' is not 'auto' and 'border-left-width' + 'padding-left' + 'width' +
        // 'padding-right' + 'border-right-width' (plus any of 'margin-left' or 'margin-right'
        // that are not 'auto') is larger than the width of the containing block, then any 'auto'
        // values for 'margin-left' or 'margin-right' are, for the following rules, treated as zero.
        if logical_width != auto && block_width > containing_width {
            if logical_margin_left == auto {
                logical_margin_left =
                    LengthPercentageOrAuto::LengthPercentage(LengthPercentage::new_len(0.));
            }
            if logical_margin_right == auto {
                logical_margin_right =
                    LengthPercentageOrAuto::LengthPercentage(LengthPercentage::new_len(0.));
            }
        }
        // This value can be negative, indicating an overflow or "overconstraint", if the width of
        // this box is greater than that of the containing one.
        let underflow = containing_width - block_width;
        match (
            logical_width == auto,
            logical_margin_left == auto,
            logical_margin_right == auto,
        ) {
            // If all of the above have a computed value other than 'auto', the values are said to be
            // "over-constrained" and one of the used values will have to be different from its computed
            // value. If the 'direction' property of the containing block has the value 'ltr', the
            // specified value of 'margin-right' is ignored and the value is calculated so as to make
            // the equality true. If the value of 'direction' is 'rtl', this happens to 'margin-left' instead.
            (false, false, false) => {
                // TODO: Support `direction: rtl` property/value
                logical_margin_right = LengthPercentageOrAuto::new_len_px(
                    logical_margin_right.to_px(containing_width) + underflow,
                )
            }
            // If there is exactly one margin value specified as 'auto', its used value follows
            // from the equality.
            (false, true, false) => {
                logical_margin_left = LengthPercentageOrAuto::new_len_px(underflow)
            }
            (false, false, true) => {
                logical_margin_right = LengthPercentageOrAuto::new_len_px(underflow)
            }
            // If both 'margin-left' and 'margin-right' are 'auto', their used values are equal.
            // This centers the element with respect to the edges of the containing block.
            (false, true, true) => {
                logical_margin_left = LengthPercentageOrAuto::new_len_px(underflow / 2.);
                logical_margin_right = LengthPercentageOrAuto::new_len_px(underflow / 2.);
            }
            // If 'width' is set to 'auto', any other 'auto' values become '0' and 'width' follows
            // from the resulting equality.
            (true, _, _) => {
                if logical_margin_left == auto {
                    logical_margin_left = LengthPercentageOrAuto::new_len(0.)
                };
                if logical_margin_right == auto {
                    logical_margin_right = LengthPercentageOrAuto::new_len(0.)
                };

                if underflow >= CSSPixelLength::new(0.) {
                    logical_width = LengthPercentageOrAuto::new_len_px(underflow)
                } else {
                    // Width cannot be negative, adjust `margin-right` instead
                    // TODO: Support `direction: rtl` property/value
                    logical_width = LengthPercentageOrAuto::new_len(0.);
                    logical_margin_right = LengthPercentageOrAuto::new_len_px(
                        logical_margin_right.to_px(containing_width) + underflow,
                    );
                }
            }
        }
        // Now that we've calculated the inline used values, store them in this box's dimensions.
        let d = &mut self.dimensions;
        d.set_inline_size(logical_width.to_px(containing_width));

        d.set(
            LogicalDirection::InlineStart,
            BoxComponent::Padding,
            logical_padding_left.to_px(containing_width),
        );
        d.set(
            LogicalDirection::InlineEnd,
            BoxComponent::Padding,
            logical_padding_right.to_px(containing_width),
        );

        d.set(
            LogicalDirection::InlineStart,
            BoxComponent::Border,
            logical_border_left,
        );
        d.set(
            LogicalDirection::InlineEnd,
            BoxComponent::Border,
            logical_border_right,
        );

        d.set(
            LogicalDirection::InlineStart,
            BoxComponent::Margin,
            logical_margin_left.to_px(containing_width),
        );
        d.set(
            LogicalDirection::InlineEnd,
            BoxComponent::Margin,
            logical_margin_right.to_px(containing_width),
        );
    }

    /// Calculates this box's (x, y) position on the page.
    fn calculate_block_position(
        &mut self,
        containing_block: PhysicalDimensions,
        scale_factor: f32,
    ) {
        let cvs = self.node.computed_values();
        let containing_width = if self.writing_mode.is_horizontal() {
            containing_block.content.width
        } else {
            containing_block.content.height
        };
        let d = &mut self.dimensions;

        d.set(
            LogicalDirection::BlockEnd,
            BoxComponent::Padding,
            cvs.padding_bottom.size.to_px(containing_width),
        );
        d.set(
            LogicalDirection::BlockStart,
            BoxComponent::Padding,
            cvs.padding_top.size.to_px(containing_width),
        );

        d.set(
            LogicalDirection::BlockEnd,
            BoxComponent::Border,
            cvs.border_bottom_width.size,
        );
        d.set(
            LogicalDirection::BlockStart,
            BoxComponent::Border,
            cvs.border_top_width.size,
        );

        d.set(
            LogicalDirection::BlockEnd,
            BoxComponent::Margin,
            cvs.margin_bottom.size.to_px(containing_width),
        );
        d.set(
            LogicalDirection::BlockStart,
            BoxComponent::Margin,
            cvs.margin_top.size.to_px(containing_width),
        );

        // Ensure window scale factor is applied before computing the start-{x, y} coordinates.
        d.scale_edges_by(scale_factor);

        let container_inline_start_coord = if self.writing_mode.is_horizontal() {
            containing_block.content.start_x
        } else {
            containing_block.content.start_y
        };

        let container_block_size = if self.writing_mode.is_horizontal() {
            containing_block.content.height
        } else {
            containing_block.content.width
        };
        d.set_inline_start_coord(
            (container_inline_start_coord
                + d.get(LogicalDirection::InlineStart, BoxComponent::Margin)
                + d.get(LogicalDirection::InlineStart, BoxComponent::Border)
                + d.get(LogicalDirection::InlineStart, BoxComponent::Padding))
            .into(),
        );
        // TODO: Always adding containing_block.content.start_y is almost certainly wrong, but
        // works...sometimes..  This should probably be `container_block_start_coord`...but this
        // didn't work when I tried it, likely due to other bugs, such as the fact that we may not
        // be mapping flow-relative directions correctly (we should sometimes be using the
        // containing block writing-mode, but we always use the writing-mode of the `self` block.)
        // https://drafts.csswg.org/css-writing-modes-4/#logical-direction-layout
        d.set_block_start_coord(
            (container_block_size
                + containing_block.content.start_y
                + d.get(LogicalDirection::BlockStart, BoxComponent::Margin)
                + d.get(LogicalDirection::BlockStart, BoxComponent::Border)
                + d.get(LogicalDirection::BlockStart, BoxComponent::Padding))
            .into(),
        );
    }

    fn layout_block_children(&mut self, scale_factor: f32) {
        let mut physical_dimensions = self.dimensions.physical();
        for child in &mut self.children {
            child.layout(physical_dimensions, scale_factor);
            // Track the block size so each child is laid out after the previous one.
            self.dimensions.set_block_size(
                self.dimensions.get_content_block_size() + child.dimensions.margin_box_block_size(),
            );
            physical_dimensions = self.dimensions.physical();
        }
    }

    /// If this block has any explicitly set values (e.g. lenght or percentage values, NOT auto) for
    /// physical properties (e.g. `width`, `height`, left/bottom/right/top properties), this
    /// function will set them.  Otherwise, the used values will be those given by other layout
    /// equations.
    fn apply_physical_properties(
        &mut self,
        containing_block: PhysicalDimensions,
        scale_factor: f32,
    ) {
        let width = self.node.computed_values().width.size;
        if let LengthPercentageOrAuto::LengthPercentage(lp) = width {
            self.dimensions
                .set_phys_width(lp.to_px(containing_block.content.width) * scale_factor);
        }

        let height = self.node.computed_values().height.size;
        if let LengthPercentageOrAuto::LengthPercentage(lp) = height {
            self.dimensions
                .set_phys_height(lp.to_px(containing_block.content.height) * scale_factor);
        }

        // FIXME: The physical bottom/left/right/top properties for margin, border, and padding
        // are broken in non-horizontal writing modes because they are applied logically, when
        // these properties should instead be applied physically.  E.g., margin-left should always affect
        // the page-relative left margin of the box, but instead reflects the flow relative margin
        // left, which physically ends up being the top margin.
    }
}

/// Writes a textual representation of the layout tree starting with the `self` LayoutBox.  Built
/// to somewhat match WebKit's version of layout dumps, which look like:
///
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
            BoxType::Anonymous | BoxType::AnonymousInline => "".to_owned(),
            BoxType::Block | BoxType::Inline => self.node.data().dump_layout_format(),
        };
        let physical_dimensions = self.dimensions.physical();
        writeln!(
            write_to,
            "{:indent_spaces$}{} {:?} LayoutBox at ({}, {}) size {}x{}",
            "",
            node_name,
            self.box_type,
            physical_dimensions.content.start_x.dump_layout_format(),
            physical_dimensions.content.start_y.dump_layout_format(),
            physical_dimensions.content.width.dump_layout_format(),
            physical_dimensions.content.height.dump_layout_format(),
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
    Anonymous,
    /// An example of an anonymous inline box is the root inline box generated by block containers
    /// who have inline content that needs a place to go.
    ///
    /// For more information about this box type, see: https://drafts.csswg.org/css-inline-3/#model
    AnonymousInline,
    Block,
    Inline,
}
