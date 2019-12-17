use crate::style::values::specified::LengthPercentage;
use crate::style::StyleParseErrorKind;
use cssparser::{ParseError, Parser};

/// Specified values for `padding-bottom`.
///
/// https://www.w3.org/TR/2018/WD-css-box-3-20181218/#property-index
#[derive(Clone, Debug)]
pub struct PaddingBottom {
    pub length_percentage: LengthPercentage,
}

impl PaddingBottom {
    pub fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        input
            .try_parse(|i| LengthPercentage::parse(i))
            .map(|lp| PaddingBottom {
                length_percentage: lp,
            })
    }
}

/// Specified values for `padding-left`.
///
/// https://www.w3.org/TR/2018/WD-css-box-3-20181218/#property-index
#[derive(Clone, Debug)]
pub struct PaddingLeft {
    pub length_percentage: LengthPercentage,
}

impl PaddingLeft {
    pub fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        input
            .try_parse(|i| LengthPercentage::parse(i))
            .map(|lp| PaddingLeft {
                length_percentage: lp,
            })
    }
}

/// Specified values for `padding-right`.
///
/// https://www.w3.org/TR/2018/WD-css-box-3-20181218/#property-index
#[derive(Clone, Debug)]
pub struct PaddingRight {
    pub length_percentage: LengthPercentage,
}

impl PaddingRight {
    pub fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        input
            .try_parse(|i| LengthPercentage::parse(i))
            .map(|lp| PaddingRight {
                length_percentage: lp,
            })
    }
}

/// Specified values for `padding-top`.
///
/// https://www.w3.org/TR/2018/WD-css-box-3-20181218/#property-index
#[derive(Clone, Debug)]
pub struct PaddingTop {
    pub length_percentage: LengthPercentage,
}

impl PaddingTop {
    pub fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        input
            .try_parse(|i| LengthPercentage::parse(i))
            .map(|lp| PaddingTop {
                length_percentage: lp,
            })
    }
}
