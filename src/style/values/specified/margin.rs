use crate::style::values::specified::LengthPercentage;
use crate::style::StyleParseErrorKind;
use cssparser::{ParseError, Parser};

/// Specified values for `margin-bottom`.
///
/// https://www.w3.org/TR/2018/WD-css-box-3-20181218/#property-index
#[derive(Clone, Debug)]
pub struct MarginBottom {
    // TODO: This should also support the `auto` keyword
    pub length_percentage: LengthPercentage,
}

impl MarginBottom {
    pub fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        input
            .try_parse(|i| LengthPercentage::parse(i))
            .map(|lp| MarginBottom {
                length_percentage: lp,
            })
    }
}

/// Specified values for `margin-left`.
///
/// https://www.w3.org/TR/2018/WD-css-box-3-20181218/#property-index
#[derive(Clone, Debug)]
pub struct MarginLeft {
    // TODO: This should also support the `auto` keyword
    pub length_percentage: LengthPercentage,
}

impl MarginLeft {
    pub fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        input
            .try_parse(|i| LengthPercentage::parse(i))
            .map(|lp| MarginLeft {
                length_percentage: lp,
            })
    }
}

/// Specified values for `margin-right`.
///
/// https://www.w3.org/TR/2018/WD-css-box-3-20181218/#property-index
#[derive(Clone, Debug)]
pub struct MarginRight {
    // TODO: This should also support the `auto` keyword
    pub length_percentage: LengthPercentage,
}

impl MarginRight {
    pub fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        input
            .try_parse(|i| LengthPercentage::parse(i))
            .map(|lp| MarginRight {
                length_percentage: lp,
            })
    }
}

/// Specified values for `margin-top`.
///
/// https://www.w3.org/TR/2018/WD-css-box-3-20181218/#property-index
#[derive(Clone, Debug)]
pub struct MarginTop {
    // TODO: This should also support the `auto` keyword
    pub length_percentage: LengthPercentage,
}

impl MarginTop {
    pub fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        input
            .try_parse(|i| LengthPercentage::parse(i))
            .map(|lp| MarginTop {
                length_percentage: lp,
            })
    }
}
