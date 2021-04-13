use crate::values::computed::{ComputeContext, ValueDefault};
use crate::values::CssValueParse;
use crate::StyleParseErrorKind;
use cssparser::{ParseError, Parser};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// https://drafts.csswg.org/css-writing-modes-4/#propdef-direction
pub enum Direction {
    Ltr,
    Rtl,
}

impl Direction {
    pub fn initial_value() -> Direction {
        Direction::Ltr
    }
}

impl CssValueParse for Direction {
    fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        try_match_ident_ignore_ascii_case! { input,
            "ltr" => Ok(Direction::Ltr),
            "rtl" => Ok(Direction::Rtl),
        }
    }
}

impl ValueDefault for Direction {
    type ComputedValue = Direction;

    fn value_default(context: &ComputeContext) -> Self::ComputedValue {
        context.parent_computed_values.direction
    }
}

/// Computed `writing-mode` values, which determine the block-flow direction, writing mode, and
/// typographic mode.
///
/// https://drafts.csswg.org/css-writing-modes-4/#propdef-writing-mode
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WritingMode {
    HorizontalTb,
    VerticalRl,
    VerticalLr,
    SidewaysRl,
    SidewaysLr,
}

impl WritingMode {
    pub fn initial_value() -> WritingMode {
        WritingMode::HorizontalTb
    }

    pub fn is_horizontal(&self) -> bool {
        match self {
            WritingMode::VerticalLr
            | WritingMode::VerticalRl
            | WritingMode::SidewaysLr
            | WritingMode::SidewaysRl => false,
            WritingMode::HorizontalTb => true,
        }
    }
}

impl CssValueParse for WritingMode {
    fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        try_match_ident_ignore_ascii_case! { input,
            "horizontal-tb" => Ok(WritingMode::HorizontalTb),
            "vertical-rl" => Ok(WritingMode::VerticalRl),
            "vertical-lr" => Ok(WritingMode::VerticalLr),
            "sideways-rl" => Ok(WritingMode::SidewaysRl),
            "sideways-lr" => Ok(WritingMode::SidewaysLr),
        }
    }
}

impl ValueDefault for WritingMode {
    type ComputedValue = WritingMode;

    fn value_default(context: &ComputeContext) -> Self::ComputedValue {
        context.parent_computed_values.writing_mode
    }
}
