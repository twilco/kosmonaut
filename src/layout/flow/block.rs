use crate::base_box_passthrough_impls;
use crate::dom::tree::NodeRef;
use crate::layout::containing_block::ContainingBlock;
use crate::layout::dimensions::Dimensions;
use crate::layout::flow::{BlockContainer, FlowSide, OriginRelativeProgression};
use crate::layout::formatting_context::FormattingContextRef;
use crate::layout::layout_box::{get_anonymous_inline_layout_box, BaseBox, LayoutBox};
use crate::layout::{BoxComponent, DumpLayoutFormat, Layout, LayoutContext};
use crate::style::values::computed::display::{DisplayBox, OuterDisplay};
use crate::style::values::computed::length::{
    CSSPixelLength, LengthPercentage, LengthPercentageOrAuto,
};
use crate::style::values::computed::{ComputedValues, Direction, Display, WritingMode};
use crate::style::values::used::ToPx;
use crate::style::values::CSSFloat;
use accountable_refcell::Ref;

#[derive(Clone, Debug, IntoStaticStr)]
pub enum BlockLevelBox {
    /// A block-level box not associated with any element.
    /// https://drafts.csswg.org/css-display/#anonymous
    AnonymousBlock(AnonymousBlockBox),
    /// A block-level block container (note block containers can also be inline-level).
    /// https://drafts.csswg.org/css-display-3/#block-container
    BlockContainer(BlockContainer),
}

impl BlockLevelBox {
    pub fn add_child(&mut self, new_child: LayoutBox) {
        match self {
            BlockLevelBox::AnonymousBlock(ab) => ab.add_child(new_child),
            BlockLevelBox::BlockContainer(bc) => bc.add_child(new_child),
        }
    }

    /// Creates a new block-level block container.
    pub fn new_block_container(node: NodeRef, formatting_context: FormattingContextRef) -> Self {
        BlockLevelBox::BlockContainer(BlockContainer::new(node, formatting_context))
    }

    pub fn children(&self) -> &Vec<LayoutBox> {
        match self {
            BlockLevelBox::AnonymousBlock(ab) => ab.children(),
            BlockLevelBox::BlockContainer(bc) => bc.children(),
        }
    }

    pub fn computed_values(&self) -> Ref<ComputedValues> {
        match self {
            BlockLevelBox::AnonymousBlock(ab) => ab.computed_values(),
            BlockLevelBox::BlockContainer(bc) => bc.computed_values(),
        }
    }

    pub fn dimensions(&self) -> Dimensions {
        match self {
            BlockLevelBox::AnonymousBlock(ab) => ab.dimensions(),
            BlockLevelBox::BlockContainer(bc) => bc.dimensions(),
        }
    }

    pub fn dimensions_mut(&mut self) -> &mut Dimensions {
        match self {
            BlockLevelBox::AnonymousBlock(ab) => ab.dimensions_mut(),
            BlockLevelBox::BlockContainer(bc) => bc.dimensions_mut(),
        }
    }

    pub fn formatting_context(&self) -> FormattingContextRef {
        match self {
            BlockLevelBox::AnonymousBlock(ab) => ab.formatting_context(),
            BlockLevelBox::BlockContainer(bc) => bc.formatting_context(),
        }
    }

    pub fn compute_block_start_coord(
        &self,
        containing_block: ContainingBlock,
        scale_factor: f32,
    ) -> CSSFloat {
        0.
    }

    pub fn compute_inline_start_coord(
        &self,
        containing_block: ContainingBlock,
        scale_factor: f32,
    ) -> CSSFloat {
        match OriginRelativeProgression::inline_start_origin_relative_direction(
            containing_block.writing_mode(),
            containing_block.direction(),
        ) {
            OriginRelativeProgression::AwayFromOrigin => {
                let margin_inline_start = self.dimensions().get(
                    FlowSide::InlineStart,
                    BoxComponent::Margin,
                    containing_block.writing_mode(),
                    containing_block.direction(),
                );
                (containing_block.self_relative_inline_start_coord() + margin_inline_start).px()
            }
            OriginRelativeProgression::TowardsOrigin => {
                let containing_block_inline_end_coord = containing_block
                    .self_relative_inline_start_coord()
                    + containing_block.self_relative_inline_size();
                let inline_margin_box_size = self
                    .dimensions()
                    .margin_box_inline_size(containing_block.writing_mode());
                (containing_block_inline_end_coord
                    - inline_margin_box_size
                    - self.dimensions().get(
                        FlowSide::InlineEnd,
                        BoxComponent::Margin,
                        containing_block.writing_mode(),
                        containing_block.direction(),
                    ))
                .px()
            }
        }
    }

