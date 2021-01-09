use crate::style::properties::PropertyDeclaration;
use crate::style::values::specified::{parse_shorthand_sides, LengthPercentage};
use crate::style::values::CssValueParse;
use crate::style::StyleParseErrorKind;
use cssparser::{ParseError, Parser};

pub fn parse_padding_shorthand_into<'i, 't>(
    declarations: &mut Vec<PropertyDeclaration>,
    input: &mut Parser<'i, 't>,
) -> Result<(), ParseError<'i, StyleParseErrorKind<'i>>> {
    let parsed_sides = parse_shorthand_sides::<LengthPercentage>(input)?;
    declarations.push(PropertyDeclaration::PaddingTop(Padding {
        length_percentage: parsed_sides.top,
    }));
    declarations.push(PropertyDeclaration::PaddingRight(Padding {
        length_percentage: parsed_sides.right,
    }));
    declarations.push(PropertyDeclaration::PaddingBottom(Padding {
        length_percentage: parsed_sides.bottom,
    }));
    declarations.push(PropertyDeclaration::PaddingLeft(Padding {
        length_percentage: parsed_sides.left,
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
