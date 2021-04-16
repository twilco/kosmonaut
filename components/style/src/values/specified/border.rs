use crate::properties::PropertyDeclaration;
use crate::values::computed::LineStyle;
use crate::values::specified::{parse_shorthand_sides, ColorUnit, NoCalcLength};
use crate::values::CssValueParse;
use crate::StyleParseErrorKind;
use cssparser::{ParseError, Parser, Token};
use primitives::sides::PhysicalSide;

pub(crate) fn parse_border_side_shorthand_into<'i, 't>(
    side: PhysicalSide,
    declarations: &mut Vec<PropertyDeclaration>,
    input: &mut Parser<'i, 't>,
) -> Result<(), ParseError<'i, StyleParseErrorKind<'i>>> {
    let (line_width, line_style, color) = parse_border_shorthand_inner(input)?;
    if let Some(line_width) = line_width {
        declarations.push(match side {
            PhysicalSide::Bottom => {
                PropertyDeclaration::BorderBottomWidth(BorderBottomWidth { line_width })
            }
            PhysicalSide::Left => {
                PropertyDeclaration::BorderLeftWidth(BorderLeftWidth { line_width })
            }
            PhysicalSide::Right => {
                PropertyDeclaration::BorderRightWidth(BorderRightWidth { line_width })
            }
            PhysicalSide::Top => PropertyDeclaration::BorderTopWidth(BorderTopWidth { line_width }),
        })
    }
    if let Some(line_style) = line_style {
        declarations.push(match side {
            PhysicalSide::Bottom => PropertyDeclaration::BorderBottomStyle(line_style),
            PhysicalSide::Left => PropertyDeclaration::BorderLeftStyle(line_style),
            PhysicalSide::Right => PropertyDeclaration::BorderRightStyle(line_style),
            PhysicalSide::Top => PropertyDeclaration::BorderTopStyle(line_style),
        })
    }
    if let Some(color) = color {
        declarations.push(match side {
            PhysicalSide::Bottom => PropertyDeclaration::BorderBottomColor(BorderColor { color }),
            PhysicalSide::Left => PropertyDeclaration::BorderLeftColor(BorderColor { color }),
            PhysicalSide::Right => PropertyDeclaration::BorderRightColor(BorderColor { color }),
            PhysicalSide::Top => PropertyDeclaration::BorderTopColor(BorderColor { color }),
        })
    }
    Ok(())
}

// TODO: According to the spec, the `border` shorthand also resets the `border-image` property
// to its initial value. https://www.w3.org/TR/css-backgrounds-3/#border-shorthands
pub(crate) fn parse_border_shorthand_into<'i, 't>(
    declarations: &mut Vec<PropertyDeclaration>,
    input: &mut Parser<'i, 't>,
) -> Result<(), ParseError<'i, StyleParseErrorKind<'i>>> {
    let (line_width, line_style, color) = parse_border_shorthand_inner(input)?;
    if let Some(line_width) = line_width {
        declarations.push(PropertyDeclaration::BorderBottomWidth(BorderBottomWidth {
            line_width,
        }));
        declarations.push(PropertyDeclaration::BorderLeftWidth(BorderLeftWidth {
            line_width,
        }));
        declarations.push(PropertyDeclaration::BorderRightWidth(BorderRightWidth {
            line_width,
        }));
        declarations.push(PropertyDeclaration::BorderTopWidth(BorderTopWidth {
            line_width,
        }));
    }
    if let Some(line_style) = line_style {
        declarations.push(PropertyDeclaration::BorderBottomStyle(line_style));
        declarations.push(PropertyDeclaration::BorderLeftStyle(line_style));
        declarations.push(PropertyDeclaration::BorderRightStyle(line_style));
        declarations.push(PropertyDeclaration::BorderTopStyle(line_style));
    }
    if let Some(color) = color {
        declarations.push(PropertyDeclaration::BorderBottomColor(BorderColor {
            color,
        }));
        declarations.push(PropertyDeclaration::BorderLeftColor(BorderColor { color }));
        declarations.push(PropertyDeclaration::BorderRightColor(BorderColor { color }));
        declarations.push(PropertyDeclaration::BorderTopColor(BorderColor { color }));
    }
    Ok(())
}

