use crate::style::values::computed::length::CSSPixelLength;
use crate::style::values::computed::{ComputeContext, ToComputedValue, ValueDefault};
use crate::style::values::specified;
use crate::style::values::specified::LengthPercentage;
use app_units::Au;

/// Computed value of a `padding-bottom`.
#[derive(Clone, Debug)]
pub struct PaddingBottom {
    size: CSSPixelLength,
}

impl PaddingBottom {
    pub fn initial_value() -> PaddingBottom {
        PaddingBottom {
            size: CSSPixelLength::new(0.),
        }
    }
}

impl ToComputedValue for specified::PaddingBottom {
    type ComputedValue = PaddingBottom;

    fn to_computed_value(&self, context: &ComputeContext) -> Self::ComputedValue {
        PaddingBottom {
            size: computed_padding_px_size(
                &self.length_percentage,
                &context,
                context.parent_computed_values.padding_bottom.size,
            ),
        }
    }
}

impl ValueDefault for specified::PaddingBottom {
    type ComputedValue = PaddingBottom;

    fn value_default(_context: &ComputeContext) -> Self::ComputedValue {
        PaddingBottom::initial_value()
    }
}

/// Computed value of a `padding-left`.
#[derive(Clone, Debug)]
pub struct PaddingLeft {
    size: CSSPixelLength,
}

impl PaddingLeft {
    pub fn initial_value() -> PaddingLeft {
        PaddingLeft {
            size: CSSPixelLength::new(0.),
        }
    }
}

impl ToComputedValue for specified::PaddingLeft {
    type ComputedValue = PaddingLeft;

    fn to_computed_value(&self, context: &ComputeContext) -> Self::ComputedValue {
        PaddingLeft {
            size: computed_padding_px_size(
                &self.length_percentage,
                &context,
                context.parent_computed_values.padding_left.size,
            ),
        }
    }
}

impl ValueDefault for specified::PaddingLeft {
    type ComputedValue = PaddingLeft;

    fn value_default(_context: &ComputeContext) -> Self::ComputedValue {
        PaddingLeft::initial_value()
    }
}

/// Computed value of a `padding-right`.
#[derive(Clone, Debug)]
pub struct PaddingRight {
    size: CSSPixelLength,
}

impl PaddingRight {
    pub fn initial_value() -> PaddingRight {
        PaddingRight {
            size: CSSPixelLength::new(0.),
        }
    }
}

impl ToComputedValue for specified::PaddingRight {
    type ComputedValue = PaddingRight;

    fn to_computed_value(&self, context: &ComputeContext) -> Self::ComputedValue {
        PaddingRight {
            size: computed_padding_px_size(
                &self.length_percentage,
                &context,
                context.parent_computed_values.padding_right.size,
            ),
        }
    }
}

impl ValueDefault for specified::PaddingRight {
    type ComputedValue = PaddingRight;

    fn value_default(_context: &ComputeContext) -> Self::ComputedValue {
        PaddingRight::initial_value()
    }
}

/// Computed value of a `padding-top`.
#[derive(Clone, Debug)]
pub struct PaddingTop {
    size: CSSPixelLength,
}

impl PaddingTop {
    pub fn initial_value() -> PaddingTop {
        PaddingTop {
            size: CSSPixelLength::new(0.),
        }
    }
}

impl ToComputedValue for specified::PaddingTop {
    type ComputedValue = PaddingTop;

    fn to_computed_value(&self, context: &ComputeContext) -> Self::ComputedValue {
        PaddingTop {
            size: computed_padding_px_size(
                &self.length_percentage,
                &context,
                context.parent_computed_values.padding_top.size,
            ),
        }
    }
}

impl ValueDefault for specified::PaddingTop {
    type ComputedValue = PaddingTop;

    fn value_default(_context: &ComputeContext) -> Self::ComputedValue {
        PaddingTop::initial_value()
    }
}

// TODO: I'm not sure this is correct...callers of this function pass the `context.parent_computed_values.padding_<side>.size`,
// whereas the spec says percentages should be based on the "logical width of the containing block", which is not the same as
// `context.parent_computed_values`, since the containing block could be an anonymous block, which would not have it's own computed values.
// I _think_ anonymous blocks inherit their parents computed values, though, so maybe it's ok?
// https://www.w3.org/TR/2018/WD-css-box-3-20181218/#property-index
fn computed_padding_px_size(
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
