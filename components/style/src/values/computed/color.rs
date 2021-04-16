use crate::values::computed::{ComputeContext, ComputeValueWithContext, ValueDefault};
use crate::values::specified;
use cssparser::RGBA;

impl ComputeValueWithContext for specified::ColorUnit {
    type ComputedValue = RGBA;

    fn compute_value_with_context(&self, context: &ComputeContext) -> Self::ComputedValue {
        match self {
            specified::ColorUnit::CurrentColor => context.parent_computed_values.color.rgba(),
            specified::ColorUnit::Numeric(rgba) => *rgba,
        }
    }
}

/// Computed value for the `color` property.
///
/// https://www.w3.org/TR/css-color-3/#foreground
#[derive(Clone, Copy, Debug)]
pub struct Color(RGBA);

impl Color {
    pub fn initial_value() -> Color {
        Color::black()
    }

    pub(super) fn rgba(self) -> RGBA {
        self.0
    }

    fn black() -> Color {
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
            specified::Color::Unit(color_unit) => {
                Color(color_unit.compute_value_with_context(context))
            }
        }
    }
}

impl ValueDefault for specified::Color {
    type ComputedValue = Color;

    fn value_default(context: &ComputeContext) -> Self::ComputedValue {
        context.parent_computed_values.color
    }
}
