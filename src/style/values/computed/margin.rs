use crate::style::values::computed::{ToComputedValue, ComputeContext};
use crate::style::values::specified;
use crate::style::values::computed::length::CSSPixelLength;

/// Computed value of a `margin-bottom`.
pub struct MarginBottom {
    size: CSSPixelLength
}

impl ToComputedValue for specified::MarginBottom {
    type ComputedValue = MarginBottom;

    fn to_computed_value(&self, context: &ComputeContext) -> Self::ComputedValue {
       match self.length_percentage {
           specified::LengthPercentage::Length(no_calc_length) => {},
           specified::LengthPercentage::Percentage(computed_percetage) => {
           }
       }
    }
}