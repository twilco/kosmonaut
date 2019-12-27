use crate::style::values::specified::{ColorUnit, NoCalcLength};
use crate::style::StyleParseErrorKind;
use cssparser::{ParseError, Parser, Token};

/// Specified `border-bottom-color` values.
///
/// https://www.w3.org/TR/2017/CR-css-backgrounds-3-20171017/#the-border-color
#[derive(Clone, Copy, Debug)]
pub struct BorderBottomColor {
    pub color: ColorUnit,
}

impl BorderBottomColor {
    pub fn initial_value() -> BorderBottomColor {
        BorderBottomColor {
            color: ColorUnit::CurrentColor,
        }
    }

    pub fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        input
            .try_parse(|i| ColorUnit::parse(i))
            .map(|color| BorderBottomColor { color })
    }
}

/// Specified `border-left-color` values.
///
/// https://www.w3.org/TR/2017/CR-css-backgrounds-3-20171017/#the-border-color
#[derive(Clone, Copy, Debug)]
pub struct BorderLeftColor {
    pub color: ColorUnit,
}

impl BorderLeftColor {
    pub fn initial_value() -> BorderLeftColor {
        BorderLeftColor {
            color: ColorUnit::CurrentColor,
        }
    }

    pub fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        input
            .try_parse(|i| ColorUnit::parse(i))
            .map(|color| BorderLeftColor { color })
    }
}

/// Specified `border-right-color` values.
///
/// https://www.w3.org/TR/2017/CR-css-backgrounds-3-20171017/#the-border-color
#[derive(Clone, Copy, Debug)]
pub struct BorderRightColor {
    pub color: ColorUnit,
}

impl BorderRightColor {
    pub fn initial_value() -> BorderRightColor {
        BorderRightColor {
            color: ColorUnit::CurrentColor,
        }
    }

    pub fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        input
            .try_parse(|i| ColorUnit::parse(i))
            .map(|color| BorderRightColor { color })
    }
}

/// Specified `border-top-color` values.
///
/// https://www.w3.org/TR/2017/CR-css-backgrounds-3-20171017/#the-border-color
#[derive(Clone, Copy, Debug)]
pub struct BorderTopColor {
    pub color: ColorUnit,
}

impl BorderTopColor {
    pub fn initial_value() -> BorderTopColor {
        BorderTopColor {
            color: ColorUnit::CurrentColor,
        }
    }

    pub fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        input
            .try_parse(|i| ColorUnit::parse(i))
            .map(|color| BorderTopColor { color })
    }
}

/// Specified `border-bottom-width` values.
///
/// https://www.w3.org/TR/2017/CR-css-backgrounds-3-20171017/#the-border-width
#[derive(Clone, Copy, Debug)]
pub struct BorderBottomWidth {
    pub line_width: LineWidth,
}

impl BorderBottomWidth {
    pub fn initial_value() -> BorderBottomWidth {
        BorderBottomWidth {
            line_width: LineWidth::Medium,
        }
    }

    pub fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        input
            .try_parse(|i| LineWidth::parse(i))
            .map(|line_width| BorderBottomWidth { line_width })
    }
}

/// Specified `border-left-width` values.
///
/// https://www.w3.org/TR/2017/CR-css-backgrounds-3-20171017/#the-border-width
#[derive(Clone, Copy, Debug)]
pub struct BorderLeftWidth {
    pub line_width: LineWidth,
}

impl BorderLeftWidth {
    pub fn initial_value() -> BorderLeftWidth {
        BorderLeftWidth {
            line_width: LineWidth::Medium,
        }
    }

    pub fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        input
            .try_parse(|i| LineWidth::parse(i))
            .map(|line_width| BorderLeftWidth { line_width })
    }
}

/// Specified `border-right-width` values.
///
/// https://www.w3.org/TR/2017/CR-css-backgrounds-3-20171017/#the-border-width
#[derive(Clone, Copy, Debug)]
pub struct BorderRightWidth {
    pub line_width: LineWidth,
}

impl BorderRightWidth {
    pub fn initial_value() -> BorderRightWidth {
        BorderRightWidth {
            line_width: LineWidth::Medium,
        }
    }

    pub fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        input
            .try_parse(|i| LineWidth::parse(i))
            .map(|line_width| BorderRightWidth { line_width })
    }
}

/// Specified `border-top-width` values.
///
/// https://www.w3.org/TR/2017/CR-css-backgrounds-3-20171017/#the-border-width
#[derive(Clone, Copy, Debug)]
pub struct BorderTopWidth {
    pub line_width: LineWidth,
}

impl BorderTopWidth {
    pub fn initial_value() -> BorderTopWidth {
        BorderTopWidth {
            line_width: LineWidth::Medium,
        }
    }

    pub fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        input
            .try_parse(|i| LineWidth::parse(i))
            .map(|line_width| BorderTopWidth { line_width })
    }
}

/// Border `<line-width>` values.
///
/// https://www.w3.org/TR/2017/CR-css-backgrounds-3-20171017/#typedef-line-width
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LineWidth {
    Length(NoCalcLength),
    Thin,
    Medium,
    Thick,
}

impl LineWidth {
    pub fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        let start = input.state();
        let location = input.current_source_location();
        let token = input.next()?;
        match *token {
            Token::Dimension {
                value, ref unit, ..
            } => match NoCalcLength::parse_dimension(value, unit) {
                Ok(no_calc_len) => {
                    return Ok(LineWidth::Length(no_calc_len));
                }
                Err(_) => return Err(location.new_unexpected_token_error(token.clone())),
            },
            _ => {}
        };
        input.reset(&start);
        try_match_ident_ignore_ascii_case! { input,
            "thin" => Ok(LineWidth::Thin),
            "medium" => Ok(LineWidth::Medium),
            "thick" => Ok(LineWidth::Thick),
        }
    }
}
