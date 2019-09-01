use crate::style::StyleParseErrorKind;

use cssparser::{Parser, ParseError};
use crate::style::values::specified::font::FontSize;
use crate::style::values::specified;

pub fn parse_font_size<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<FontSize, ParseError<'i, StyleParseErrorKind<'i>>> {
    specified::FontSize::parse(input)
}
