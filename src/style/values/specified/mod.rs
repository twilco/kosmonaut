/// The specified value of a CSS property is the value it receives from the document's style sheet.
/// The specified value for a given property is determined according to the following rules:
///
///   1. If the document's style sheet explicitly specifies a value for the property, the given
///      value will be used.
///   2. If the document's style sheet doesn't specify a value but it is an inherited property, the
///      value will be taken from the parent element.
///   3. If none of the above pertain, the element's initial value will be used.
///
/// https://developer.mozilla.org/en-US/docs/Web/CSS/specified_value
/// https://www.w3.org/TR/CSS22/cascade.html#specified-value
pub mod background;
pub mod border;
pub mod color;
pub mod font;
pub mod height;
pub mod length;
pub mod margin;
pub mod padding;
pub mod width;

pub use background::BackgroundColor;

pub use border::BorderBottomWidth;
pub use border::BorderColor;
pub use border::BorderLeftWidth;
pub use border::BorderRightWidth;
pub use border::BorderTopWidth;

pub use color::Color;
pub use color::ColorUnit;

pub use font::FontSize;
pub use font::FONT_MEDIUM_PX;

pub use height::Height;

pub use length::AbsoluteLength;
pub use length::LengthPercentage;
pub use length::LengthPercentageOrAuto;
pub use length::NoCalcLength;

pub use margin::Margin;
pub use padding::Padding;

use crate::style::values::CssValueParse;
use crate::style::StyleParseErrorKind;
use cssparser::{ParseError, Parser};
use std::fmt::Debug;
pub use width::Width;

#[derive(Clone, Copy, Debug)]
pub struct ParsedShorthandSides<T> {
    pub top: T,
    pub right: T,
    pub bottom: T,
    pub left: T,
}

/// Tries to parse shorthand values for the given `T`, returning the resulting parsed sides or a
/// parse error if no value could be parsed successfully.
///
/// The order of the parsed values is assigned to sides according to the typical pattern for
/// shorthands.  See the margin shorthand for an example:
/// https://drafts.csswg.org/css-box-3/#margin-shorthand
pub fn parse_shorthand_sides<'i, 't, T: CssValueParse + Clone + Debug>(
    input: &mut Parser<'i, 't>,
) -> Result<ParsedShorthandSides<T>, ParseError<'i, StyleParseErrorKind<'i>>> {
    let (first_val, second_val, third_val, fourth_val) = (
        input.try_parse(|i| T::parse(i)),
        input.try_parse(|i| T::parse(i)),
        input.try_parse(|i| T::parse(i)),
        input.try_parse(|i| T::parse(i)),
    );
    // Based on how many LP-auto values we were able to successfully parse, apply the longhands per
    // spec: https://drafts.csswg.org/css-box-3/#margin-shorthand
    let parsed_sides = if let Ok(fourth) = fourth_val {
        ParsedShorthandSides {
            top: first_val.unwrap(),
            right: second_val.unwrap(),
            bottom: third_val.unwrap(),
            left: fourth,
        }
    } else if let Ok(third) = third_val {
        let second = second_val.unwrap();
        ParsedShorthandSides {
            top: first_val.unwrap(),
            right: second.clone(),
            bottom: third,
            left: second,
        }
    } else if let Ok(second) = second_val {
        let first = first_val.unwrap();
        ParsedShorthandSides {
            top: first.clone(),
            right: second.clone(),
            bottom: first,
            left: second,
        }
    } else if let Ok(first) = first_val {
        ParsedShorthandSides {
            top: first.clone(),
            right: first.clone(),
            bottom: first.clone(),
            left: first,
        }
    } else {
        return Err(first_val.unwrap_err());
    };
    Ok(parsed_sides)
}
