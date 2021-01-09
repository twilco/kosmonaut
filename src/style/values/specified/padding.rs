use crate::style::properties::PropertyDeclaration;
use crate::style::values::specified::LengthPercentage;
use crate::style::values::CssValueParse;
use crate::style::StyleParseErrorKind;
use cssparser::{ParseError, Parser};

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

    declarations.push(PropertyDeclaration::PaddingTop(Padding {
        length_percentage: top,
    }));
    declarations.push(PropertyDeclaration::PaddingRight(Padding {
        length_percentage: right,
    }));
    declarations.push(PropertyDeclaration::PaddingBottom(Padding {
        length_percentage: bottom,
    }));
    declarations.push(PropertyDeclaration::PaddingLeft(Padding {
        length_percentage: left,
    }));
    Ok(())
}

/// Specified value for `padding-<side>`.
///
/// https://www.w3.org/TR/css-box-3/#padding-physical
#[derive(Clone, Debug)]
pub struct Padding {
    pub length_percentage: LengthPercentage,
}

impl CssValueParse for Padding {
    fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        input
            .try_parse(|i| LengthPercentage::parse(i))
            .map(|lp| Padding {
                length_percentage: lp,
            })
    }
}
