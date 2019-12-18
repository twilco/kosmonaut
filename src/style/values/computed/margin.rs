use crate::style::values::computed::length::CSSPixelLength;
use crate::style::values::computed::{ComputeContext, ToComputedValue, ValueDefault};
use crate::style::values::specified;
use crate::style::values::specified::LengthPercentage;
use app_units::Au;

/// Computed value of a `margin-bottom`.
#[derive(Clone, Debug)]
pub struct MarginBottom {
    size: CSSPixelLength,
}

impl MarginBottom {
    pub fn initial_value() -> MarginBottom {
        MarginBottom {
            size: CSSPixelLength::new(0.),
        }
    }
}

impl ToComputedValue for specified::MarginBottom {
    type ComputedValue = MarginBottom;

    fn to_computed_value(&self, context: &ComputeContext) -> Self::ComputedValue {
        MarginBottom {
            size: computed_margin_px_size(
                &self.length_percentage,
                &context,
                context.parent_computed_values.margin_bottom.size,
            ),
        }
    }
}

impl ValueDefault for specified::MarginBottom {
    type ComputedValue = MarginBottom;

    fn value_default(_context: &ComputeContext) -> Self::ComputedValue {
        MarginBottom::initial_value()
    }
}

/// Computed value of a `margin-left`.
#[derive(Clone, Debug)]
pub struct MarginLeft {
    size: CSSPixelLength,
}

impl MarginLeft {
    pub fn initial_value() -> MarginLeft {
        MarginLeft {
            size: CSSPixelLength::new(0.),
        }
    }
}

impl ToComputedValue for specified::MarginLeft {
    type ComputedValue = MarginLeft;

    fn to_computed_value(&self, context: &ComputeContext) -> Self::ComputedValue {
        MarginLeft {
            size: computed_margin_px_size(
                &self.length_percentage,
                &context,
                context.parent_computed_values.margin_left.size,
            ),
        }
    }
}

impl ValueDefault for specified::MarginLeft {
    type ComputedValue = MarginLeft;

    fn value_default(_context: &ComputeContext) -> Self::ComputedValue {
        MarginLeft::initial_value()
    }
}

/// Computed value of a `margin-right`.
#[derive(Clone, Debug)]
pub struct MarginRight {
    size: CSSPixelLength,
}

impl MarginRight {
    pub fn initial_value() -> MarginRight {
        MarginRight {
            size: CSSPixelLength::new(0.),
        }
    }
}

impl ToComputedValue for specified::MarginRight {
    type ComputedValue = MarginRight;

    fn to_computed_value(&self, context: &ComputeContext) -> Self::ComputedValue {
        MarginRight {
            size: computed_margin_px_size(
                &self.length_percentage,
                &context,
                context.parent_computed_values.margin_right.size,
            ),
        }
    }
}

impl ValueDefault for specified::MarginRight {
    type ComputedValue = MarginRight;

    fn value_default(_context: &ComputeContext) -> Self::ComputedValue {
        MarginRight::initial_value()
    }
}

/// Computed value of a `margin-top`.
#[derive(Clone, Debug)]
pub struct MarginTop {
    size: CSSPixelLength,
}

impl MarginTop {
    pub fn initial_value() -> MarginTop {
        MarginTop {
            size: CSSPixelLength::new(0.),
        }
    }
}

impl ToComputedValue for specified::MarginTop {
    type ComputedValue = MarginTop;

    fn to_computed_value(&self, context: &ComputeContext) -> Self::ComputedValue {
        MarginTop {
            size: computed_margin_px_size(
                &self.length_percentage,
                &context,
                context.parent_computed_values.margin_top.size,
            ),
        }
    }
}

impl ValueDefault for specified::MarginTop {
    type ComputedValue = MarginTop;

    fn value_default(_context: &ComputeContext) -> Self::ComputedValue {
        MarginTop::initial_value()
    }
}

// TODO: I'm not sure this is correct...callers of this function pass the `context.parent_computed_values.margin_<side>.size`,
// whereas the spec says percentages should be based on the "logical width of the containing block", which is not the same as
// `context.parent_computed_values`, since the containing block could be an anonymous block, which would not have it's own computed values.
// I _think_ anonymous blocks inherit their parents computed values, though, so maybe it's ok?
fn computed_margin_px_size(
    self_lp: &LengthPercentage,
    context: &ComputeContext,
    parent_px_size: CSSPixelLength,
) -> CSSPixelLength {
    match self_lp {
        specified::LengthPercentage::Length(no_calc_length) => match no_calc_length {
            specified::NoCalcLength::Absolute(abs_len) => abs_len.to_computed_value(context),
        },
        specified::LengthPercentage::Percentage(percentage) => {
            CSSPixelLength::from(Au::from(parent_px_size).scale_by(percentage.0))
        }
    }
}