pub(crate) fn parse_border_color_shorthand_into<'i, 't>(
    declarations: &mut Vec<PropertyDeclaration>,
    input: &mut Parser<'i, 't>,
) -> Result<(), ParseError<'i, StyleParseErrorKind<'i>>> {
    let parsed_sides = parse_shorthand_sides::<ColorUnit>(input)?;
    declarations.push(PropertyDeclaration::BorderTopColor(BorderColor {
        color: parsed_sides.top,
    }));
    declarations.push(PropertyDeclaration::BorderRightColor(BorderColor {
        color: parsed_sides.right,
    }));
    declarations.push(PropertyDeclaration::BorderLeftColor(BorderColor {
        color: parsed_sides.left,
    }));
    declarations.push(PropertyDeclaration::BorderBottomColor(BorderColor {
        color: parsed_sides.bottom,
    }));
    Ok(())
}

pub(crate) fn parse_border_style_shorthand_into<'i, 't>(
    declarations: &mut Vec<PropertyDeclaration>,
    input: &mut Parser<'i, 't>,
) -> Result<(), ParseError<'i, StyleParseErrorKind<'i>>> {
    let parsed_sides = parse_shorthand_sides::<LineStyle>(input)?;
    declarations.push(PropertyDeclaration::BorderTopStyle(parsed_sides.top));
    declarations.push(PropertyDeclaration::BorderRightStyle(parsed_sides.right));
    declarations.push(PropertyDeclaration::BorderBottomStyle(parsed_sides.bottom));
    declarations.push(PropertyDeclaration::BorderLeftStyle(parsed_sides.left));
    Ok(())
}

pub(crate) fn parse_border_width_shorthand_into<'i, 't>(
    declarations: &mut Vec<PropertyDeclaration>,
    input: &mut Parser<'i, 't>,
) -> Result<(), ParseError<'i, StyleParseErrorKind<'i>>> {
    let parsed_sides = parse_shorthand_sides::<LineWidth>(input)?;
    declarations.push(PropertyDeclaration::BorderTopWidth(BorderTopWidth {
        line_width: parsed_sides.top,
    }));
    declarations.push(PropertyDeclaration::BorderRightWidth(BorderRightWidth {
        line_width: parsed_sides.right,
    }));
    declarations.push(PropertyDeclaration::BorderBottomWidth(BorderBottomWidth {
        line_width: parsed_sides.bottom,
    }));
    declarations.push(PropertyDeclaration::BorderLeftWidth(BorderLeftWidth {
        line_width: parsed_sides.left,
    }));
    Ok(())
}

fn parse_border_shorthand_inner<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<
    (Option<LineWidth>, Option<LineStyle>, Option<ColorUnit>),
    ParseError<'i, StyleParseErrorKind<'i>>,
> {
    // There are three optional components in the `border-<side>` shorthand that can appear in any
    // order.
    let (mut line_width, mut line_style, mut color) = (None, None, None);
    for _ in 0..3 {
        let parsed_line_width = input.try_parse(|i| LineWidth::parse(i));
        if let Ok(parsed_line_width) = parsed_line_width {
            if line_width.is_none() {
                line_width = Some(parsed_line_width);
                continue;
            }
        }
        let parsed_line_style = input.try_parse(|i| LineStyle::parse(i));
        if let Ok(parsed_line_style) = parsed_line_style {
            if line_style.is_none() {
                line_style = Some(parsed_line_style);
                continue;
            }
        }
        let parsed_color = input.try_parse(|i| ColorUnit::parse(i));
        if let Ok(parsed_color) = parsed_color {
            if color.is_none() {
                color = Some(parsed_color);
                continue;
            }
        }

        let location = input.current_source_location();
        return Err(location.new_unexpected_token_error(input.next()?.clone()));
    }
    Ok((line_width, line_style, color))
}

