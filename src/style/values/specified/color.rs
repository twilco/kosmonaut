use crate::style::{StyleParseErrorKind, ValueParseErrorKind};
use cssparser::{
    BasicParseErrorKind, Color as CSSParserColor, ColorComponentParser, ParseError, ParseErrorKind,
    Parser, RGBA,
};

/// Specified values for the `color` property.
///
/// https://www.w3.org/TR/css-color-3/#foreground
#[derive(Clone, Copy, Debug)]
pub enum Color {
    /// A `<color>`-unit.
    Unit(ColorUnit),
    /// The `inherit` keyword.
    Inherit,
}

impl Color {
    pub fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        if let Ok(color_unit) = input.try_parse(|i| ColorUnit::parse(i)) {
            return Ok(Color::Unit(color_unit));
        };

        try_match_ident_ignore_ascii_case! { input,
            "inherit" => Ok(Color::Inherit),
        }
    }

    /// According to https://www.w3.org/TR/css-color-3/#foreground, the initial value of the `color`
    /// property "depends on the user agent".  We'll choose black for now.
    pub fn initial_value() -> Self {
        Color::Unit(ColorUnit::black())
    }
}

/// A specified `<color>`-unit value, named `ColorUnit` to disambiguate it from the `color` property.
///
/// https://www.w3.org/TR/css-color-3/#valuea-def-color
// TODO: There is currently no computed counterpart to this type, as we currently represent computed
// color units as simply `cssparser::RGBA`.  This might need to change eventually.
#[derive(Clone, Copy, Debug)]
pub enum ColorUnit {
    /// The `currentColor` keyword.
    CurrentColor,
    /// A numeric `color`.
    Numeric(cssparser::RGBA),
}

impl ColorUnit {
    pub fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        let component_parser = ComponentParser {};
        match input.try_parse(|i| CSSParserColor::parse_with(&component_parser, i)) {
            Ok(value) => Ok(match value {
                CSSParserColor::CurrentColor => ColorUnit::CurrentColor,
                CSSParserColor::RGBA(rgba) => ColorUnit::Numeric(rgba),
            }),
            Err(e) => match e.kind {
                ParseErrorKind::Basic(BasicParseErrorKind::UnexpectedToken(t)) => {
                    Err(e.location.new_custom_error(StyleParseErrorKind::ValueError(
                        ValueParseErrorKind::InvalidColor(t),
                    )))
                }
                _ => Err(e),
            },
        }
    }

    pub fn black() -> ColorUnit {
        ColorUnit::Numeric(RGBA {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 1,
        })
    }
}

struct ComponentParser;
impl<'i> ColorComponentParser<'i> for ComponentParser {
    type Error = StyleParseErrorKind<'i>;
}
