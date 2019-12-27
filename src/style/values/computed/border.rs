use crate::style::values::computed::length::CSSPixelLength;
use crate::style::values::computed::{
    ComputeContext, ComputeValue, ComputeValueWithContext, ValueDefault,
};
use crate::style::values::specified;
use crate::style::values::specified::border::LineWidth;
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
        specified::ColorUnit::CurrentColor => {
            context
                .computed_color
                .expect("border-color property computed before the color property")
                .0
        }
        specified::ColorUnit::Numeric(rgba) => rgba,
    }
}

/// Computed `border-bottom-color`.
///
/// https://www.w3.org/TR/2017/CR-css-backgrounds-3-20171017/#the-border-color
#[derive(Clone, Copy, Debug)]
pub struct BorderBottomColor {
    pub color: RGBA,
}

impl BorderBottomColor {
    /// Note `current_color` refers to `currentColor` from the specification.
    /// https://www.w3.org/TR/css-color-3/#currentcolor
    pub fn initial_value(current_color_property: RGBA) -> Self {
        BorderBottomColor {
            color: current_color_property,
        }
    }
}

impl ComputeValueWithContext for specified::BorderBottomColor {
    type ComputedValue = BorderBottomColor;

    fn compute_value_with_context(&self, context: &ComputeContext) -> Self::ComputedValue {
        BorderBottomColor {
            color: compute_border_side_color(self.color, context),
        }
    }
}

impl ValueDefault for specified::BorderBottomColor {
    type ComputedValue = BorderBottomColor;

    fn value_default(context: &ComputeContext) -> Self::ComputedValue {
        BorderBottomColor::initial_value(
            context
                .computed_color
                .expect("color should've been computed before border-bottom-color value default")
                .0,
        )
    }
}

/// Computed `border-left-color`.
///
/// https://www.w3.org/TR/2017/CR-css-backgrounds-3-20171017/#the-border-color
#[derive(Clone, Copy, Debug)]
pub struct BorderLeftColor {
    pub color: RGBA,
}

impl BorderLeftColor {
    /// Note `current_color` refers to `currentColor` from the specification.
    /// https://www.w3.org/TR/css-color-3/#currentcolor
    pub fn initial_value(current_color_property: RGBA) -> Self {
        BorderLeftColor {
            color: current_color_property,
        }
    }
}

impl ComputeValueWithContext for specified::BorderLeftColor {
    type ComputedValue = BorderLeftColor;

    fn compute_value_with_context(&self, context: &ComputeContext) -> Self::ComputedValue {
        BorderLeftColor {
            color: compute_border_side_color(self.color, context),
        }
    }
}

impl ValueDefault for specified::BorderLeftColor {
    type ComputedValue = BorderLeftColor;

    fn value_default(context: &ComputeContext) -> Self::ComputedValue {
        BorderLeftColor::initial_value(
            context
                .computed_color
                .expect("color should've been computed before border-left-color value default")
                .0,
        )
    }
}

/// Computed `border-right-color`.
///
/// https://www.w3.org/TR/2017/CR-css-backgrounds-3-20171017/#the-border-color
#[derive(Clone, Copy, Debug)]
pub struct BorderRightColor {
    pub color: RGBA,
}

impl BorderRightColor {
    /// Note `current_color` refers to `currentColor` from the specification.
    /// https://www.w3.org/TR/css-color-3/#currentcolor
    pub fn initial_value(current_color_property: RGBA) -> Self {
        BorderRightColor {
            color: current_color_property,
        }
    }
}

impl ComputeValueWithContext for specified::BorderRightColor {
    type ComputedValue = BorderRightColor;

    fn compute_value_with_context(&self, context: &ComputeContext) -> Self::ComputedValue {
        BorderRightColor {
            color: compute_border_side_color(self.color, context),
        }
    }
}

impl ValueDefault for specified::BorderRightColor {
    type ComputedValue = BorderRightColor;

    fn value_default(context: &ComputeContext) -> Self::ComputedValue {
        BorderRightColor::initial_value(
            context
                .computed_color
                .expect("color should've been computed before border-right-color value default")
                .0,
        )
    }
}

/// Computed `border-top-color`.
///
/// https://www.w3.org/TR/2017/CR-css-backgrounds-3-20171017/#the-border-color
#[derive(Clone, Copy, Debug)]
pub struct BorderTopColor {
    pub color: RGBA,
}

impl BorderTopColor {
    /// Note `current_color` refers to `currentColor` from the specification.
    /// https://www.w3.org/TR/css-color-3/#currentcolor
    pub fn initial_value(current_color_property: RGBA) -> Self {
        BorderTopColor {
            color: current_color_property,
        }
    }
}

