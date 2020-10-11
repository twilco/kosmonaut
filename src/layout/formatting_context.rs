use crate::layout::dimensions::{LogicalDimensions, PhysicalDimensions};
use crate::layout::layout_box::LayoutBox;
use image::error::UnsupportedErrorKind::Format;
use std::mem::discriminant;

/// A formatting context is the environment into which a set of related boxes are laid out.
/// Different formatting contexts lay out their boxes according to different rules.
///
/// https://drafts.csswg.org/css-display/#formatting-context
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FormattingContext {
    Block,
    Inline,
}

/// A formatting context can contain sub-formatting-contexts and boxes.
pub enum FormattingContextContent {
    SubContext(QualifiedFormattingContext),
    /// The root of some amount of boxes.
    Box(LayoutBox),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum QualifiedFormattingContext {
    /// Layout of this formatting context is independent of its parent context.
    /// https://www.w3.org/TR/css-display-3/#independent-formatting-context
    Independent(FormattingContext),
    /// Layout of this formatting context is dependent on the formatting context of its parent.  For
    /// example, inline formatting contexts are part of (and therefore "dependent" on) their parent
    /// block formatting context.
    /// https://www.w3.org/TR/css-display-3/#inline-formatting-context
    Dependent(FormattingContext),
}

impl QualifiedFormattingContext {
    pub fn is_inline_formatting_context(&self) -> bool {
        match self {
            QualifiedFormattingContext::Independent(fc)
            | QualifiedFormattingContext::Dependent(fc) => fc == &FormattingContext::Inline,
        }
    }
}
