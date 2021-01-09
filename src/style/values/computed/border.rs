use crate::style::values::computed::length::CSSPixelLength;
use crate::style::values::computed::{
    ComputeContext, ComputeValue, ComputeValueWithContext, ValueDefault,
};
use crate::style::values::specified::border::LineWidth;
use crate::style::values::{specified, CssValueParse};
use crate::style::StyleParseErrorKind;
use cssparser::{ParseError, Parser, RGBA};

/// The intitial value for `border-<side>-style` properties.  This would normally be handled by
/// an `impl ValueDefault`, but `border-<side>-style` properties are not complex enough to require
/// a struct and thus have nothing to `impl ValueDefault` for.
///
/// https://www.w3.org/TR/2017/CR-css-backgrounds-3-20171017/#the-border-style
pub fn border_side_initial_style() -> LineStyle {
    LineStyle::None
}

pub fn compute_border_side_color(
    self_color: specified::ColorUnit,
    context: &ComputeContext,
) -> RGBA {
    match self_color {
        specified::ColorUnit::CurrentColor => context
            .computed_color
            .expect("border-color property computed before the color property")
            .rgba(),
        specified::ColorUnit::Numeric(rgba) => rgba,
    }
}

pub fn compute_border_side_width(
    self_line_width: LineWidth,
    computed_side_style: LineStyle,
) -> CSSPixelLength {
    match computed_side_style {
        LineStyle::None | LineStyle::Hidden => CSSPixelLength::new(0.),
        _ => self_line_width.compute_value(),
    }
}

/// Computed `border-<side>-color`.
///
/// https://www.w3.org/TR/css-backgrounds-3/#border-color
#[derive(Clone, Copy, Debug)]
pub struct BorderColor {
    pub rgba: RGBA,
}

impl BorderColor {
    /// Note `computed_color_prop` refers to `currentColor` from the specification.
    /// https://www.w3.org/TR/css-color-3/#currentcolor
    pub fn initial_value(computed_color_prop: RGBA) -> Self {
        BorderColor {
            rgba: computed_color_prop,
        }
    }
}

impl ComputeValueWithContext for specified::BorderColor {
    type ComputedValue = BorderColor;

    fn compute_value_with_context(&self, context: &ComputeContext) -> Self::ComputedValue {
        BorderColor {
            rgba: compute_border_side_color(self.color, context),
        }
    }
}

impl ValueDefault for specified::BorderColor {
    type ComputedValue = BorderColor;

    fn value_default(context: &ComputeContext) -> Self::ComputedValue {
        BorderColor::initial_value(
            context
                .computed_color
                .expect("color should've been computed before border-bottom-color value default")
                .rgba(),
        )
    }
}

/// Computed `border-<side>-width`.
///
/// https://www.w3.org/TR/css-backgrounds-3/#border-width
#[derive(Clone, Copy, Debug)]
pub struct BorderWidth {
    pub size: CSSPixelLength,
}

impl BorderWidth {
    pub fn initial_value(computed_bottom_style: LineStyle) -> BorderWidth {
        BorderWidth {
            size: compute_border_side_width(
                specified::BorderBottomWidth::initial_value().line_width,
                computed_bottom_style,
            ),
        }
    }
}

impl ComputeValueWithContext for specified::BorderBottomWidth {
    type ComputedValue = BorderWidth;

    fn compute_value_with_context(&self, context: &ComputeContext) -> Self::ComputedValue {
        BorderWidth {
            size: compute_border_side_width(self.line_width, context.border_bottom_style()),
        }
    }
}

impl ValueDefault for specified::BorderBottomWidth {
    type ComputedValue = BorderWidth;

    fn value_default(context: &ComputeContext) -> Self::ComputedValue {
        BorderWidth::initial_value(context.border_bottom_style())
    }
}

impl ComputeValueWithContext for specified::BorderLeftWidth {
    type ComputedValue = BorderWidth;

    fn compute_value_with_context(&self, context: &ComputeContext) -> Self::ComputedValue {
        BorderWidth {
            size: compute_border_side_width(self.line_width, context.border_left_style()),
        }
    }
}

impl ValueDefault for specified::BorderLeftWidth {
    type ComputedValue = BorderWidth;

    fn value_default(context: &ComputeContext) -> Self::ComputedValue {
        BorderWidth::initial_value(context.border_left_style())
    }
}

impl ComputeValueWithContext for specified::BorderRightWidth {
    type ComputedValue = BorderWidth;

    fn compute_value_with_context(&self, context: &ComputeContext) -> Self::ComputedValue {
        BorderWidth {
            size: compute_border_side_width(self.line_width, context.border_right_style()),
        }
    }
}

impl ValueDefault for specified::BorderRightWidth {
    type ComputedValue = BorderWidth;

    fn value_default(context: &ComputeContext) -> Self::ComputedValue {
        BorderWidth::initial_value(context.border_right_style())
    }
}

impl ComputeValueWithContext for specified::BorderTopWidth {
    type ComputedValue = BorderWidth;

    fn compute_value_with_context(&self, context: &ComputeContext) -> Self::ComputedValue {
        BorderWidth {
            size: compute_border_side_width(self.line_width, context.border_top_style()),
        }
    }
}

impl ValueDefault for specified::BorderTopWidth {
    type ComputedValue = BorderWidth;

    fn value_default(context: &ComputeContext) -> Self::ComputedValue {
        BorderWidth::initial_value(context.border_top_style())
    }
}

/// The border `<line-style>` type.
///
/// https://www.w3.org/TR/2017/CR-css-backgrounds-3-20171017/#typedef-line-style
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LineStyle {
    None,
    Hidden,
    Dotted,
    Dashed,
    Solid,
    Double,
    Groove,
    Ridge,
    Inset,
    Outset,
}

impl CssValueParse for LineStyle {
    fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        try_match_ident_ignore_ascii_case! { input,
            "none" => Ok(LineStyle::None),
            "hidden" => Ok(LineStyle::Hidden),
            "dotted" => Ok(LineStyle::Dotted),
            "dashed" => Ok(LineStyle::Dashed),
            "solid" => Ok(LineStyle::Solid),
            "double" => Ok(LineStyle::Double),
            "groove" => Ok(LineStyle::Groove),
            "ridge" => Ok(LineStyle::Ridge),
            "inset" => Ok(LineStyle::Inset),
            "outset" => Ok(LineStyle::Outset),
        }
    }
}

impl ComputeValue for LineWidth {
    type ComputedValue = CSSPixelLength;

    fn compute_value(&self) -> Self::ComputedValue {
        match self {
            LineWidth::Thin => CSSPixelLength::new(1.),
            LineWidth::Medium => CSSPixelLength::new(3.),
            LineWidth::Thick => CSSPixelLength::new(5.),
            LineWidth::Length(no_calc_len) => no_calc_len.compute_value(),
        }
    }
}
