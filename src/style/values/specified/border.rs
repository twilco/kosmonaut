use crate::style::values::specified::{ColorUnit, NoCalcLength};
use crate::style::StyleParseErrorKind;
use cssparser::{ParseError, Parser, Token};
use crate::style::properties::PropertyDeclaration;
use crate::Side;
use crate::style::values::computed::LineStyle;

pub fn parse_border_side_shorthand_into<'i, 't>(
    side: Side,
    declarations: &mut Vec<PropertyDeclaration>,
    input: &mut Parser<'i, 't>,
) -> Result<(), ParseError<'i, StyleParseErrorKind<'i>>> {
    let (line_width, line_style, color) = parse_border_shorthand_inner(input)?;
    if let Some(line_width) = line_width {
        declarations.push(match side {
            Side::Bottom => PropertyDeclaration::BorderBottomWidth(BorderBottomWidth { line_width }),
            Side::Left => PropertyDeclaration::BorderLeftWidth(BorderLeftWidth { line_width }),
            Side::Right => PropertyDeclaration::BorderRightWidth(BorderRightWidth { line_width }),
            Side::Top => PropertyDeclaration::BorderTopWidth(BorderTopWidth { line_width }),
        })
    }
    if let Some(line_style) = line_style {
        declarations.push(match side {
            Side::Bottom => PropertyDeclaration::BorderBottomStyle(line_style),
            Side::Left => PropertyDeclaration::BorderLeftStyle(line_style),
            Side::Right => PropertyDeclaration::BorderRightStyle(line_style),
            Side::Top => PropertyDeclaration::BorderTopStyle(line_style),
        })
    }
    if let Some(color) = color {
        declarations.push(match side {
            Side::Bottom => PropertyDeclaration::BorderBottomColor(BorderBottomColor { color }),
            Side::Left => PropertyDeclaration::BorderLeftColor(BorderLeftColor { color}),
            Side::Right => PropertyDeclaration::BorderRightColor(BorderRightColor { color}),
            Side::Top => PropertyDeclaration::BorderTopColor(BorderTopColor { color}),
        })
    }
    Ok(())
}

// TODO: According to the spec, the `border` shorthand also resets the `border-image` property
// to its initial value. https://www.w3.org/TR/css-backgrounds-3/#border-shorthands
pub fn parse_border_shorthand_into<'i, 't>(
    declarations: &mut Vec<PropertyDeclaration>,
    input: &mut Parser<'i, 't>,
) -> Result<(), ParseError<'i, StyleParseErrorKind<'i>>> {
    let (line_width, line_style, color) = parse_border_shorthand_inner(input)?;
    if let Some(line_width) = line_width {
        declarations.push(PropertyDeclaration::BorderBottomWidth(BorderBottomWidth { line_width }));
        declarations.push(PropertyDeclaration::BorderLeftWidth(BorderLeftWidth { line_width }));
        declarations.push(PropertyDeclaration::BorderRightWidth(BorderRightWidth { line_width }));
        declarations.push(PropertyDeclaration::BorderTopWidth(BorderTopWidth { line_width }));
    }
    if let Some(line_style) = line_style {
        declarations.push(PropertyDeclaration::BorderBottomStyle(line_style));
        declarations.push(PropertyDeclaration::BorderLeftStyle(line_style));
        declarations.push(PropertyDeclaration::BorderRightStyle(line_style));
        declarations.push(PropertyDeclaration::BorderTopStyle(line_style));
    }
    if let Some(color) = color {
        declarations.push(PropertyDeclaration::BorderBottomColor(BorderBottomColor { color }));
        declarations.push(PropertyDeclaration::BorderLeftColor(BorderLeftColor { color}));
        declarations.push(PropertyDeclaration::BorderRightColor(BorderRightColor { color}));
        declarations.push(PropertyDeclaration::BorderTopColor(BorderTopColor { color}));
    }
    Ok(())
}