/// Specified `border-<side>-color` value.
///
/// https://www.w3.org/TR/css-backgrounds-3/#background-color
#[derive(Clone, Copy, Debug)]
pub struct BorderColor {
    pub(crate) color: ColorUnit,
}

impl BorderColor {
    pub fn initial_value() -> BorderColor {
        BorderColor {
            color: ColorUnit::CurrentColor,
        }
    }
}

impl CssValueParse for BorderColor {
    fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        input
            .try_parse(|i| ColorUnit::parse(i))
            .map(|color| BorderColor { color })
    }
}

fn parse_line_width<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<LineWidth, ParseError<'i, StyleParseErrorKind<'i>>> {
    input.try_parse(|i| LineWidth::parse(i))
}

/// Specified `border-bottom-width` value.
///
/// https://www.w3.org/TR/css-backgrounds-3/#border-width
#[derive(Clone, Copy, Debug)]
pub struct BorderBottomWidth {
    pub(crate) line_width: LineWidth,
}

impl BorderBottomWidth {
    pub fn initial_value() -> BorderBottomWidth {
        BorderBottomWidth {
            line_width: LineWidth::Medium,
        }
    }
}

impl CssValueParse for BorderBottomWidth {
    fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        parse_line_width(input).map(|line_width| BorderBottomWidth { line_width })
    }
}

/// Specified `border-left-width` value.
///
/// https://www.w3.org/TR/css-backgrounds-3/#border-width
#[derive(Clone, Copy, Debug)]
pub struct BorderLeftWidth {
    pub(crate) line_width: LineWidth,
}

impl BorderLeftWidth {
    pub fn initial_value() -> BorderLeftWidth {
        BorderLeftWidth {
            line_width: LineWidth::Medium,
        }
    }
}

impl CssValueParse for BorderLeftWidth {
    fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        parse_line_width(input).map(|line_width| BorderLeftWidth { line_width })
    }
}

/// Specified `border-top-width` value.
///
/// https://www.w3.org/TR/css-backgrounds-3/#border-width
#[derive(Clone, Copy, Debug)]
pub struct BorderTopWidth {
    pub(crate) line_width: LineWidth,
}

impl BorderTopWidth {
    pub fn initial_value() -> BorderTopWidth {
        BorderTopWidth {
            line_width: LineWidth::Medium,
        }
    }
}

impl CssValueParse for BorderTopWidth {
    fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        parse_line_width(input).map(|line_width| BorderTopWidth { line_width })
    }
}

/// Specified `border-right-width` value.
///
/// https://www.w3.org/TR/css-backgrounds-3/#border-width
#[derive(Clone, Copy, Debug)]
pub struct BorderRightWidth {
    pub(crate) line_width: LineWidth,
}

impl BorderRightWidth {
    pub fn initial_value() -> BorderRightWidth {
        BorderRightWidth {
            line_width: LineWidth::Medium,
        }
    }
}

impl CssValueParse for BorderRightWidth {
    fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        parse_line_width(input).map(|line_width| BorderRightWidth { line_width })
    }
}

/// Border `<line-width>` values.
///
/// https://www.w3.org/TR/2017/CR-css-backgrounds-3-20171017/#typedef-line-width
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum LineWidth {
    Length(NoCalcLength),
    Thin,
    Medium,
    Thick,
}

impl CssValueParse for LineWidth {
    fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        let start = input.state();
        let location = input.current_source_location();
        let token = input.next()?;
        match *token {
            Token::Dimension {
                value, ref unit, ..
            } => {
                return match NoCalcLength::parse_dimension(value, unit) {
                    Ok(no_calc_len) => Ok(LineWidth::Length(no_calc_len)),
                    Err(_) => Err(location.new_unexpected_token_error(token.clone())),
                }
            }
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
