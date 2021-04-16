use crate::apply_box_sizing_properties_base_box_passthrough_impls;
use crate::behavior::{ApplyBoxSizingProperties, BaseLayoutBoxBehavior};
use crate::containing_block::ContainingBlock;
use crate::dimensions::Dimensions;
use crate::flow::{BlockContainer, OriginRelativeProgression};
use crate::formatting_context::FormattingContextRef;
use crate::layout_box::{get_anonymous_inline_layout_box, BaseBox, LayoutBox};
use crate::layout_box_behavior_base_box_passthrough_impls;
use crate::{BoxComponent, DumpLayoutFormat, Layout, LayoutContext};
use accountable_refcell::Ref;
use dom::tree::NodeRef;
use enum_dispatch::enum_dispatch;
use primitives::sides::{FlowSide, PhysicalSide};
use primitives::units::{CSSFloat, CSSPixelLength};
use style::values::computed::length::{LengthPercentage, LengthPercentageOrAuto};
use style::values::computed::ComputedValues;
use style::values::used::ToPx;

#[enum_dispatch]
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
    /// Creates a new block-level block container.
    pub(crate) fn new_block_container(
        node: NodeRef,
        formatting_context: FormattingContextRef,
    ) -> Self {
        BlockLevelBox::BlockContainer(BlockContainer::new(node, formatting_context))
    }

    pub(crate) fn add_child(&mut self, new_child: LayoutBox) {
        match self {
            BlockLevelBox::AnonymousBlock(ab) => ab.add_child(new_child),
            BlockLevelBox::BlockContainer(bc) => bc.add_child(new_child),
        }
    }

    pub(crate) fn children(&self) -> &Vec<LayoutBox> {
        match self {
            BlockLevelBox::AnonymousBlock(ab) => ab.children(),
            BlockLevelBox::BlockContainer(bc) => bc.children(),
        }
    }

    pub(crate) fn get_mut_inline_container(&mut self) -> Option<&mut LayoutBox> {
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

    fn flip_block_start_coord(
        &mut self,
        layout_viewport_block_size: CSSPixelLength,
        containing_block: ContainingBlock,
    ) {
        let containing_writing_mode = containing_block.writing_mode();
        // We probably shouldn't be flipping the block start coordinate if block progression isn't
        // reversed (towards the page origin).
        assert_eq!(
            OriginRelativeProgression::block_start_origin_relative_direction(
                containing_writing_mode
            ),
            OriginRelativeProgression::TowardsOrigin
        );

        let flipped_block_start_coord =
            compute_flipped_block_start_coord(ComputeFlippedBlockStartCoordInput {
                layout_viewport_block_size,
                containing_block_block_size: containing_block.self_relative_block_size(),
                content_box_block_size: self
                    .dimensions()
                    .content_box_block_size(containing_writing_mode),
                preferred_block_size: self.computed_values().block_size(containing_writing_mode),
                current_block_start_coord: self
                    .dimensions()
                    .get_block_start_coord(containing_writing_mode),
            });
        self.dimensions_mut()
            .set_block_start_coord(flipped_block_start_coord, containing_writing_mode);
    }

    fn layout_children(&mut self, context: &LayoutContext) {
        let direction = self.computed_values().direction;
        let writing_mode = self.computed_values().writing_mode;

        let (children, self_dimensions) = match self {
            BlockLevelBox::AnonymousBlock(abb) => (&mut abb.children, abb.base.dimensions_mut()),
            BlockLevelBox::BlockContainer(bc) => (&mut bc.children, bc.base.dimensions_mut()),
        };
        for child in children {
            // The rectangle selected as the containing block will need to change when we support other
            // `position` property types (e.g. some may want the content-box, others the margin-box, etc).
            // For now, the behavior of the default `position` value, "static" is hardcoded here.
            // https://www.w3.org/TR/CSS2/visudet.html#containing-block-details
            // 10.1.2: For other [not-root] elements, if the element's position is 'relative' or
            // 'static', the containing block is formed by the content edge of the nearest block
            // container ancestor box.
            child.layout(LayoutContext::new(
                ContainingBlock::new(self_dimensions.content, direction, writing_mode),
                context.layout_viewport,
            ));

            // Add this child's margin-box to our content box so the next child is laid out after
            // this one.
            self_dimensions.add_to_block_size(
                child.dimensions().margin_box_block_size(writing_mode),
                context.containing_block.writing_mode(),
            );
        }
    }

    fn solve_and_set_inline_level_properties(&mut self, context: &LayoutContext) {
        let containing_block = context.containing_block;
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

        let cvs = self.computed_values();
        let (ml, mr, mb, mt) = (
            cvs.margin_left,
            cvs.margin_right,
            cvs.margin_bottom,
            cvs.margin_top,
        );
        drop(cvs);
        if let LengthPercentageOrAuto::LengthPercentage(lp) = ml.size {
            self.dimensions_mut()
                .set_margin_physical(PhysicalSide::Left, lp.to_px(containing_block.width()))
        }
        if let LengthPercentageOrAuto::LengthPercentage(lp) = mr.size {
            self.dimensions_mut()
                .set_margin_physical(PhysicalSide::Right, lp.to_px(containing_block.width()))
        }
        if let LengthPercentageOrAuto::LengthPercentage(lp) = mb.size {
            self.dimensions_mut()
                .set_margin_physical(PhysicalSide::Bottom, lp.to_px(containing_block.height()))
        }
        if let LengthPercentageOrAuto::LengthPercentage(lp) = mt.size {
            self.dimensions_mut()
                .set_margin_physical(PhysicalSide::Top, lp.to_px(containing_block.height()))
        }

        let inline_start_coord = compute_inline_start_coord(&self.dimensions(), containing_block);
        self.dimensions_mut()
            .set_inline_start_coord(inline_start_coord, containing_block.writing_mode());
    }

    /// Corresponds to CSS 2.1 section 10.6.3.  Currently no other sections are implemented.
    /// https://www.w3.org/TR/2011/REC-CSS2-20110607/visudet.html#normal-block
    fn solve_and_set_block_level_properties(&mut self, context: &LayoutContext) {
        let containing_block = context.containing_block;
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

        let preceeding_sibling_blockwise_space_consumed = if self.is_root() {
            // This is sort of a hack, but if the containing block is the viewport (i.e. this is the
            // root box), then there are no preceeding siblings and thus this should be zero.
            // Without this hack, the block size of the containing block is the viewport block size,
            // meaning boxes will be laid out past the block-end of the viewport (meaning they will
            // not be visible).
            CSSPixelLength::new(0.)
        } else {
            containing_block.self_relative_block_size()
        };
        let block_start_coord = compute_block_start_coord(
            &self.dimensions(),
            preceeding_sibling_blockwise_space_consumed,
            context,
        );
        self.dimensions_mut()
            .set_block_start_coord(block_start_coord, containing_block.writing_mode());
    }
}

