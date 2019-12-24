use crate::style::values::specified::LengthPercentageOrAuto;
use crate::style::StyleParseErrorKind;
use cssparser::{ParseError, Parser};

/// Specified values for the `height` property.
///
/// https://www.w3.org/TR/css-sizing-3/#property-index
// TODO: Need to support various other value types, such as `{min, max}-content`.
#[derive(Clone, Copy, Debug)]
pub enum Height {
    LengthPercentageOrAuto(LengthPercentageOrAuto),
}

impl Height {
    pub fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        input
            .try_parse(|i| LengthPercentageOrAuto::parse(i))
            .map(|lp_or_auto| Height::LengthPercentageOrAuto(lp_or_auto))
    }
}
