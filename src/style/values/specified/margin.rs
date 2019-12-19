use crate::style::values::specified::LengthPercentageOrAuto;
use crate::style::StyleParseErrorKind;
use cssparser::{ParseError, Parser};

/// Specified values for `margin-bottom`.
///
/// https://www.w3.org/TR/2018/WD-css-box-3-20181218/#property-index
#[derive(Clone, Debug)]
pub struct MarginBottom {
    pub lp_or_auto: LengthPercentageOrAuto,
}

impl MarginBottom {
    pub fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        input
            .try_parse(|i| LengthPercentageOrAuto::parse(i))
            .map(|lp_or_auto| MarginBottom {
                lp_or_auto,
            })
    }
}

/// Specified values for `margin-left`.
///
/// https://www.w3.org/TR/2018/WD-css-box-3-20181218/#property-index
#[derive(Clone, Debug)]
pub struct MarginLeft {
    pub lp_or_auto: LengthPercentageOrAuto,
}

impl MarginLeft {
    pub fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        input
            .try_parse(|i| LengthPercentageOrAuto::parse(i))
            .map(|lp_or_auto| MarginLeft {
                lp_or_auto,
            })
    }
}

/// Specified values for `margin-right`.
///
/// https://www.w3.org/TR/2018/WD-css-box-3-20181218/#property-index
#[derive(Clone, Debug)]
pub struct MarginRight {
    pub lp_or_auto: LengthPercentageOrAuto,
}

impl MarginRight {
    pub fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        input
            .try_parse(|i| LengthPercentageOrAuto::parse(i))
            .map(|lp_or_auto| MarginRight {
                lp_or_auto,
            })
    }
}

/// Specified values for `margin-top`.
///
/// https://www.w3.org/TR/2018/WD-css-box-3-20181218/#property-index
#[derive(Clone, Debug)]
pub struct MarginTop {
    pub lp_or_auto: LengthPercentageOrAuto,
}

impl MarginTop {
    pub fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        input
            .try_parse(|i| LengthPercentageOrAuto::parse(i))
            .map(|lp_or_auto| MarginTop {
                lp_or_auto,
            })
    }
}
