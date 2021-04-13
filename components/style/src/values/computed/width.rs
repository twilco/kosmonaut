use crate::values::computed::length::{LengthPercentage, LengthPercentageOrAuto};
use crate::values::computed::{
    ComputeContext, ComputeValue, ComputeValueWithContext, ValueDefault,
};
use crate::values::specified;

/// Computed value of a `width`.
#[derive(Clone, Copy, Debug)]
pub struct Width {
    pub size: LengthPercentageOrAuto,
}

impl Width {
    pub fn initial_value() -> Width {
        Width {
            size: LengthPercentageOrAuto::Auto,
        }
    }
}

impl ComputeValueWithContext for specified::Width {
    type ComputedValue = Width;

    fn compute_value_with_context(&self, _context: &ComputeContext) -> Self::ComputedValue {
        let computed_lp_auto: LengthPercentageOrAuto = match self {
            // TODO: I think we repeat computing the value of specified::LengthPercentageOrAuto a lot...eventually consider
            // simply implementing `ComputeValueWithContext` for specified::LengthPercentageOrAuto.
            specified::Width::LengthPercentageOrAuto(lp_auto) => match lp_auto {
                specified::LengthPercentageOrAuto::Auto => LengthPercentageOrAuto::Auto,
                specified::LengthPercentageOrAuto::LengthPercentage(lp) => match lp {
                    specified::LengthPercentage::Length(no_calc_length) => {
                        no_calc_length.compute_value().into()
                    }
                    specified::LengthPercentage::Percentage(percentage) => {
                        LengthPercentageOrAuto::LengthPercentage(LengthPercentage::Percentage(
                            *percentage,
                        ))
                    }
                },
            },
        };

        Width {
            size: computed_lp_auto,
        }
    }
}

impl ValueDefault for specified::Width {
    type ComputedValue = Width;

    fn value_default(_context: &ComputeContext) -> Self::ComputedValue {
        Width::initial_value()
    }
}
