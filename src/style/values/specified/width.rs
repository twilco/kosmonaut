use crate::style::values::specified::LengthPercentageOrAuto;
use crate::style::values::CssValueParse;
use crate::style::StyleParseErrorKind;
use cssparser::{ParseError, Parser};

/// Specified value for the `width` property.
///
/// https://www.w3.org/TR/css-sizing-3/#property-index
// TODO: Need to support various other value types, such as `{min, max}-content`.
#[derive(Clone, Copy, Debug)]
pub enum Width {
    LengthPercentageOrAuto(LengthPercentageOrAuto),
}

impl CssValueParse for Width {
    fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        input
            .try_parse(|i| LengthPercentageOrAuto::parse(i))
            .map(Width::LengthPercentageOrAuto)
    }
}
