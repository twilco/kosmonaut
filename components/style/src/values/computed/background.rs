use crate::values::computed::{ComputeContext, ComputeValueWithContext, ValueDefault};
use crate::values::specified;
use cssparser::RGBA;

/// Computed values for the `background-color` property.
///
/// https://www.w3.org/TR/2017/CR-css-backgrounds-3-20171017/#propdef-background-color
#[derive(Clone, Copy, Debug)]
pub struct BackgroundColor(RGBA);

impl BackgroundColor {
    pub fn initial_value(computed_color_prop: RGBA) -> BackgroundColor {
        BackgroundColor(match specified::BackgroundColor::initial_value().unit() {
            specified::ColorUnit::CurrentColor => computed_color_prop,
            specified::ColorUnit::Numeric(rgba) => rgba,
        })
    }

    pub fn rgba(self) -> RGBA {
        self.0
    }
}

impl ComputeValueWithContext for specified::BackgroundColor {
    type ComputedValue = BackgroundColor;

    fn compute_value_with_context(&self, context: &ComputeContext) -> Self::ComputedValue {
        BackgroundColor(self.unit().compute_value_with_context(context))
    }
}

impl ValueDefault for specified::BackgroundColor {
    type ComputedValue = BackgroundColor;

    fn value_default(context: &ComputeContext) -> Self::ComputedValue {
        BackgroundColor::initial_value(context.color().rgba())
    }
}
