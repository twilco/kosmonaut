use crate::dom::tree::{NodeData, NodeRef};
use crate::layout::containing_block::ContainingBlock;
use crate::layout::dimensions::Dimensions;
use crate::layout::flow::block::{AnonymousBlockBox, BlockLevelBox};
use crate::layout::flow::inline::{
    AnonymousInlineBox, InlineBox, InlineLevelBox, InlineLevelContent, TextRun,
};
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
use accountable_refcell::Ref;
use image::error::UnsupportedErrorKind::Format;
use std::any::type_name_of_val;
use std::io::Write;
use std::mem::discriminant;
use std::rc::Rc;
use strum_macros::IntoStaticStr;

/// The `LayoutBox` is Kosmonaut's representation of the box tree.  Note that, per-spec, the box
/// tree also contains things that are not strictly boxes, like text runs.
///
/// Loosely maps to the "Generated box" column from the table in this section,
/// https://drafts.csswg.org/css-display/#the-display-properties, with the addition of other
/// scattered box types (like anonymous box types).
#[derive(Clone, Debug, IntoStaticStr)]
pub enum LayoutBox {
    BlockLevel(BlockLevelBox),
    InlineLevel(InlineLevelContent),
}

impl LayoutBox {
    /// Creates a root inline box (which is another name for an anonymous inline box).
    ///
    /// The given node should be that of the element generating root inline box.
    pub fn create_root_inline_box(node: NodeRef, formatting_context: FormattingContextRef) -> Self {
        assert!(formatting_context.is_inline_formatting_context());
        AnonymousInlineBox::new(node, formatting_context).into()
    }

    pub fn add_child(&mut self, child_box: LayoutBox) {
        match self {
            LayoutBox::BlockLevel(blb) => blb.add_child(child_box),
            LayoutBox::InlineLevel(ilc) => ilb.add_child(child_box),
        }
    }

    pub fn computed_values(&self) -> Ref<ComputedValues> {
        match self {
            LayoutBox::BlockLevel(blb) => blb.computed_values(),
            LayoutBox::InlineLevel(ilc) => ilb.computed_values(),
        }
    }

    pub fn dimensions(&self) -> Dimensions {
        match self {
            LayoutBox::BlockLevel(blb) => blb.dimensions(),
            LayoutBox::InlineLevel(ilc) => ilb.dimensions(),
        }
    }

    pub fn dimensions_mut(&mut self) -> &mut Dimensions {
        match self {
            LayoutBox::BlockLevel(blb) => blb.dimensions_mut(),
            LayoutBox::InlineLevel(ilc) => ilb.dimensions_mut(),
        }
    }

    pub fn formatting_context(&self) -> FormattingContextRef {
        match self {
            LayoutBox::BlockLevel(blb) => blb.formatting_context(),
            LayoutBox::InlineLevel(ilc) => ilb.formatting_context(),
        }
    }

    /// Returns a box capable of containing inline children.  If `self` is already an inline-level
    /// box, this will be `self`.  In other cases, we may need to get and or create a child box
    /// capable of containing inline children.
    pub fn get_mut_inline_container(&mut self) -> Option<&mut LayoutBox> {
        match self {
            LayoutBox::BlockLevel(blb) => blb.get_mut_inline_container(),
            LayoutBox::InlineLevel(InlineLevelContent::InlineLevelBox(ilb)) => Some(match ilb {
                InlineLevelBox::AnonymousInline(_) => self,
                InlineLevelBox::InlineBox(_) => self,
            }),
            LayoutBox::InlineLevel(InlineLevelContent::TextRun(_)) => None,
        }
    }

    pub fn inner_box_type_name(&self) -> &'static str {
        match self {
            LayoutBox::BlockLevel(blb) => blb.into(),
            LayoutBox::InlineLevel(ilc) => ilc.into(),
        }
    }

    pub fn is_anonymous_inline(&self) -> bool {
        match self {
            LayoutBox::BlockLevel(_) => false,
            LayoutBox::InlineLevel(ilc) => ilc.is_anonymous_inline(),
        }
    }
}

impl From<AnonymousBlockBox> for LayoutBox {
    fn from(abb: AnonymousBlockBox) -> Self {
        LayoutBox::BlockLevel(BlockLevelBox::AnonymousBlock(abb))
    }
}

impl From<AnonymousInlineBox> for LayoutBox {
    fn from(aib: AnonymousInlineBox) -> Self {
        LayoutBox::InlineLevel(InlineLevelContent::InlineLevelBox(
            InlineLevelBox::AnonymousInline(aib),
        ))
    }
}

impl From<InlineBox> for LayoutBox {
    fn from(ib: InlineBox) -> Self {
        LayoutBox::InlineLevel(InlineLevelContent::InlineLevelBox(
            InlineLevelBox::InlineBox(ib),
        ))
    }
}

impl From<TextRun> for LayoutBox {
    fn from(tr: TextRun) -> Self {
        LayoutBox::InlineLevel(InlineLevelContent::TextRun(tr))
    }
}

impl Layout for LayoutBox {
    fn layout(&mut self, context: LayoutContext) {
        match self {
            LayoutBox::BlockLevel(blb) => blb.layout(context),
            LayoutBox::InlineLevel(ilc) => ilc.layout(context),
        }
    }
}

pub fn get_anonymous_inline_layout_box(boxes: &mut Vec<LayoutBox>) -> Option<&mut LayoutBox> {
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
            LayoutBox::BlockLevel(blb) => blb.dump_layout_format(),
            LayoutBox::InlineLevel(ilc) => ilc.dump_layout_format(),
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
            LayoutBox::BlockLevel(blb) => Some(blb.children()),
            LayoutBox::InlineLevel(InlineLevelContent::InlineLevelBox(ib)) => Some(ib.children()),
            LayoutBox::InlineLevel(InlineLevelContent::TextRun(_)) => None,
        }
        .map(|children| {
            let new_indent = indent_spaces + 2;
            children.iter().for_each(|child| {
                child.dump_layout(write_to, new_indent, verbose);
            })
        });
    }
}
