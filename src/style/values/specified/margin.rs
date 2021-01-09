use crate::style::properties::PropertyDeclaration;
use crate::style::values::specified::LengthPercentageOrAuto;
use crate::style::values::CssValueParse;
use crate::style::StyleParseErrorKind;
use cssparser::{ParseError, Parser};

// pub fn parse_shorthand<'i, 't, T>(
//     input: &mut Parser<'i, 't>,
// ) -> Result<(T, T, T, T), ParseError<'i, StyleParseErrorKind<'i>>> {
//
// }

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

    declarations.push(PropertyDeclaration::MarginTop(Margin { lp_or_auto: top }));
    declarations.push(PropertyDeclaration::MarginRight(Margin {
        lp_or_auto: right,
    }));
    declarations.push(PropertyDeclaration::MarginBottom(Margin {
        lp_or_auto: bottom,
    }));
    declarations.push(PropertyDeclaration::MarginLeft(Margin { lp_or_auto: left }));
    Ok(())
}

/// Specified value for `margin-top`.
///
/// https://www.w3.org/TR/css-box-3/#margin-physical
#[derive(Clone, Debug)]
pub struct Margin {
    pub lp_or_auto: LengthPercentageOrAuto,
}

impl CssValueParse for Margin {
    fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        input
            .try_parse(|i| LengthPercentageOrAuto::parse(i))
            .map(|lp_or_auto| Margin { lp_or_auto })
    }
}
