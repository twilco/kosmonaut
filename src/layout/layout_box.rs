use crate::dom::tree::{NodeData, NodeRef};
use crate::layout::containing_block::ContainingBlock;
use crate::layout::dimensions::Dimensions;
use crate::layout::flow::FlowSide;
use crate::layout::formatting_context::{
    FormattingContext, FormattingContextRef, QualifiedFormattingContext,
};
use crate::layout::rect::Rect;
use crate::layout::{BoxComponent, DumpLayout, DumpLayoutFormat};
use crate::style::values::computed::display::{DisplayBox, OuterDisplay};
use crate::style::values::computed::length::{
    CSSPixelLength, LengthPercentage, LengthPercentageOrAuto,
};
use crate::style::values::computed::{ComputedValues, Direction, Display, WritingMode};
use crate::style::values::used::ToPx;
use crate::Side;
use image::error::UnsupportedErrorKind::Format;
use std::alloc::Layout;
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
    /// A block-level box not associated with any element.
    /// https://drafts.csswg.org/css-display/#anonymous
    AnonymousBlock(AnonymousBlockBox),

    /// An inline-level box not associated with any element.
    ///
    /// An example of an anonymous inline box is the root inline box generated by block containers
    /// who have inline content that needs a place to go.
    ///
    /// For more information about this box type, see: https://drafts.csswg.org/css-inline-3/#model
    ///
    /// An aside, quoting https://drafts.csswg.org/css-display/#block-container:
    ///   > Note, this root inline box concept effectively replaces the "anonymous inline element"
    ///     concept introduced in CSS2§9.2.2.1.
    AnonymousInline(AnonymousInlineBox),

    /// https://drafts.csswg.org/css-display-3/#block-container
    BlockContainer(BlockContainer),

    /// A non-replaced inline-level box whose inner display type is flow. The contents of an inline
    /// box participate in the same inline formatting context as the inline box itself.
    ///
    /// This is also known as an "inline-block".
    ///
    /// https://drafts.csswg.org/css-display/#inline-box
    InlineBox(InlineBox),

    /// A representation of the contents of a text DOM node.
    ///
    /// https://drafts.csswg.org/css-display-3/#text-run
    TextRun(TextRun),
}

impl LayoutBox {
    /// Creates a root inline box (which is another name for an anonymous inline box).
    ///
    /// The given node should be that of the element generating root inline box.
    pub fn create_root_inline_box(node: NodeRef, formatting_context: FormattingContextRef) -> Self {
        assert!(formatting_context.is_inline_formatting_context());
        LayoutBox::AnonymousInline(AnonymousInlineBox::new(node, formatting_context))
    }

