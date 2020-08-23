use crate::layout::flow::BlockFormattingContext;
use crate::layout::layout_box::LayoutBox;
use crate::layout::dimensions::{LogicalDimensions, PhysicalDimensions};

/// A formatting context is the environment into which a set of related boxes are laid out.
/// Different formatting contexts lay out their boxes according to different rules.
///
/// https://drafts.csswg.org/css-display/#formatting-context
pub enum FormattingContext {
    Block(BlockFormattingContext),
    Inline
}

/// A formatting context can contain sub-formatting-contexts and boxes.
pub enum FormattingContextContent {
    SubContext(QualifiedFormattingContext),
    /// The root of some amount of boxes.
    Box(LayoutBox)
}

pub enum QualifiedFormattingContext {
    /// Layout of this formatting context is independent of its parent context.
    /// https://www.w3.org/TR/css-display-3/#independent-formatting-context
    Independent(FormattingContext),
    /// Layout of this formatting context is dependent on the formatting context of its parent.
    Dependent(FormattingContext)
}

pub trait Layout {
    fn layout(containing_block: PhysicalDimensions);
}
