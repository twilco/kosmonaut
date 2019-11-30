use crate::style::StyleParseErrorKind;
use cssparser::{ParseError, Parser};

use crate::style::values::specified::length::LengthPercentage;

/// The default font size.
pub const FONT_MEDIUM_PX: i32 = 16;

#[derive(Clone, Debug, PartialEq)]
/// A specified font-size value
pub enum FontSize {
    /// A keyword size, e.g. medium
    Keyword(KeywordSize),
    // font-size: larger
    // Larger,
    /// A length; e.g. 10px.
    Length(LengthPercentage),
    // font-size: smaller
    // Smaller,
}

/// CSS font keywords
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(missing_docs)]
pub enum KeywordSize {
    XXSmall,
    XSmall,
    Small,
    Medium,
    Large,
    XLarge,
    XXLarge,
    XXXLarge,
}

impl KeywordSize {
    pub fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        try_match_ident_ignore_ascii_case! { input,
            "xx-small" => Ok(KeywordSize::XXSmall),
            "x-small" => Ok(KeywordSize::XSmall),
            "small" => Ok(KeywordSize::Small),
            "medium" => Ok(KeywordSize::Medium),
            "large" => Ok(KeywordSize::Large),
            "x-large" => Ok(KeywordSize::XLarge),
            "xx-large" => Ok(KeywordSize::XXLarge),
            "xxx-large" => Ok(KeywordSize::XXXLarge),
        }
    }
}

impl FontSize {
    pub fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        if let Ok(lp) = input.try_parse(|i| LengthPercentage::parse(i)) {
            return Ok(FontSize::Length(lp));
        }
        match input.try_parse(|i| KeywordSize::parse(i)) {
            Ok(kws) => Ok(FontSize::Keyword(kws)),
            Err(parse_err) => Err(parse_err),
        }
    }

    pub fn initial_value() -> Self {
        FontSize::Keyword(KeywordSize::Medium)
    }
}
