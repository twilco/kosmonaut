use crate::dom::tree::{NodeData, NodeRef};
use crate::layout::containing_block::ContainingBlock;
use crate::layout::dimensions::Dimensions;
use crate::layout::flow::block::{AnonymousBlockBox, BlockLevelBox};
use crate::layout::flow::inline::{AnonymousInlineBox, InlineBox, InlineLevelBox, TextRun};
use crate::layout::flow::{
    solve_block_level_inline_size, BlockContainer, BlockLevelBox, FlowSide, InlineLevelBox,
    SolveInlineSizeInput,
};
use crate::layout::formatting_context::{
    FormattingContext, FormattingContextRef, QualifiedFormattingContext,
};
use crate::layout::rect::Rect;
use crate::layout::{BoxComponent, DumpLayout, DumpLayoutFormat, Layout, LayoutContext};
use crate::style::values::computed::display::{DisplayBox, OuterDisplay};
use crate::style::values::computed::length::{
    CSSPixelLength, LengthPercentage, LengthPercentageOrAuto,
};
use crate::style::values::computed::{ComputedValues, Direction, Display, WritingMode};
use crate::style::values::used::ToPx;
use crate::Side;
use image::error::UnsupportedErrorKind::Format;
use std::any::type_name_of_val;
use std::cell::Ref;
use std::io::Write;
use std::mem::discriminant;
use std::rc::Rc;
use strum_macros::IntoStaticStr;

/// The `LayoutBox` is Kosmonaut's representation of the box tree.
///
/// Loosely maps to the "Generated box" column from the table in this section,
/// https://drafts.csswg.org/css-display/#the-display-properties, with the addition of other
/// scattered box types (like anonymous box types).
#[derive(Clone, Debug, IntoStaticStr)]
pub enum LayoutBox {
    BlockLevel(BlockLevelBox),
    InlineLevel(InlineLevelBox),
}

impl LayoutBox {
    /// Creates a root inline box (which is another name for an anonymous inline box).
    ///
    /// The given node should be that of the element generating root inline box.
    pub fn create_root_inline_box(node: NodeRef, formatting_context: FormattingContextRef) -> Self {
        assert!(formatting_context.is_inline_formatting_context());
        LayoutBox::InlineLevel(InlineLevelBox::AnonymousInline(AnonymousInlineBox::new(
            node,
            formatting_context,
        )))
    }

    pub fn add_child(&mut self, child_box: LayoutBox) {
        match self {
            LayoutBox::BlockLevel(blb) => blb.add_child(child_box),
            LayoutBox::InlineLevel(ilb) => ilb.add_child(child_box),
        }
        // match child_box {
        //     LayoutBox::AnonymousBlock(ab) => self.add_anonymous_block_child(ab),
        //     LayoutBox::AnonymousInline(aib) => self.add_anonymous_inline_child(aib),
        //     LayoutBox::BlockContainer(bc) => self.add_block_container(bc),
        //     LayoutBox::InlineBox(ib) => self.add_inline_box_child(ib),
        //     LayoutBox::TextRun(tr) => self.add_text_run_child(tr),
        // }
    }

    pub fn add_anonymous_block_child(&mut self, abb_child: AnonymousBlockBox) {
        match self {
            LayoutBox::AnonymousBlock(abb) => panic(type_name_of_val(abb)),
            LayoutBox::AnonymousInline(aib) => panic(type_name_of_val(aib)),
            LayoutBox::BlockContainer(bc) => bc.children.push(abb_child.into()),
            LayoutBox::InlineBox(ib) => panic(type_name_of_val(ib)),
            LayoutBox::TextRun(tr) => panic(type_name_of_val(tr)),
        }

        fn panic(type_name: &'static str) {
            panic!(
                "tried to add anonymous bloc box as a child of a(n) {} box.  is this correct?",
                type_name
            )
        }
    }

    pub fn add_anonymous_inline_child(&mut self, aib_child: AnonymousInlineBox) {
        match self {
            LayoutBox::AnonymousBlock(ab) => panic(type_name_of_val(ab)),
            LayoutBox::AnonymousInline(aib) => panic(type_name_of_val(aib)),
            LayoutBox::BlockContainer(bc) => panic(type_name_of_val(bc)),
            LayoutBox::InlineBox(ib) => panic(type_name_of_val(ib)),
            LayoutBox::TextRun(tr) => panic(type_name_of_val(tr)),
        }

        fn panic(type_name: &'static str) {
            panic!(
                "tried to add anonymous inline box as a child of a(n) {} box.  is this correct?",
                type_name
            )
        }
    }