impl Layout for BlockLevelBox {
    // TODO: Throughout block-level layout, we use the self_relative_* [1] methods of containing blocks
    // when making calculations.  We need to confirm that this is correct.
    //
    // [1] "self-relative" means the containing block evaluates abstract flow directions against its
    // own writing-mode, rather than that of it's own containing block.
    fn layout(&mut self, context: LayoutContext) {
        self.solve_and_set_inline_level_properties(&context);
        self.solve_and_set_block_level_properties(&context);
        self.layout_children(&context);
        let containing_block = context.containing_block;
        // After we've laid out our children, we have enough information (our intrinsic,
        // content-based size) to flip our block-start coordinate, if it's necessary.
        // https://www.w3.org/TR/css-sizing-3/#intrinsic
        if context.block_start_origin_relative_progression()
            == OriginRelativeProgression::TowardsOrigin
        {
            self.flip_block_start_coord(
                context.layout_viewport_block_size(containing_block.writing_mode()),
                containing_block,
            );
        }

        // After computing and applying values normally through layout, override these values with
        // the author's preferred box sizes (if present).
        self.apply_box_sizing_properties(containing_block);
    }
}

/// https://drafts.csswg.org/css-display/#anonymous
#[derive(Clone, Debug)]
pub struct AnonymousBlockBox {
    base: BaseBox,
    children: Vec<LayoutBox>,
}

impl AnonymousBlockBox {
    pub(crate) fn new(node: NodeRef, formatting_context: FormattingContextRef) -> Self {
        Self {
            base: BaseBox::new(node, formatting_context),
            children: Vec::new(),
        }
    }

    pub(crate) fn add_child(&mut self, child: LayoutBox) {
        self.children.push(child)
    }

    fn children(&self) -> &Vec<LayoutBox> {
        &self.children
    }

    fn children_mut(&mut self) -> &mut Vec<LayoutBox> {
        &mut self.children
    }
}

impl BaseLayoutBoxBehavior for AnonymousBlockBox {
    layout_box_behavior_base_box_passthrough_impls!();
}

impl ApplyBoxSizingProperties for AnonymousBlockBox {
    apply_box_sizing_properties_base_box_passthrough_impls!();
}

impl DumpLayoutFormat for AnonymousBlockBox {
    fn dump_layout_format(&self) -> String {
        // Anonymous boxes are not generated by an element of the DOM, so simply print the name
        // of this struct for the dump-layout display.
        "AnonymousBlockBox".to_string()
    }
}

#[derive(Copy, Clone, Debug)]
struct SolveInlineSizeInput {
    containing_block: ContainingBlock,
    margin_inline_start: LengthPercentageOrAuto,
    margin_inline_end: LengthPercentageOrAuto,
    border_inline_start: CSSPixelLength,
    border_inline_end: CSSPixelLength,
    padding_inline_start: LengthPercentage,
    padding_inline_end: LengthPercentage,
    inline_size: LengthPercentageOrAuto,
}

