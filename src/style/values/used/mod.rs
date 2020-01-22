use crate::style::values::computed::length::CSSPixelLength;

pub mod length;

/// A trait to represent the conversion from computed property values to an absolute pixel length.
///
/// This is useful because certain properties can have computed absolute pixel lengths OR
/// computed percentages.  This trait provides an easy way to get an absolute pixel length
/// regardless of the underlying value type of the property.
pub trait ToPx {
    fn to_px(&self, containing_size: CSSPixelLength) -> CSSPixelLength;
}