    pub fn add_block_container(&mut self, bc_child: BlockContainer) {
        match self {
            LayoutBox::AnonymousBlock(abb) => abb.children.push(bc_child.into()),
            LayoutBox::AnonymousInline(aib) => panic(type_name_of_val(aib)),
            LayoutBox::BlockContainer(bc) => bc.children.push(bc_child.into()),
            LayoutBox::InlineBox(ib) => panic(type_name_of_val(ib)),
            LayoutBox::TextRun(tr) => panic(type_name_of_val(tr)),
        }

        fn panic(type_name: &'static str) {
            panic!(
                "Tried to add block container as a child of a(n) {} box.  Is this correct?",
                type_name
            )
        }
    }

    pub fn add_inline_box_child(&mut self, ib_child: InlineBox) {
        match self {
            LayoutBox::AnonymousBlock(ab) => panic(type_name_of_val(ab)),
            LayoutBox::AnonymousInline(aib) => aib.children.push(ib_child.into()),
            LayoutBox::BlockContainer(bc) => panic(type_name_of_val(bc)),
            LayoutBox::InlineBox(ib) => ib.children.push(ib_child.into()),
            LayoutBox::TextRun(tr) => panic(type_name_of_val(tr)),
        }

        fn panic(type_name: &'static str) {
            panic!(
                "Tried to add inline box as a child of a(n) {} box.  Is this correct?",
                type_name
            )
        }
    }

    pub fn add_text_run_child(&mut self, text_run_child: TextRun) {
        match self {
            LayoutBox::AnonymousBlock(ab) => panic(type_name_of_val(ab)),
            LayoutBox::AnonymousInline(aib) => aib.children.push(text_run_child.into()),
            LayoutBox::BlockContainer(bc) => panic(type_name_of_val(bc)),
            LayoutBox::InlineBox(ib) => ib.children.push(text_run_child.into()),
            LayoutBox::TextRun(tr) => panic(type_name_of_val(tr)),
        }

        fn panic(type_name: &'static str) {
            panic!(
                "Tried to add text run as a child of a(n) {} box.  Is this correct?",
                type_name
            )
        }
    }

    pub fn computed_values(&self) -> Ref<ComputedValues> {
        match self {
            LayoutBox::BlockLevel(blb) => blb.computed_values(),
            LayoutBox::InlineLevel(ilb) => ilb.computed_values(),
        }
        // match self {
        //     LayoutBox::AnonymousBlock(abb) => abb.computed_values(),
        //     LayoutBox::AnonymousInline(aib) => aib.computed_values(),
        //     LayoutBox::BlockContainer(bc) => bc.computed_values(),
        //     LayoutBox::InlineBox(ib) => ib.computed_values(),
        //     LayoutBox::TextRun(tr) => tr.computed_values(),
        // }
    }

    pub fn dimensions(&self) -> Dimensions {
        match self {
            LayoutBox::BlockLevel(blb) => blb.dimensions(),
            LayoutBox::InlineLevel(ilb) => ilb.dimensions(),
        }
        // match self {
        //     LayoutBox::BlockContainer(bc) => bc.dimensions(),
        //     LayoutBox::AnonymousBlock(abb) => abb.dimensions(),
        //     LayoutBox::AnonymousInline(aib) => aib.dimensions(),
        //     LayoutBox::InlineBox(ib) => ib.dimensions(),
        //     LayoutBox::TextRun(tr) => tr.dimensions(),
        // }
    }

    pub fn dimensions_mut(&mut self) -> &mut Dimensions {
        match self {
            LayoutBox::BlockLevel(blb) => blb.dimensions_mut(),
            LayoutBox::InlineLevel(ilb) => ilb.dimensions_mut(),
        }
        // match self {
        //     LayoutBox::BlockContainer(bc) => bc.dimensions_mut(),
        //     LayoutBox::AnonymousBlock(abb) => abb.dimensions_mut(),
        //     LayoutBox::AnonymousInline(aib) => aib.dimensions_mut(),
        //     LayoutBox::InlineBox(ib) => ib.dimensions_mut(),
        //     LayoutBox::TextRun(tr) => tr.dimensions_mut(),
        // }
    }

