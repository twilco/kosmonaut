use crate::style::values::computed::length::{LengthPercentage, LengthPercentageOrAuto};
use crate::style::values::computed::{ComputeContext, ToComputedValue, ValueDefault};
use crate::style::values::specified;

/// Computed value of a `margin-bottom`.
#[derive(Clone, Copy, Debug)]
pub struct MarginBottom {
    size: LengthPercentageOrAuto,
}

impl MarginBottom {
    pub fn initial_value() -> MarginBottom {
        MarginBottom {
            size: LengthPercentageOrAuto::new_len(0.),
        }
    }
}

impl ToComputedValue for specified::MarginBottom {
    type ComputedValue = MarginBottom;

    fn to_computed_value(&self, context: &ComputeContext) -> Self::ComputedValue {
        MarginBottom {
            size: computed_margin_size(&self.lp_or_auto, &context),
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
#[derive(Clone, Copy, Debug)]
pub struct MarginLeft {
    size: LengthPercentageOrAuto,
}

impl MarginLeft {
    pub fn initial_value() -> MarginLeft {
        MarginLeft {
            size: LengthPercentageOrAuto::new_len(0.),
        }
    }
}

impl ToComputedValue for specified::MarginLeft {
    type ComputedValue = MarginLeft;

    fn to_computed_value(&self, context: &ComputeContext) -> Self::ComputedValue {
        MarginLeft {
            size: computed_margin_size(&self.lp_or_auto, &context),
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
#[derive(Clone, Copy, Debug)]
pub struct MarginRight {
    size: LengthPercentageOrAuto,
}

impl MarginRight {
    pub fn initial_value() -> MarginRight {
        MarginRight {
            size: LengthPercentageOrAuto::new_len(0.),
        }
    }
}

impl ToComputedValue for specified::MarginRight {
    type ComputedValue = MarginRight;

    fn to_computed_value(&self, context: &ComputeContext) -> Self::ComputedValue {
        MarginRight {
            size: computed_margin_size(&self.lp_or_auto, &context),
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
#[derive(Clone, Copy, Debug)]
pub struct MarginTop {
    size: LengthPercentageOrAuto,
}

impl MarginTop {
    pub fn initial_value() -> MarginTop {
        MarginTop {
            size: LengthPercentageOrAuto::new_len(0.),
        }
    }
}

impl ToComputedValue for specified::MarginTop {
    type ComputedValue = MarginTop;

    fn to_computed_value(&self, context: &ComputeContext) -> Self::ComputedValue {
        MarginTop {
            size: computed_margin_size(&self.lp_or_auto, &context),
        }
    }
}

impl ValueDefault for specified::MarginTop {
    type ComputedValue = MarginTop;

    fn value_default(_context: &ComputeContext) -> Self::ComputedValue {
        MarginTop::initial_value()
    }
}

fn computed_margin_size(
    lp_auto: &specified::LengthPercentageOrAuto,
    context: &ComputeContext,
) -> LengthPercentageOrAuto {
    match lp_auto {
        specified::LengthPercentageOrAuto::Auto => LengthPercentageOrAuto::Auto,
        specified::LengthPercentageOrAuto::LengthPercentage(lp) => match lp {
            specified::LengthPercentage::Length(no_calc_length) => match no_calc_length {
                specified::NoCalcLength::Absolute(abs_len) => {
                    abs_len.to_computed_value(context).into()
                }
            },
            specified::LengthPercentage::Percentage(percentage) => {
                LengthPercentageOrAuto::LengthPercentage(LengthPercentage::Percentage(*percentage))
            }
        },
    }
}
