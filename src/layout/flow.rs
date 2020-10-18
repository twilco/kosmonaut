use crate::layout::containing_block::ContainingBlock;
use crate::style::values::computed::length::{
    CSSPixelLength, LengthPercentage, LengthPercentageOrAuto,
};
use crate::style::values::computed::Direction;
use crate::style::values::used::ToPx;

/// Sides relative to the flow of the page, rather than physical sides (e.g. left, top, ...).
///
/// https://drafts.csswg.org/css-writing-modes-4/#logical-directions
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FlowSide {
    BlockStart,
    BlockEnd,
    InlineStart,
    InlineEnd,
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

/// Pure function to determine used inline-wise direction values.
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
    let available_inline_space_px = containing_block.inline_size() - margin_box_inline_size;
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
        margin_inline_start: margin_inline_start.to_px(containing_block.inline_size()),
        margin_inline_end: margin_inline_end.to_px(containing_block.inline_size()),
        inline_size: inline_size.to_px(containing_block.inline_size()),
    }
}
