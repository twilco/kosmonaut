/// Some of this code was taken from Servo: https://github.com/servo/servo
/// Kosmonaut complies Servo's license, the Mozilla Public License 2.0.
pub mod display;
pub mod font;
pub mod length;
pub mod percentage;

pub use display::Display;
pub use font::FontSize;
pub use percentage::Percentage;

/// A trait to represent the conversion between computed and specified values.
pub trait ToComputedValue {
    /// The computed value type we're going to be converted to.
    type ComputedValue;

    /// Convert a specified value to a computed value, using itself and the data
    /// inside the `Context`.
    #[inline]
    fn to_computed_value(&self, context: &ComputeContext) -> Self::ComputedValue;
}

/// A `ComputeContext` is all the data a specified value could ever need to compute
/// itself and be transformed to a computed value.
pub struct ComputeContext {
    // TODO: Viewport dimensions will be needed
}
