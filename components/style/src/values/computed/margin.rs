use crate::values::computed::length::{LengthPercentage, LengthPercentageOrAuto};
use crate::values::computed::{
    ComputeContext, ComputeValue, ComputeValueWithContext, ValueDefault,
};
use crate::values::specified;

/// Computed value of a `margin-<side>`.
///
/// https://www.w3.org/TR/css-box-3/#margin-physical
#[derive(Clone, Copy, Debug)]
pub struct Margin {
    pub size: LengthPercentageOrAuto,
}

impl Margin {
    pub fn initial_value() -> Margin {
        Margin {
            size: LengthPercentageOrAuto::new_len(0.),
        }
    }
}

impl ComputeValueWithContext for specified::Margin {
    type ComputedValue = Margin;

    fn compute_value_with_context(&self, _context: &ComputeContext) -> Self::ComputedValue {
        Margin {
            size: computed_margin_size(&self.lp_or_auto),
        }
    }
}

impl ValueDefault for specified::Margin {
    type ComputedValue = Margin;

    fn value_default(_context: &ComputeContext) -> Self::ComputedValue {
        Margin::initial_value()
    }
}

fn computed_margin_size(lp_auto: &specified::LengthPercentageOrAuto) -> LengthPercentageOrAuto {
    match lp_auto {
        specified::LengthPercentageOrAuto::Auto => LengthPercentageOrAuto::Auto,
        specified::LengthPercentageOrAuto::LengthPercentage(lp) => match lp {
            specified::LengthPercentage::Length(no_calc_length) => {
                no_calc_length.compute_value().into()
            }
            specified::LengthPercentage::Percentage(percentage) => {
                LengthPercentageOrAuto::LengthPercentage(LengthPercentage::Percentage(*percentage))
            }
        },
    }
}