pub fn parse_border_color_shorthand_into<'i, 't>(
    declarations: &mut Vec<PropertyDeclaration>,
    input: &mut Parser<'i, 't>,
) -> Result<(), ParseError<'i, StyleParseErrorKind<'i>>> {
    let (first_val, second_val, third_val, fourth_val) = (
        input.try_parse(|i| ColorUnit::parse(i)),
        input.try_parse(|i| ColorUnit::parse(i)),
        input.try_parse(|i| ColorUnit::parse(i)),
        input.try_parse(|i| ColorUnit::parse(i)),
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

    declarations.push(PropertyDeclaration::BorderTopColor(BorderTopColor { color: top }));
    declarations.push(PropertyDeclaration::BorderRightColor(BorderRightColor { color: bottom }));
    declarations.push(PropertyDeclaration::BorderLeftColor(BorderLeftColor { color: left }));
    declarations.push(PropertyDeclaration::BorderBottomColor(BorderBottomColor { color: right }));
    Ok(())
}

pub fn parse_border_style_shorthand_into<'i, 't>(
    declarations: &mut Vec<PropertyDeclaration>,
    input: &mut Parser<'i, 't>,
) -> Result<(), ParseError<'i, StyleParseErrorKind<'i>>> {
    let (first_val, second_val, third_val, fourth_val) = (
        input.try_parse(|i| LineStyle::parse(i)),
        input.try_parse(|i| LineStyle::parse(i)),
        input.try_parse(|i| LineStyle::parse(i)),
        input.try_parse(|i| LineStyle::parse(i)),
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

    declarations.push(PropertyDeclaration::BorderTopStyle(top));
    declarations.push(PropertyDeclaration::BorderRightStyle(right));
    declarations.push(PropertyDeclaration::BorderBottomStyle(bottom));
    declarations.push(PropertyDeclaration::BorderLeftStyle(left));
    Ok(())
}

pub fn parse_border_width_shorthand_into<'i, 't>(
    declarations: &mut Vec<PropertyDeclaration>,
    input: &mut Parser<'i, 't>,
) -> Result<(), ParseError<'i, StyleParseErrorKind<'i>>> {
    let (first_val, second_val, third_val, fourth_val) = (
        input.try_parse(|i| LineWidth::parse(i)),
        input.try_parse(|i| LineWidth::parse(i)),
        input.try_parse(|i| LineWidth::parse(i)),
        input.try_parse(|i| LineWidth::parse(i)),
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

    declarations.push(PropertyDeclaration::BorderTopWidth(BorderTopWidth {
        line_width: top,
    }));
    declarations.push(PropertyDeclaration::BorderRightWidth(BorderRightWidth {
        line_width: right,
    }));
    declarations.push(PropertyDeclaration::BorderBottomWidth(BorderBottomWidth {
        line_width: bottom,
    }));
    declarations.push(PropertyDeclaration::BorderLeftWidth(BorderLeftWidth {
        line_width: left,
    }));
    Ok(())
}

fn parse_border_shorthand_inner<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<(Option<LineWidth>, Option<LineStyle>, Option<ColorUnit>), ParseError<'i, StyleParseErrorKind<'i>>> {
    // There are three optional components in the `border-<side>` shorthand that can appear in any
    // order.
    let (mut line_width, mut line_style, mut color) = (None, None, None);
    for _ in 0..3 {
        let parsed_line_width = input.try_parse(|i| LineWidth::parse(i));
        if parsed_line_width.is_ok() {
            if line_width.is_none() {
                line_width = Some(parsed_line_width.unwrap());
                continue;
            }
        }
        let parsed_line_style = input.try_parse(|i| LineStyle::parse(i));
        if parsed_line_style.is_ok() {
            if line_style.is_none() {
                line_style = Some(parsed_line_style.unwrap());
                continue;
            }
        }
        let parsed_color = input.try_parse(|i| ColorUnit::parse(i));
        if parsed_color.is_ok() {
            if color.is_none() {
                color = Some(parsed_color.unwrap());
                continue;
            }
        }

        let location = input.current_source_location();
        return Err(location.new_unexpected_token_error(input.next()?.clone()))
    }
    Ok((line_width, line_style, color))
}

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
