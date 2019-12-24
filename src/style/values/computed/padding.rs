use crate::style::values::computed::length::LengthPercentage;
use crate::style::values::computed::{ComputeContext, ToComputedValue, ValueDefault};
use crate::style::values::specified;

/// Computed value of a `padding-bottom`.
#[derive(Clone, Copy, Debug)]
pub struct PaddingBottom {
    size: LengthPercentage,
}

impl PaddingBottom {
    pub fn initial_value() -> PaddingBottom {
        PaddingBottom {
            size: LengthPercentage::new_len(0.),
        }
    }
}

impl ToComputedValue for specified::PaddingBottom {
    type ComputedValue = PaddingBottom;

    fn to_computed_value(&self, context: &ComputeContext) -> Self::ComputedValue {
        PaddingBottom {
            size: computed_padding_size(&self.length_percentage, &context),
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
#[derive(Clone, Copy, Debug)]
pub struct PaddingLeft {
    size: LengthPercentage,
}

impl PaddingLeft {
    pub fn initial_value() -> PaddingLeft {
        PaddingLeft {
            size: LengthPercentage::new_len(0.),
        }
    }
}

impl ToComputedValue for specified::PaddingLeft {
    type ComputedValue = PaddingLeft;

    fn to_computed_value(&self, context: &ComputeContext) -> Self::ComputedValue {
        PaddingLeft {
            size: computed_padding_size(&self.length_percentage, &context),
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
#[derive(Clone, Copy, Debug)]
pub struct PaddingRight {
    size: LengthPercentage,
}

impl PaddingRight {
    pub fn initial_value() -> PaddingRight {
        PaddingRight {
            size: LengthPercentage::new_len(0.),
        }
    }
}

impl ToComputedValue for specified::PaddingRight {
    type ComputedValue = PaddingRight;

    fn to_computed_value(&self, context: &ComputeContext) -> Self::ComputedValue {
        PaddingRight {
            size: computed_padding_size(&self.length_percentage, &context),
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
#[derive(Clone, Copy, Debug)]
pub struct PaddingTop {
    size: LengthPercentage,
}

impl PaddingTop {
    pub fn initial_value() -> PaddingTop {
        PaddingTop {
            size: LengthPercentage::new_len(0.),
        }
    }
}

impl ToComputedValue for specified::PaddingTop {
    type ComputedValue = PaddingTop;

    fn to_computed_value(&self, context: &ComputeContext) -> Self::ComputedValue {
        PaddingTop {
            size: computed_padding_size(&self.length_percentage, &context),
        }
    }
}

impl ValueDefault for specified::PaddingTop {
    type ComputedValue = PaddingTop;

    fn value_default(_context: &ComputeContext) -> Self::ComputedValue {
        PaddingTop::initial_value()
    }
}

fn computed_padding_size(
    lp: &specified::LengthPercentage,
    context: &ComputeContext,
) -> LengthPercentage {
    match lp {
        specified::LengthPercentage::Length(no_calc_length) => match no_calc_length {
            specified::NoCalcLength::Absolute(abs_len) => abs_len.to_computed_value(context).into(),
        },
        specified::LengthPercentage::Percentage(percentage) => {
            LengthPercentage::Percentage(*percentage)
        }
    }
}
