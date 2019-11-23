use crate::style::StyleParseErrorKind;

use crate::style::values::specified::font::FontSize;
use cssparser::{ParseError, Parser};

// TODO: This may not be needed.
pub fn parse_font_size<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<FontSize, ParseError<'i, StyleParseErrorKind<'i>>> {
    FontSize::parse(input)
}
