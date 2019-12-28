use crate::style::values::computed::length::LengthPercentage;
use crate::style::values::computed::{
    ComputeContext, ComputeValue, ComputeValueWithContext, ValueDefault,
};
use crate::style::values::specified;

/// Computed value of a `padding-bottom`.
#[derive(Clone, Copy, Debug)]
pub struct PaddingBottom {
    pub size: LengthPercentage,
}

impl PaddingBottom {
    pub fn initial_value() -> PaddingBottom {
        PaddingBottom {
            size: LengthPercentage::new_len(0.),
        }
    }
}

impl ComputeValueWithContext for specified::PaddingBottom {
    type ComputedValue = PaddingBottom;

    fn compute_value_with_context(&self, _context: &ComputeContext) -> Self::ComputedValue {
        PaddingBottom {
            size: computed_padding_size(&self.length_percentage),
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
    pub size: LengthPercentage,
}

impl PaddingLeft {
    pub fn initial_value() -> PaddingLeft {
        PaddingLeft {
            size: LengthPercentage::new_len(0.),
        }
    }
}

impl ComputeValueWithContext for specified::PaddingLeft {
    type ComputedValue = PaddingLeft;

    fn compute_value_with_context(&self, _context: &ComputeContext) -> Self::ComputedValue {
        PaddingLeft {
            size: computed_padding_size(&self.length_percentage),
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
    pub size: LengthPercentage,
}

impl PaddingRight {
    pub fn initial_value() -> PaddingRight {
        PaddingRight {
            size: LengthPercentage::new_len(0.),
        }
    }
}

impl ComputeValueWithContext for specified::PaddingRight {
    type ComputedValue = PaddingRight;

    fn compute_value_with_context(&self, _context: &ComputeContext) -> Self::ComputedValue {
        PaddingRight {
            size: computed_padding_size(&self.length_percentage),
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
    pub size: LengthPercentage,
}

impl PaddingTop {
    pub fn initial_value() -> PaddingTop {
        PaddingTop {
            size: LengthPercentage::new_len(0.),
        }
    }
}

impl ComputeValueWithContext for specified::PaddingTop {
    type ComputedValue = PaddingTop;

    fn compute_value_with_context(&self, _context: &ComputeContext) -> Self::ComputedValue {
        PaddingTop {
            size: computed_padding_size(&self.length_percentage),
        }
    }
}

impl ValueDefault for specified::PaddingTop {
    type ComputedValue = PaddingTop;

    fn value_default(_context: &ComputeContext) -> Self::ComputedValue {
        PaddingTop::initial_value()
    }
}

fn computed_padding_size(lp: &specified::LengthPercentage) -> LengthPercentage {
    match lp {
        specified::LengthPercentage::Length(no_calc_length) => {
            no_calc_length.compute_value().into()
        }
        specified::LengthPercentage::Percentage(percentage) => {
            LengthPercentage::Percentage(*percentage)
        }
    }
}