impl ComputeValueWithContext for specified::BorderTopColor {
    type ComputedValue = BorderTopColor;

    fn compute_value_with_context(&self, context: &ComputeContext) -> Self::ComputedValue {
        BorderTopColor {
            color: compute_border_side_color(self.color, context),
        }
    }
}

impl ValueDefault for specified::BorderTopColor {
    type ComputedValue = BorderTopColor;

    fn value_default(context: &ComputeContext) -> Self::ComputedValue {
        BorderTopColor::initial_value(
            context
                .computed_color
                .expect("color should've been computed before border-top-color value default")
                .0,
        )
    }
}

/// Computed `border-bottom-width`.
///
/// https://www.w3.org/TR/2017/CR-css-backgrounds-3-20171017/#the-border-width
#[derive(Clone, Copy, Debug)]
pub struct BorderBottomWidth {
    pub size: CSSPixelLength,
}

impl BorderBottomWidth {
    pub fn initial_value() -> BorderBottomWidth {
        specified::BorderBottomWidth::initial_value().compute_value()
    }
}

impl ComputeValue for specified::BorderBottomWidth {
    type ComputedValue = BorderBottomWidth;

    fn compute_value(&self) -> Self::ComputedValue {
        BorderBottomWidth {
            size: self.line_width.compute_value(),
        }
    }
}

impl ValueDefault for specified::BorderBottomWidth {
    type ComputedValue = BorderBottomWidth;

    fn value_default(_context: &ComputeContext) -> Self::ComputedValue {
        BorderBottomWidth::initial_value()
    }
}

/// Computed `border-left-width`.
///
/// https://www.w3.org/TR/2017/CR-css-backgrounds-3-20171017/#the-border-width
#[derive(Clone, Copy, Debug)]
pub struct BorderLeftWidth {
    pub size: CSSPixelLength,
}

impl BorderLeftWidth {
    pub fn initial_value() -> BorderLeftWidth {
        specified::BorderLeftWidth::initial_value().compute_value()
    }
}

impl ComputeValue for specified::BorderLeftWidth {
    type ComputedValue = BorderLeftWidth;

    fn compute_value(&self) -> Self::ComputedValue {
        BorderLeftWidth {
            size: self.line_width.compute_value(),
        }
    }
}

impl ValueDefault for specified::BorderLeftWidth {
    type ComputedValue = BorderLeftWidth;

    fn value_default(_context: &ComputeContext) -> Self::ComputedValue {
        BorderLeftWidth::initial_value()
    }
}

/// Computed `border-right-width`.
///
/// https://www.w3.org/TR/2017/CR-css-backgrounds-3-20171017/#the-border-width
#[derive(Clone, Copy, Debug)]
pub struct BorderRightWidth {
    pub size: CSSPixelLength,
}

impl BorderRightWidth {
    pub fn initial_value() -> BorderRightWidth {
        specified::BorderRightWidth::initial_value().compute_value()
    }
}

impl ComputeValue for specified::BorderRightWidth {
    type ComputedValue = BorderRightWidth;

    fn compute_value(&self) -> Self::ComputedValue {
        BorderRightWidth {
            size: self.line_width.compute_value(),
        }
    }
}

impl ValueDefault for specified::BorderRightWidth {
    type ComputedValue = BorderRightWidth;

    fn value_default(_context: &ComputeContext) -> Self::ComputedValue {
        BorderRightWidth::initial_value()
    }
}

/// Computed `border-top-width`.
///
/// https://www.w3.org/TR/2017/CR-css-backgrounds-3-20171017/#the-border-width
#[derive(Clone, Copy, Debug)]
pub struct BorderTopWidth {
    pub size: CSSPixelLength,
}

impl BorderTopWidth {
    pub fn initial_value() -> BorderTopWidth {
        specified::BorderTopWidth::initial_value().compute_value()
    }
}

impl ComputeValue for specified::BorderTopWidth {
    type ComputedValue = BorderTopWidth;

    fn compute_value(&self) -> Self::ComputedValue {
        BorderTopWidth {
            size: self.line_width.compute_value(),
        }
    }
}

impl ValueDefault for specified::BorderTopWidth {
    type ComputedValue = BorderTopWidth;

    fn value_default(_context: &ComputeContext) -> Self::ComputedValue {
        BorderTopWidth::initial_value()
    }
}

/// The border `<line-style>` type.
///
/// https://www.w3.org/TR/2017/CR-css-backgrounds-3-20171017/#typedef-line-style
#[derive(Clone, Copy, Debug)]
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

impl LineStyle {
    pub fn parse<'i, 't>(
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