    pub fn add_child(&mut self, child_box: LayoutBox) {
        match child_box {
            LayoutBox::AnonymousBlock(ab) => self.add_anonymous_block_child(ab),
            LayoutBox::AnonymousInline(aib) => self.add_anonymous_inline_child(aib),
            LayoutBox::BlockContainer(bc) => self.add_block_container(bc),
            LayoutBox::InlineBox(ib) => self.add_inline_box_child(ib),
            LayoutBox::TextRun(tr) => self.add_text_run_child(tr),
        }
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
            LayoutBox::AnonymousBlock(abb) => abb.computed_values(),
            LayoutBox::AnonymousInline(aib) => aib.computed_values(),
            LayoutBox::BlockContainer(bc) => bc.computed_values(),
            LayoutBox::InlineBox(ib) => ib.computed_values(),
            LayoutBox::TextRun(tr) => tr.computed_values(),
        }
    }

    pub fn formatting_context(&self) -> FormattingContextRef {
        match self {
            LayoutBox::AnonymousInline(aib) => aib.formatting_context(),
            LayoutBox::AnonymousBlock(ab) => ab.formatting_context(),
            LayoutBox::BlockContainer(bc) => bc.formatting_context(),
            LayoutBox::InlineBox(ib) => ib.formatting_context(),
            LayoutBox::TextRun(tr) => tr.formatting_context(),
        }
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
        self.into()
    }

    pub fn is_anonymous_inline(&self) -> bool {
        match self {
            LayoutBox::AnonymousInline(_) => true,
            _ => false,
        }
    }

    pub fn layout(&mut self, containing_block: ContainingBlock, scale_factor: f32) {
        let display = self.computed_values().display;
        match display {
            // The outer display determines how this box participates in layout.
            // https://drafts.csswg.org/css-display/#outer-display-type
            Display::Full(full_display) => match full_display.outer() {
                OuterDisplay::Block => {
                    self.calc_block_level_non_replaced_inlinewise_properties(
                        containing_block,
                        scale_factor,
                    );
                }
                OuterDisplay::Inline => unimplemented!(),
            },
            Display::Box(DisplayBox::None) => unimplemented!(),
        }

        // Layout all children.  This probably belongs somewhere else.
        let direction = self.computed_values().direction;
        let writing_mode = self.computed_values().writing_mode;
        let self_containing_block = ContainingBlock::new(self.dimensions().content, direction, writing_mode);
        match self {
            LayoutBox::AnonymousBlock(abb) => {
                abb.children.iter_mut().for_each(|child| child.layout(self_containing_block, scale_factor))
            }
            LayoutBox::AnonymousInline(aib) => {
                aib.children.iter_mut().for_each(|child| child.layout(self_containing_block, scale_factor))
            }
            LayoutBox::BlockContainer(bc) => {
                bc.children.iter_mut().for_each(|child| child.layout(self_containing_block, scale_factor))
            }
            LayoutBox::InlineBox(ib) => {
                ib.children.iter_mut().for_each(|child| child.layout(self_containing_block, scale_factor))
            }
            LayoutBox::TextRun(_) => {}
        }
    }

    /// Determines used inline-wise direction values.
    ///
    /// Corresponds to CSS 2.1 section 10.3.3.  https://www.w3.org/TR/2011/REC-CSS2-20110607/visudet.html#blockwidth
    pub fn calc_block_level_non_replaced_inlinewise_properties(
        &mut self,
        containing_block: ContainingBlock,
        scale_factor: f32,
    ) {
        let computed_values = self.computed_values();
        let mut margin_inline_start = computed_values.margin_flow_relative(FlowSide::InlineStart);
        let mut margin_inline_end = computed_values.margin_flow_relative(FlowSide::InlineEnd);
        let border_inline_start = computed_values.border_flow_relative(FlowSide::InlineStart);
        let border_inline_end = computed_values.border_flow_relative(FlowSide::InlineEnd);
        let padding_inline_start = computed_values.padding_flow_relative(FlowSide::InlineStart);
        let padding_inline_end = computed_values.padding_flow_relative(FlowSide::InlineEnd);
        let mut inline_size = computed_values.inline_size();
        // Release this &self borrow so we can mutably borrow below.
        drop(computed_values);

        let margin_box_inline_size = margin_inline_start.to_px(containing_block.inline_size())
            + margin_inline_end.to_px(containing_block.inline_size())
            + border_inline_start
            + border_inline_end
            + padding_inline_start.to_px(containing_block.inline_size())
            + padding_inline_end.to_px(containing_block.inline_size())
            + inline_size.to_px(containing_block.inline_size());

        let zero = LengthPercentageOrAuto::new_len(0.);
        let auto = LengthPercentageOrAuto::Auto;
        // If 'width' is not 'auto' and 'border-left-width' + 'padding-left' + 'width' + 'padding-right'
        // + 'border-right-width' (plus any of 'margin-left' or 'margin-right' that are not 'auto')
        // is larger than the width of the containing block, then any 'auto' values for 'margin-left'
        // or 'margin-right' are, for the following rules, treated as zero.
        if inline_size != auto && margin_box_inline_size > containing_block.inline_size() {
            margin_inline_start = zero;
            margin_inline_end = zero;
        }

        // This can be be negative, indicating an overflow (this box has a larger inline-size than
        // the containing block).
        let remaining_inline_size_px = containing_block.inline_size() - margin_box_inline_size;
        let remaining_inline_size = LengthPercentageOrAuto::new_len_px(remaining_inline_size_px);
        match (
            margin_inline_start == auto,
            inline_size == auto,
            margin_inline_end == auto,
        ) {
            (false, false, false) => {
                // If all of the above have a computed value other than 'auto', the values are said to be
                // "over-constrained" and one of the used values will have to be different from its
                // computed value.
                // For an explanation of this rule, see: https://stackoverflow.com/a/34931986/2421349

                // If the 'direction' property of the containing block has the value 'ltr', the
                // specified value of 'margin-right' is ignored and the value is calculated so as
                // to make the equality true. If the value of 'direction' is 'rtl', this happens to
                // 'margin-left' instead.
                match containing_block.direction() {
                    Direction::Ltr => margin_inline_end = remaining_inline_size,
                    Direction::Rtl => margin_inline_start = remaining_inline_size,
                }
            }
            (_, true, _) => {
                // If 'width' is set to 'auto', any other 'auto' values become '0' and 'width'
                // follows from the resulting equality.
                inline_size = remaining_inline_size;
                if margin_inline_start == auto {
                    margin_inline_start = zero;
                }
                if margin_inline_end == auto {
                    margin_inline_end = zero;
                }
            }
            (true, false, true) => {
                // If both inline-start and inline-end margins are 'auto', their used values are equal.
                // This centers the element in the inline-direction.
                let half_remaining_inline_size =
                    LengthPercentageOrAuto::new_len_px(remaining_inline_size_px / 2.0);
                margin_inline_start = half_remaining_inline_size;
                margin_inline_end = half_remaining_inline_size;
            }
            // If there is exactly one value specified as 'auto', its used value follows from the
            // equality.
            (true, false, false) => margin_inline_start = remaining_inline_size,
            (false, false, true) => margin_inline_end = remaining_inline_size,
        }

        let writing_mode = containing_block.writing_mode();
        let direction = containing_block.direction();
        self.dimensions_mut().set_margin(
            FlowSide::InlineStart,
            margin_inline_start.to_px(containing_block.inline_size()),
            writing_mode,
            direction,
        );
        self.dimensions_mut().set_margin(
            FlowSide::InlineEnd,
            margin_inline_end.to_px(containing_block.inline_size()),
            writing_mode,
            direction,
        );
        self.dimensions_mut().set_border(
            FlowSide::InlineStart,
            border_inline_start,
            writing_mode,
            direction,
        );
        self.dimensions_mut().set_border(
            FlowSide::InlineEnd,
            border_inline_end,
            writing_mode,
            direction,
        );
        self.dimensions_mut().set_padding(
            FlowSide::InlineStart,
            padding_inline_start.to_px(containing_block.inline_size()),
            writing_mode,
            direction,
        );
        self.dimensions_mut().set_padding(
            FlowSide::InlineEnd,
            padding_inline_end.to_px(containing_block.inline_size()),
            writing_mode,
            direction,
        );

        self.dimensions_mut().set_inline_size(
            inline_size.to_px(containing_block.inline_size()),
            writing_mode,
        );
    }

    pub fn dimensions(&self) -> Dimensions {
        match self {
            LayoutBox::BlockContainer(bc) => bc.dimensions(),
            LayoutBox::AnonymousBlock(abb) => abb.dimensions(),
            LayoutBox::AnonymousInline(aib) => aib.dimensions(),
            LayoutBox::InlineBox(ib) => ib.dimensions(),
            LayoutBox::TextRun(tr) => tr.dimensions(),
        }
    }

    pub fn dimensions_mut(&mut self) -> &mut Dimensions {
        match self {
            LayoutBox::BlockContainer(bc) => bc.dimensions_mut(),
            LayoutBox::AnonymousBlock(abb) => abb.dimensions_mut(),
            LayoutBox::AnonymousInline(aib) => aib.dimensions_mut(),
            LayoutBox::InlineBox(ib) => ib.dimensions_mut(),
            LayoutBox::TextRun(tr) => tr.dimensions_mut(),
        }
    }
}

