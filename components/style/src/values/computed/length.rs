use crate::values::computed::{ComputeValue, Percentage};
use crate::values::specified;
use primitives::units::CSSPixelLength;

/// A computed `<length>` value, a computed `<percentage>` value, or the `auto` keyword.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum LengthPercentageOrAuto {
    LengthPercentage(LengthPercentage),
    Auto,
}

impl LengthPercentageOrAuto {
    pub fn new_len(px_len: f32) -> LengthPercentageOrAuto {
        LengthPercentageOrAuto::LengthPercentage(LengthPercentage::Length(CSSPixelLength::new(
            px_len,
        )))
    }

    pub fn new_len_px(px_len: CSSPixelLength) -> LengthPercentageOrAuto {
        LengthPercentageOrAuto::LengthPercentage(LengthPercentage::Length(px_len))
    }
}

impl From<CSSPixelLength> for LengthPercentageOrAuto {
    fn from(px_length: CSSPixelLength) -> Self {
        LengthPercentageOrAuto::LengthPercentage(LengthPercentage::from(px_length))
    }
}

/// A computed `<length>` value, or a computed `<percentage>` value.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum LengthPercentage {
    Length(CSSPixelLength),
    Percentage(Percentage),
}

impl LengthPercentage {
    pub(super) fn new_len(px_len: f32) -> LengthPercentage {
        LengthPercentage::Length(CSSPixelLength::new(px_len))
    }
}

impl From<CSSPixelLength> for LengthPercentage {
    fn from(px_length: CSSPixelLength) -> Self {
        LengthPercentage::Length(px_length)
    }
}

impl ComputeValue for specified::AbsoluteLength {
    type ComputedValue = CSSPixelLength;

    fn compute_value(&self) -> Self::ComputedValue {
        CSSPixelLength::new(self.to_px())
    }
}

impl ComputeValue for specified::NoCalcLength {
    type ComputedValue = CSSPixelLength;

    fn compute_value(&self) -> Self::ComputedValue {
        match self {
            specified::NoCalcLength::Absolute(abs_len) => abs_len.compute_value(),
        }
    }
}