    pub fn get_mut_inline_container(&mut self) -> Option<&mut LayoutBox> {
        match self {
            BlockLevelBox::AnonymousBlock(abb) => {
                get_anonymous_inline_layout_box(&mut abb.children)
            }
            BlockLevelBox::BlockContainer(bc) => bc
                .children_mut()
                .iter_mut()
                .last()
                .map(|last_child| match last_child {
                    LayoutBox::BlockLevel(BlockLevelBox::AnonymousBlock(ref mut abb)) => {
                        get_anonymous_inline_layout_box(abb.children_mut())
                    }
                    _ => None,
                })
                .flatten(),
        }
    }

    fn layout_children(&mut self, scale_factor: f32, containing_writing_mode: WritingMode) {
        fn layout_and_get_block_size(
            layout_box: &mut LayoutBox,
            layout_context: LayoutContext,
        ) -> CSSPixelLength {
            layout_box.layout(layout_context);
            layout_box
                .dimensions()
                .get_content_block_size(layout_context.containing_block.writing_mode())
        }

        let direction = self.computed_values().direction;
        let writing_mode = self.computed_values().writing_mode;
        let self_containing_block =
            ContainingBlock::new(self.dimensions().content, direction, writing_mode);
        let layout_context = LayoutContext::new(self_containing_block, scale_factor);
        let children_total_block_size = match self {
            BlockLevelBox::AnonymousBlock(abb) => abb
                .children_mut()
                .iter_mut()
                .fold(CSSPixelLength::new(0.), |accumulator, child| {
                    accumulator + layout_and_get_block_size(child, layout_context)
                }),
            BlockLevelBox::BlockContainer(bc) => bc
                .children_mut()
                .iter_mut()
                .fold(CSSPixelLength::new(0.), |accumulator, child| {
                    accumulator + layout_and_get_block_size(child, layout_context)
                }),
        };
        let current_block_size = self
            .dimensions()
            .get_content_block_size(containing_writing_mode);
        self.dimensions_mut().set_block_size(
            current_block_size + children_total_block_size,
            containing_writing_mode,
        );
    }

    pub fn solve_and_set_inline_level_properties(
        &mut self,
        containing_block: ContainingBlock,
        scale_factor: f32,
    ) {
        // Use the containing block's writing mode for resolving flow-relative directions.
        // https://drafts.csswg.org/css-writing-modes-4/#logical-direction-layout
        let writing_mode = containing_block.writing_mode();

        let computed_values = self.computed_values();
        let border_inline_start =
            computed_values.border_flow_relative(FlowSide::InlineStart, writing_mode);
        let border_inline_end =
            computed_values.border_flow_relative(FlowSide::InlineEnd, writing_mode);
        let padding_inline_start =
            computed_values.padding_flow_relative(FlowSide::InlineStart, writing_mode);
        let padding_inline_end =
            computed_values.padding_flow_relative(FlowSide::InlineEnd, writing_mode);

        let solved_inline_sizes = solve_block_level_inline_size(SolveInlineSizeInput {
            containing_block,
            margin_inline_start: computed_values
                .margin_flow_relative(FlowSide::InlineStart, writing_mode),
            margin_inline_end: computed_values
                .margin_flow_relative(FlowSide::InlineEnd, writing_mode),
            border_inline_start,
            border_inline_end,
            padding_inline_start,
            padding_inline_end,
            inline_size: computed_values.inline_size(writing_mode),
        });
        // Release this &self borrow so we can mutably borrow below.
        drop(computed_values);

        let direction = containing_block.direction();
        self.dimensions_mut().set_margin(
            FlowSide::InlineStart,
            solved_inline_sizes.margin_inline_start,
            writing_mode,
            direction,
        );
        self.dimensions_mut().set_margin(
            FlowSide::InlineEnd,
            solved_inline_sizes.margin_inline_end,
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
            padding_inline_start.to_px(containing_block.self_relative_inline_size()),
            writing_mode,
            direction,
        );
        self.dimensions_mut().set_padding(
            FlowSide::InlineEnd,
            padding_inline_end.to_px(containing_block.self_relative_inline_size()),
            writing_mode,
            direction,
        );
        self.dimensions_mut()
            .set_inline_size(solved_inline_sizes.inline_size, writing_mode);
        // Before computing the `inline_start_coord` of this box, we need to apply values the author
        // has specified in the inline-direction (e.g. `width` in `writing:mode: horizontal-tb`, or
        // `height` in the other `writing-modes`.
        // self.apply_inline_physical_properties(containing_block, scale_factor);
        // TODO: Need to apply block-direction physical property
        let inline_start_coord = self.compute_inline_start_coord(containing_block, scale_factor);
        self.dimensions_mut()
            .set_inline_start_coord(inline_start_coord, containing_block.writing_mode());
    }

