use crate::style::values::specified::LengthPercentage;
use crate::style::StyleParseErrorKind;
use cssparser::{ParseError, Parser};
use crate::style::properties::PropertyDeclaration;

pub fn parse_padding_shorthand_into<'i, 't>(
    declarations: &mut Vec<PropertyDeclaration>,
    input: &mut Parser<'i, 't>,
) -> Result<(), ParseError<'i, StyleParseErrorKind<'i>>> {
    let (first_val, second_val, third_val, fourth_val) = (
        input.try_parse(|i| LengthPercentage::parse(i)),
        input.try_parse(|i| LengthPercentage::parse(i)),
        input.try_parse(|i| LengthPercentage::parse(i)),
        input.try_parse(|i| LengthPercentage::parse(i)),
    );
    // Based on how many LP-auto values we were able to successfully parse, apply the longhands per
    // spec: https://drafts.csswg.org/css-box-3/#padding-shorthand
    let (top, right, bottom, left) = if fourth_val.is_ok() {
        (
            first_val.unwrap(),
            second_val.unwrap(),
            third_val.unwrap(),
            fourth_val.unwrap(),
        )
    } else if third_val.is_ok() {
        let second = second_val.unwrap();
        (first_val.unwrap(), second, third_val.unwrap(), second)
    } else if second_val.is_ok() {
        let (first, second) = (first_val.unwrap(), second_val.unwrap());
        (first, second, first, second)
    } else if first_val.is_ok() {
        let val = first_val.unwrap();
        (val, val, val, val)
    } else {
        return Err(first_val.unwrap_err());
    };

    declarations.push(PropertyDeclaration::PaddingTop(PaddingTop {
        length_percentage: top,
    }));
    declarations.push(PropertyDeclaration::PaddingRight(PaddingRight {
        length_percentage: right,
    }));
    declarations.push(PropertyDeclaration::PaddingBottom(PaddingBottom {
        length_percentage: bottom,
    }));
    declarations.push(PropertyDeclaration::PaddingLeft(PaddingLeft {
        length_percentage: left,
    }));
    Ok(())
}

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
