use crate::style::values::computed::{ComputeContext, ComputeValueWithContext, ValueDefault};
use crate::style::values::specified;
use cssparser::RGBA;

/// Computed value for the `color` property.
///
/// https://www.w3.org/TR/css-color-3/#foreground
#[derive(Clone, Copy, Debug)]
pub struct Color(pub RGBA);

impl Color {
    pub fn initial_value() -> Color {
        Color::black()
    }

    pub fn black() -> Color {
        Color(RGBA {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 1,
        })
    }
}

impl ComputeValueWithContext for specified::Color {
    type ComputedValue = Color;

    fn compute_value_with_context(&self, context: &ComputeContext) -> Self::ComputedValue {
        match self {
            specified::Color::Inherit => context.parent_computed_values.color,
            specified::Color::Unit(color_unit) => match color_unit {
                specified::ColorUnit::CurrentColor => context.parent_computed_values.color,
                specified::ColorUnit::Numeric(rgba) => Color(*rgba),
            },
        }
    }
}

impl ValueDefault for specified::Color {
    type ComputedValue = Color;

    fn value_default(context: &ComputeContext) -> Self::ComputedValue {
        context.parent_computed_values.color
    }
}