    /// Corresponds to CSS 2.1 section 10.6.3.  Currently no other sections are implemented.
    /// https://www.w3.org/TR/2011/REC-CSS2-20110607/visudet.html#normal-block
    pub fn solve_and_set_block_level_properties(
        &mut self,
        containing_block: ContainingBlock,
        scale_factor: f32,
    ) {
        // Use the containing block's writing mode for resolving flow-relative directions.
        // https://drafts.csswg.org/css-writing-modes-4/#logical-direction-layout
        let writing_mode = containing_block.writing_mode();

        let computed_values = self.computed_values();
        let mut margin_block_start =
            computed_values.margin_flow_relative(FlowSide::BlockStart, writing_mode);
        let mut margin_block_end =
            computed_values.margin_flow_relative(FlowSide::BlockEnd, writing_mode);
        let border_block_start =
            computed_values.border_flow_relative(FlowSide::BlockStart, writing_mode);
        let border_block_end =
            computed_values.border_flow_relative(FlowSide::BlockEnd, writing_mode);
        let padding_block_start =
            computed_values.padding_flow_relative(FlowSide::BlockStart, writing_mode);
        let padding_block_end =
            computed_values.padding_flow_relative(FlowSide::BlockEnd, writing_mode);
        let block_size = computed_values.block_size(writing_mode);
        // Release immutable borrow of &self so we can mutably borrow below.
        drop(computed_values);

        let zero = LengthPercentageOrAuto::new_len(0.);
        let auto = LengthPercentageOrAuto::Auto;
        // If the block-start or blond-end margins are auto, their used value is 0.
        if margin_block_start == auto {
            margin_block_start = zero
        }
        if margin_block_end == auto {
            margin_block_end = zero
        }

        let direction = containing_block.direction();
        self.dimensions_mut().set_margin(
            FlowSide::BlockStart,
            margin_block_start.to_px(containing_block.self_relative_block_size()),
            writing_mode,
            direction,
        );
        self.dimensions_mut().set_margin(
            FlowSide::BlockEnd,
            margin_block_end.to_px(containing_block.self_relative_block_size()),
            writing_mode,
            direction,
        );
        self.dimensions_mut().set_border(
            FlowSide::BlockStart,
            border_block_start,
            writing_mode,
            direction,
        );
        self.dimensions_mut().set_border(
            FlowSide::BlockEnd,
            border_block_end,
            writing_mode,
            direction,
        );
        self.dimensions_mut().set_padding(
            FlowSide::BlockStart,
            padding_block_start.to_px(containing_block.self_relative_block_size()),
            writing_mode,
            direction,
        );
        self.dimensions_mut().set_padding(
            FlowSide::BlockEnd,
            padding_block_end.to_px(containing_block.self_relative_block_size()),
            writing_mode,
            direction,
        );

        self.dimensions_mut().set_block_size(
            block_size.to_px(containing_block.self_relative_block_size()),
            writing_mode,
        );

        // TODO: This block-start-coord calculation needs to go somewhere else.  Refactor with `layout_children`
        // block-size refactor
        let block_start_coord = self.compute_block_start_coord(containing_block, scale_factor);
        self.dimensions_mut()
            .set_block_start_coord(block_start_coord, containing_block.writing_mode());
        // TODO: It is not intuitive that this method sets the block_size of this box.  Rename or
        // refactor
        self.layout_children(scale_factor, containing_block.writing_mode());
    }