fn get_anonymous_inline_layout_box(boxes: &mut Vec<LayoutBox>) -> Option<&mut LayoutBox> {
    boxes.iter_mut().find(|child| child.is_anonymous_inline())
}

impl From<AnonymousBlockBox> for LayoutBox {
    fn from(ab: AnonymousBlockBox) -> Self {
        LayoutBox::AnonymousBlock(ab)
    }
}

impl From<AnonymousInlineBox> for LayoutBox {
    fn from(aib: AnonymousInlineBox) -> Self {
        LayoutBox::AnonymousInline(aib)
    }
}

impl From<BlockContainer> for LayoutBox {
    fn from(block_container: BlockContainer) -> Self {
        LayoutBox::BlockContainer(block_container)
    }
}

impl From<InlineBox> for LayoutBox {
    fn from(inline_box: InlineBox) -> Self {
        LayoutBox::InlineBox(inline_box)
    }
}

impl From<TextRun> for LayoutBox {
    fn from(text_run: TextRun) -> Self {
        LayoutBox::TextRun(text_run)
    }
}

// TODO: Use or remove.
/// Determines what type of layout the box attached to this enum participates in (e.g. block,
/// inline, flex, ...).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LayoutParticipation {
    /// https://drafts.csswg.org/css-display-3/#block-level
    BlockLevel,
    /// https://drafts.csswg.org/css-display-3/#inline-level
    InlineLevel,
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

