use crate::style::properties::PropertyDeclaration;
use crate::style::values::specified::LengthPercentageOrAuto;
use crate::style::StyleParseErrorKind;
use cssparser::{ParseError, Parser};

pub fn parse_margin_shorthand_into<'i, 't>(
    declarations: &mut Vec<PropertyDeclaration>,
    input: &mut Parser<'i, 't>,
) -> Result<(), ParseError<'i, StyleParseErrorKind<'i>>> {
    let (first_val, second_val, third_val, fourth_val) = (
        input.try_parse(|i| LengthPercentageOrAuto::parse(i)),
        input.try_parse(|i| LengthPercentageOrAuto::parse(i)),
        input.try_parse(|i| LengthPercentageOrAuto::parse(i)),
        input.try_parse(|i| LengthPercentageOrAuto::parse(i)),
    );
    // Based on how many LP-auto values we were able to successfully parse, apply the longhands per
    // spec: https://drafts.csswg.org/css-box-3/#margin-shorthand
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

    declarations.push(PropertyDeclaration::MarginTop(MarginTop {
        lp_or_auto: top,
    }));
    declarations.push(PropertyDeclaration::MarginRight(MarginRight {
        lp_or_auto: right,
    }));
    declarations.push(PropertyDeclaration::MarginBottom(MarginBottom {
        lp_or_auto: bottom,
    }));
    declarations.push(PropertyDeclaration::MarginLeft(MarginLeft {
        lp_or_auto: left,
    }));
    Ok(())
}

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
            .map(|lp_or_auto| MarginBottom { lp_or_auto })
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
            .map(|lp_or_auto| MarginLeft { lp_or_auto })
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
            .map(|lp_or_auto| MarginRight { lp_or_auto })
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
            .map(|lp_or_auto| MarginTop { lp_or_auto })
    }
}
