use crate::values::computed::length::LengthPercentage;
use crate::values::computed::{
    ComputeContext, ComputeValue, ComputeValueWithContext, ValueDefault,
};
use crate::values::specified;

/// Computed value of a `padding-<side>`.
/// https://www.w3.org/TR/css-box-3/#padding-physical
#[derive(Clone, Copy, Debug)]
pub struct Padding {
    pub size: LengthPercentage,
}

impl Padding {
    pub fn initial_value() -> Padding {
        Padding {
            size: LengthPercentage::new_len(0.),
        }
    }
}

impl ComputeValueWithContext for specified::Padding {
    type ComputedValue = Padding;

    fn compute_value_with_context(&self, _context: &ComputeContext) -> Self::ComputedValue {
        Padding {
            size: computed_padding_size(&self.length_percentage),
        }
    }
}

impl ValueDefault for specified::Padding {
    type ComputedValue = Padding;

    fn value_default(_context: &ComputeContext) -> Self::ComputedValue {
        Padding::initial_value()
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