    pub fn formatting_context(&self) -> FormattingContextRef {
        match self {
            LayoutBox::BlockLevel(blb) => blb.formatting_context(),
            LayoutBox::InlineLevel(ilb) => ilb.formatting_context(),
        }
        // match self {
        //     LayoutBox::AnonymousInline(aib) => aib.formatting_context(),
        //     LayoutBox::AnonymousBlock(ab) => ab.formatting_context(),
        //     LayoutBox::BlockContainer(bc) => bc.formatting_context(),
        //     LayoutBox::InlineBox(ib) => ib.formatting_context(),
        //     LayoutBox::TextRun(tr) => tr.formatting_context(),
        // }
    }

    /// Returns a box capable of containing inline children.  If `self` is already an inline-box,
    /// this will be `self`.  In other cases, we may need to get and or create a child box capable
    /// of containing inline children.
    pub fn get_mut_inline_container(&mut self) -> Option<&mut LayoutBox> {
        match self {
            LayoutBox::InlineBox(_) | LayoutBox::AnonymousInline(_) => Some(self),
            LayoutBox::BlockContainer(bc) => bc
                .children
                .iter_mut()
                .last()
                .map(|last_child| match last_child {
                    LayoutBox::AnonymousBlock(abb) => {
                        get_anonymous_inline_layout_box(&mut abb.children)
                    }
                    _ => None,
                })
                .flatten(),
            LayoutBox::AnonymousBlock(abb) => get_anonymous_inline_layout_box(&mut abb.children),
            LayoutBox::TextRun(_) => panic!("get_inline_container_box called on text run"),
        }
    }

    pub fn inner_box_type_name(&self) -> &'static str {
        match self {
            LayoutBox::BlockLevel(blb) => blb.into(),
            LayoutBox::InlineLevel(ilb) => ilb.into(),
        }
    }

    pub fn is_anonymous_inline(&self) -> bool {
        match self {
            LayoutBox::BlockLevel(_) => false,
            LayoutBox::InlineLevel(ilb) => ilb.is_anonymous_inline(),
        }
    }
}

impl Layout for LayoutBox {
    fn layout(&mut self, context: LayoutContext) {
        match self {
            LayoutBox::BlockLevel(blb) => blb.layout(context),
            LayoutBox::InlineLevel(ilb) => ilb.layout(context),
        }
    }
}

fn get_anonymous_inline_layout_box(boxes: &mut Vec<LayoutBox>) -> Option<&mut LayoutBox> {
    boxes.iter_mut().find(|child| child.is_anonymous_inline())
}

/// Base box containing state and behavior common to all boxes.  To be clear, this is an ease-of-use
/// construct, not something that maps to spec-language.
#[derive(Clone, Debug)]
pub struct BaseBox {
    dimensions: Dimensions,
    /// The formatting context this box participates in.
    formatting_context: FormattingContextRef,
    /// Reference to the closest non-anonymous node.  This distinction only matters for anonymous
    /// boxes, since anonymous boxes are by definition not associated with a node, but need access
    /// to a node to get computed values during layout.  If the box is a block, inline, or any other
    /// non-anonymous box, this field is simply the actual DOM node associated with this box.
    node: NodeRef,
}

impl BaseBox {
    pub fn new(node: NodeRef, formatting_context: FormattingContextRef) -> BaseBox {
        BaseBox {
            dimensions: Dimensions::default(),
            formatting_context: formatting_context.clone(),
            node,
        }
    }

    /// Retrieve the computed values of the node associated with this layout box.
    pub fn computed_values(&self) -> Ref<ComputedValues> {
        self.node.computed_values()
    }

    pub fn dimensions(&self) -> Dimensions {
        self.dimensions
    }

    pub fn dimensions_mut(&mut self) -> &mut Dimensions {
        &mut self.dimensions
    }

    pub fn formatting_context(&self) -> FormattingContextRef {
        self.formatting_context.clone()
    }