    fn apply_inline_physical_properties(
        &mut self,
        containing_block: ContainingBlock,
        scale_factor: f32,
    ) {
        match containing_block.writing_mode() {
            WritingMode::HorizontalTb => {
                let width = self.computed_values().width.size;
                if let LengthPercentageOrAuto::LengthPercentage(lp) = width {
                    self.dimensions_mut()
                        .set_width(lp.to_px(containing_block.rect().width) * scale_factor);
                }
            }
            WritingMode::VerticalRl
            | WritingMode::SidewaysRl
            | WritingMode::VerticalLr
            | WritingMode::SidewaysLr => {
                let height = self.computed_values().height.size;
                if let LengthPercentageOrAuto::LengthPercentage(lp) = height {
                    self.dimensions_mut()
                        .set_height(lp.to_px(containing_block.rect().height) * scale_factor);
                }
            }
        }
    }
    /// If this block has any explicitly set values (e.g. length or percentage values, NOT auto) for
    /// physical properties (e.g. `width`, `height`, left/bottom/right/top properties), this
    /// function will set them.  Otherwise, the used values will be those given by other layout
    /// equations.
    fn apply_physical_properties(&mut self, containing_block: ContainingBlock, scale_factor: f32) {
        let width = self.computed_values().width.size;
        if let LengthPercentageOrAuto::LengthPercentage(lp) = width {
            self.dimensions_mut()
                .set_width(lp.to_px(containing_block.rect().width) * scale_factor);
        }

        let height = self.computed_values().height.size;
        if let LengthPercentageOrAuto::LengthPercentage(lp) = height {
            self.dimensions_mut()
                .set_height(lp.to_px(containing_block.rect().height) * scale_factor);
        }
        // FIXME: The physical bottom/left/right/top properties for margin, border, and padding
        // are broken in non-horizontal writing modes because they are applied logically, when
        // these properties should instead be applied physically.  E.g., margin-left should always affect
        // the page-relative left margin of the box, but instead reflects the flow relative margin
        // left, which physically ends up being the top margin.
    }
}

impl DumpLayoutFormat for BlockLevelBox {
    fn dump_layout_format(&self) -> String {
        match self {
            BlockLevelBox::AnonymousBlock(ab) => ab.dump_layout_format(),
            BlockLevelBox::BlockContainer(bc) => bc.dump_layout_format(),
        }
    }
}

impl Layout for BlockLevelBox {
    // TODO: Throughout block-level layout, we use the self_relative_* [1] methods of containing blocks
    // when making calculations.  We need to confirm that this is correct.
    //
    // [1] "self-relative" means the containing block evaluates abstract flow directions against its
    // own writing-mode, rather than that of it's own containing block.
    fn layout(&mut self, context: LayoutContext) {
        let LayoutContext {
            containing_block,
            scale_factor,
        } = context;
        let display = self.computed_values().display;
        match display {
            // The outer display determines how this box participates in layout.
            // https://drafts.csswg.org/css-display/#outer-display-type
            Display::Full(full_display) => match full_display.outer() {
                OuterDisplay::Block => {
                    self.solve_and_set_inline_level_properties(containing_block, scale_factor);
                    self.solve_and_set_block_level_properties(containing_block, scale_factor);
                    self.apply_physical_properties(containing_block, scale_factor);
                }
                // OuterDisplay::Inline => unimplemented!("OuterDisplay::Inline in BlockLevelBox Layout impl"),
                OuterDisplay::Inline => {}
            },
            Display::Box(DisplayBox::None) => {
                unimplemented!("display none in BlockLevelBox Layout impl")
            }
        }
    }
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

    pub fn children(&self) -> &Vec<LayoutBox> {
        &self.children
    }