struct SolveInlineSizeOutput {
    margin_inline_start: CSSPixelLength,
    margin_inline_end: CSSPixelLength,
    inline_size: CSSPixelLength,
}

/// While the spec _should_ be the correct way to calculate these inline properties, that is not
/// reality.  If there are specified values for inline margin properties, they always override the
/// values calculated by the spec formula.
fn solve_block_level_inline_size(input: SolveInlineSizeInput) -> SolveInlineSizeOutput {
    let mut spec_inline_sizes = solve_block_level_inline_size_to_spec(input);
    if let LengthPercentageOrAuto::LengthPercentage(lp) = input.margin_inline_start {
        spec_inline_sizes.margin_inline_start =
            lp.to_px(input.containing_block.self_relative_inline_size())
    }
    if let LengthPercentageOrAuto::LengthPercentage(lp) = input.margin_inline_end {
        spec_inline_sizes.margin_inline_end =
            lp.to_px(input.containing_block.self_relative_inline_size())
    }
    spec_inline_sizes
}

/// Pure function to determine used inline-wise direction sizes for a block-level box.
///
/// Corresponds to CSS 2.1 section 10.3.3.  https://www.w3.org/TR/2011/REC-CSS2-20110607/visudet.html#blockwidth
/// No other sections besides 10.3.3 are currently handled.
fn solve_block_level_inline_size_to_spec(input: SolveInlineSizeInput) -> SolveInlineSizeOutput {
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
            margin_inline_end = available_inline_space
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

/// Computes the block start coordinate value (`x` or `y` depending on the writing mode) for
/// the given box according to the rules of block layout.
fn compute_block_start_coord(
    box_dimensions: &Dimensions,
    preceeding_sibling_blockwise_space_consumed: CSSPixelLength,
    layout_context: &LayoutContext,
) -> CSSFloat {
    let containing_block = layout_context.containing_block;
    (containing_block.self_relative_block_start_coord()
        + preceeding_sibling_blockwise_space_consumed
        // Since block_start_coord is the start of the box content, also factor in this boxes
        // margin, border, and padding to the calculation.
        + box_dimensions.get_mbp(FlowSide::BlockStart, containing_block.writing_mode(), containing_block.direction())).px()
}

struct ComputeFlippedBlockStartCoordInput {
    layout_viewport_block_size: CSSPixelLength,
    containing_block_block_size: CSSPixelLength,
    /// The content size of the given box to flip.  The given box must have laid out its children
    /// in order to get this value.
    content_box_block_size: CSSPixelLength,
    preferred_block_size: LengthPercentageOrAuto,
    /// The block start coordinate that was computed for "normal" block progression
    /// (`OriginRelativeProgression::AwayFromOrigin`).
    current_block_start_coord: CSSFloat,
}

fn compute_flipped_block_start_coord(input: ComputeFlippedBlockStartCoordInput) -> CSSFloat {
    let ComputeFlippedBlockStartCoordInput {
        layout_viewport_block_size,
        containing_block_block_size,
        content_box_block_size,
        preferred_block_size,
        current_block_start_coord,
    } = input;

    // If the given box has a preferred (extrinsic) block size, we must use that to position it.
    // Otherwise, use the content (intrinsic) block size calculated during layout.
    let block_size_for_positioning = if preferred_block_size != LengthPercentageOrAuto::Auto {
        preferred_block_size.to_px(containing_block_block_size)
    } else {
        content_box_block_size
    };

    (layout_viewport_block_size - current_block_start_coord - block_size_for_positioning).px()
}

fn compute_inline_start_coord(
    box_dimensions: &Dimensions,
    containing_block: ContainingBlock,
) -> CSSFloat {
    match OriginRelativeProgression::inline_start_origin_relative_direction(
        containing_block.writing_mode(),
        containing_block.direction(),
    ) {
        OriginRelativeProgression::AwayFromOrigin => {
            let mbp_inline_start = box_dimensions.get_mbp(
                FlowSide::InlineStart,
                containing_block.writing_mode(),
                containing_block.direction(),
            );
            (containing_block.self_relative_inline_start_coord() + mbp_inline_start).px()
        }
        OriginRelativeProgression::TowardsOrigin => {
            let containing_block_inline_end_coord = containing_block
                .self_relative_inline_start_coord()
                + containing_block.self_relative_inline_size();
            let inline_border_box_size =
                box_dimensions.border_box_inline_size(containing_block.writing_mode());
            (containing_block_inline_end_coord
                - inline_border_box_size
                - box_dimensions.get(
                    FlowSide::InlineStart,
                    BoxComponent::Margin,
                    containing_block.writing_mode(),
                    containing_block.direction(),
                ))
            .px()
        }
    }
}