    pub fn has_inline_formatting_context(&self) -> bool {
        self.formatting_context.is_inline_formatting_context()
    }

    /// Determines if this layout box is associated with the root DOM node (<html>).
    pub fn is_root(&self) -> bool {
        match self.node.parent() {
            None => false,
            Some(parent) => matches!(*parent.data(), NodeData::Document(_)),
        }
    }

    pub fn node(&self) -> NodeRef {
        self.node.clone()
    }
}

#[macro_export]
macro_rules! base_box_passthrough_impls {
    () => {
        #[inline(always)]
        pub fn computed_values(&self) -> Ref<ComputedValues> {
            self.base.computed_values()
        }

        #[inline(always)]
        pub fn dimensions(&self) -> Dimensions {
            self.base.dimensions()
        }

        #[inline(always)]
        pub fn dimensions_mut(&mut self) -> &mut Dimensions {
            self.base.dimensions_mut()
        }

        #[inline(always)]
        pub fn formatting_context(&self) -> FormattingContextRef {
            self.base.formatting_context()
        }

        #[inline(always)]
        pub fn has_inline_formatting_context(&self) -> bool {
            self.base.has_inline_formatting_context()
        }

        #[inline(always)]
        pub fn is_root(&self) -> bool {
            self.base.is_root()
        }

        #[inline(always)]
        pub fn node(&self) -> NodeRef {
            self.base.node()
        }
    };
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
///
/// With the `verbose` flag, much more information is printed (such as all of the margin, border,
/// and padding values).
impl DumpLayout for LayoutBox {
    fn dump_layout<W: Write>(&self, write_to: &mut W, indent_spaces: usize, verbose: bool) {
        let node_name_and_data = match self {
            LayoutBox::AnonymousBlock(_) | LayoutBox::AnonymousInline(_) => "".to_owned(),
            LayoutBox::BlockContainer(bc) => bc.node().data().dump_layout_format(),
            LayoutBox::InlineBox(ib) => ib.node().data().dump_layout_format(),
            LayoutBox::TextRun(tr) => tr.node().data().dump_layout_format(),
        };
        let dimensions = self.dimensions();
        let verbose_str = if verbose {
            format!(
                " (ml{} mr{} mb{} mt{} bl{} br{} bb{} bt{} pl{} pr{} pb{} pt{})",
                dimensions.margin.left.dump_layout_format(),
                dimensions.margin.right.dump_layout_format(),
                dimensions.margin.bottom.dump_layout_format(),
                dimensions.margin.top.dump_layout_format(),
                dimensions.border.left.dump_layout_format(),
                dimensions.border.right.dump_layout_format(),
                dimensions.border.bottom.dump_layout_format(),
                dimensions.border.top.dump_layout_format(),
                dimensions.padding.left.dump_layout_format(),
                dimensions.padding.right.dump_layout_format(),
                dimensions.padding.bottom.dump_layout_format(),
                dimensions.padding.top.dump_layout_format(),
            )
        } else {
            "".to_owned()
        };
        let node_dump = if node_name_and_data.len() > 0 {
            format!("{} ", node_name_and_data)
        } else {
            "".to_owned()
        };
        writeln!(
            write_to,
            "{:indent_spaces$}{}{} at ({}, {}) size {}x{}{}",
            "",
            node_dump,
            self.inner_box_type_name(),
            dimensions.content.start_x.dump_layout_format(),
            dimensions.content.start_y.dump_layout_format(),
            dimensions.content.width.dump_layout_format(),
            dimensions.content.height.dump_layout_format(),
            verbose_str,
            indent_spaces = indent_spaces,
        )
        .expect("error writing layout dump");

        match self {
            LayoutBox::AnonymousBlock(abb) => Some(&abb.children),
            LayoutBox::AnonymousInline(aib) => Some(&aib.children),
            LayoutBox::BlockContainer(bc) => Some(&bc.children),
            LayoutBox::InlineBox(ib) => Some(&ib.children),
            LayoutBox::TextRun(_) => None,
        }
        .map(|children| {
            let new_indent = indent_spaces + 2;
            children.iter().for_each(|child| {
                child.dump_layout(write_to, new_indent, verbose);
            })
        });
    }
}