    pub fn children_mut(&mut self) -> &mut Vec<LayoutBox> {
        &mut self.children
    }
}

impl DumpLayoutFormat for AnonymousBlockBox {
    fn dump_layout_format(&self) -> String {
        // Anonymous boxes are not generated by an element of the DOM, so simply print the name
        // of this struct for the dump-layout display.
        "AnonymousBlockBox".to_string()
    }
}

pub struct SolveInlineSizeInput {
    pub containing_block: ContainingBlock,
    pub margin_inline_start: LengthPercentageOrAuto,
    pub margin_inline_end: LengthPercentageOrAuto,
    pub border_inline_start: CSSPixelLength,
    pub border_inline_end: CSSPixelLength,
    pub padding_inline_start: LengthPercentage,
    pub padding_inline_end: LengthPercentage,
    pub inline_size: LengthPercentageOrAuto,
}

pub struct SolveInlineSizeOutput {
    pub margin_inline_start: CSSPixelLength,
    pub margin_inline_end: CSSPixelLength,
    pub inline_size: CSSPixelLength,
}

/// Pure function to determine used inline-wise direction sizes for a block-level box.
///
/// Corresponds to CSS 2.1 section 10.3.3.  https://www.w3.org/TR/2011/REC-CSS2-20110607/visudet.html#blockwidth
/// No other sections besides 10.3.3 are currently handled.
pub fn solve_block_level_inline_size(input: SolveInlineSizeInput) -> SolveInlineSizeOutput {
    let SolveInlineSizeInput {
        containing_block,
        mut margin_inline_start,
        mut margin_inline_end,
        border_inline_start,
        border_inline_end,
        padding_inline_start,
        padding_inline_end,
        mut inline_size,
    } = input;

    let margin_box_inline_size = margin_inline_start
        .to_px(containing_block.self_relative_inline_size())
        + margin_inline_end.to_px(containing_block.self_relative_inline_size())
        + border_inline_start
        + border_inline_end
        + padding_inline_start.to_px(containing_block.self_relative_inline_size())
        + padding_inline_end.to_px(containing_block.self_relative_inline_size())
        + inline_size.to_px(containing_block.self_relative_inline_size());

    let zero = LengthPercentageOrAuto::new_len(0.);
    let auto = LengthPercentageOrAuto::Auto;
    // If 'width' is not 'auto' and 'border-left-width' + 'padding-left' + 'width' + 'padding-right'
    // + 'border-right-width' (plus any of 'margin-left' or 'margin-right' that are not 'auto')
    // is larger than the width of the containing block, then any 'auto' values for 'margin-left'
    // or 'margin-right' are, for the following rules, treated as zero.
    if inline_size != auto && margin_box_inline_size > containing_block.self_relative_inline_size()
    {
        margin_inline_start = zero;
        margin_inline_end = zero;
    }

    // This can be be negative, indicating an overflow (this box has a larger inline-size than
    // the containing block).
    let available_inline_space_px =
        containing_block.self_relative_inline_size() - margin_box_inline_size;
    let available_inline_space = LengthPercentageOrAuto::new_len_px(available_inline_space_px);
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
                Direction::Ltr => margin_inline_end = available_inline_space,
                Direction::Rtl => margin_inline_start = available_inline_space,
            }
        }
        (_, true, _) => {
            // If 'width' is set to 'auto', any other 'auto' values become '0' and 'width'
            // follows from the resulting equality.
            inline_size = available_inline_space;
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
                LengthPercentageOrAuto::new_len_px(available_inline_space_px / 2.0);
            margin_inline_start = half_remaining_inline_size;
            margin_inline_end = half_remaining_inline_size;
        }
        // If there is exactly one value specified as 'auto', its used value follows from the
        // equality.
        (true, false, false) => margin_inline_start = available_inline_space,
        (false, false, true) => margin_inline_end = available_inline_space,
    }

    SolveInlineSizeOutput {
        margin_inline_start: margin_inline_start
            .to_px(containing_block.self_relative_inline_size()),
        margin_inline_end: margin_inline_end.to_px(containing_block.self_relative_inline_size()),
        inline_size: inline_size.to_px(containing_block.self_relative_inline_size()),
    }
}
