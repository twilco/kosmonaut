use crate::values::computed::length::LengthPercentage;
use crate::values::computed::length::LengthPercentageOrAuto;
use crate::values::used::ToPx;
use primitives::units::CSSPixelLength;

impl ToPx for LengthPercentageOrAuto {
    /// Note this implementation naively treats `auto` as a zero pixel length, which correct in a
    /// general sense but incorrect in most layout contexts.  Without more layout context
    /// (e.g. normal block flow, inline block flow, etc), returning zero is the best that can be
    /// done here — refer to the layout flow spec in question for the correct method for calculating
    /// `auto`.
    fn to_px(&self, containing_size: CSSPixelLength) -> CSSPixelLength {
        match self {
            LengthPercentageOrAuto::Auto => CSSPixelLength::new(0.),
            LengthPercentageOrAuto::LengthPercentage(lp) => lp.to_px(containing_size),
        }
    }
}

impl ToPx for LengthPercentage {
    fn to_px(&self, containing_size: CSSPixelLength) -> CSSPixelLength {
        match self {
            LengthPercentage::Length(len) => *len,
            LengthPercentage::Percentage(percentage) => percentage.px_relative_to(containing_size),
        }
    }
}
