use crate::properties::PropertyDeclaration;
use crate::values::specified::{parse_shorthand_sides, LengthPercentageOrAuto};
use crate::values::CssValueParse;
use crate::StyleParseErrorKind;
use cssparser::{ParseError, Parser};

pub fn parse_margin_shorthand_into<'i, 't>(
    declarations: &mut Vec<PropertyDeclaration>,
    input: &mut Parser<'i, 't>,
) -> Result<(), ParseError<'i, StyleParseErrorKind<'i>>> {
    let parsed_sides = parse_shorthand_sides::<LengthPercentageOrAuto>(input)?;
    declarations.push(PropertyDeclaration::MarginTop(Margin {
        lp_or_auto: parsed_sides.top,
    }));
    declarations.push(PropertyDeclaration::MarginRight(Margin {
        lp_or_auto: parsed_sides.right,
    }));
    declarations.push(PropertyDeclaration::MarginBottom(Margin {
        lp_or_auto: parsed_sides.bottom,
    }));
    declarations.push(PropertyDeclaration::MarginLeft(Margin {
        lp_or_auto: parsed_sides.left,
    }));
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
