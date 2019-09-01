use crate::style::values::specified::length::LengthPercentage;
use cssparser::{Parser, ParseError};
use crate::style::StyleParseErrorKind;

#[derive(Clone, Debug, PartialEq)]
/// A specified font-size value
pub enum FontSize {
    /// A length; e.g. 10px.
    Length(LengthPercentage),
    /// font-size: smaller
    Smaller,
    /// font-size: larger
    Larger,
}

impl FontSize {
    pub fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        if let Ok(lp) =
        input.try_parse(|i| LengthPercentage::parse(i))
        {
            return Ok(FontSize::Length(lp))
        }

        try_match_ident_ignore_ascii_case! { input,
            "smaller" => Ok(FontSize::Smaller),
            "larger" => Ok(FontSize::Larger),
        }
    }
}
