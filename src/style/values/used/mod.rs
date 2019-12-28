use crate::style::values::computed::length::CSSPixelLength;

pub mod length;

/// A trait to represent conversion of computed property values to absolute pixels in the presence
/// of a containing block.
///
/// This is useful because certain properties can have computed absolute pixel lengths OR
/// computed percentages.  This trait provides an easy way to get an absolute pixel length
/// regardless of the underlying value type of the property.
// TODO: Does this need to be its own trait?  Or can it just be a method implemented on the few things
// this trait is valid for?
pub trait ToPx {
    fn to_px(&self, containing_size: CSSPixelLength) -> CSSPixelLength;
}