/// https://drafts.csswg.org/css-display/#anonymous
#[derive(Clone, Debug)]
pub struct AnonymousBlockBox {
    base: BaseBox,
    children: Vec<LayoutBox>,
}

impl AnonymousBlockBox {
    base_box_passthrough_impls!();

    pub fn new(node: NodeRef, formatting_context: FormattingContextRef) -> Self {
        Self {
            base: BaseBox::new(node, formatting_context),
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: LayoutBox) {
        self.children.push(child)
    }
}

#[derive(Clone, Debug)]
pub struct AnonymousInlineBox {
    base: BaseBox,
    children: Vec<LayoutBox>,
}

impl AnonymousInlineBox {
    base_box_passthrough_impls!();

    pub fn new(node: NodeRef, formatting_context: FormattingContextRef) -> Self {
        Self {
            base: BaseBox::new(node, formatting_context),
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: LayoutBox) {
        self.children.push(child)
    }
}

/// A box that contains either contains only inline-level boxes participating in an inline
/// formatting context, or contains only block-level boxes participating in a block formatting
/// context (possibly generating anonymous block boxes to ensure this constraint, as defined in
/// [CSS2§9.2.1.1](https://www.w3.org/TR/CSS2/visuren.html#anonymous-block-level).
///
/// Note that this struct contains no children, and instead turns into one of the above upon adding
/// a child via `add_inline_child` or `add_block_child`.
///
/// https://drafts.csswg.org/css-display-3/#block-container
#[derive(Clone, Debug)]
pub struct BlockContainer {
    base: BaseBox,
    children: Vec<LayoutBox>,
}

impl BlockContainer {
    base_box_passthrough_impls!();

    pub fn new(node: NodeRef, fc: FormattingContextRef) -> Self {
        BlockContainer {
            base: BaseBox::new(node, fc),
            children: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct InlineBox {
    base: BaseBox,
    children: Vec<LayoutBox>,
}

impl InlineBox {
    base_box_passthrough_impls!();

    pub fn new(node: NodeRef, formatting_context: FormattingContextRef) -> Self {
        Self {
            base: BaseBox::new(node, formatting_context),
            children: Vec::new(),
        }
    }
}

/// A representation of the contents of a text DOM node.
///
/// https://drafts.csswg.org/css-display-3/#text-run
#[derive(Clone, Debug)]
pub struct TextRun {
    base: BaseBox,
    /// The text contents of the node.
    ///
    /// TODO: This can be an owned String for now for simplicity's sake, but it would be probably
    /// be more efficient if this were a `&'DOM_LIFETIME str`.
    contents: String,
}

impl TextRun {
    base_box_passthrough_impls!();

    pub fn new(node: NodeRef, formatting_context: FormattingContextRef, contents: String) -> Self {
        Self {
            base: BaseBox::new(node, formatting_context),
            contents,
        }
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
