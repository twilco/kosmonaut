/// Some of this code was taken from Servo: https://github.com/servo/servo
/// Kosmonaut complies Servo's license, the Mozilla Public License 2.0.
pub mod display;
pub mod font;
pub mod height;
pub mod length;
pub mod margin;
pub mod padding;
pub mod percentage;
pub mod width;

use crate::style::values::computed::height::Height;
use crate::style::values::computed::margin::{MarginBottom, MarginLeft, MarginRight, MarginTop};
use crate::style::values::computed::padding::{
    PaddingBottom, PaddingLeft, PaddingRight, PaddingTop,
};
use crate::style::values::computed::width::Width;
pub use display::Display;
pub use font::FontSize;
pub use percentage::Percentage;

/// A finalized set of computed values.
///
/// https://www.w3.org/TR/2018/CR-css-cascade-3-20180828/#computed-value
#[derive(Debug, Clone, Builder)]
pub struct ComputedValues {
    pub display: Display,
    pub font_size: FontSize,
    pub height: Height,
    pub margin_bottom: MarginBottom,
    pub margin_left: MarginLeft,
    pub margin_right: MarginRight,
    pub margin_top: MarginTop,
    pub padding_bottom: PaddingBottom,
    pub padding_left: PaddingLeft,
    pub padding_right: PaddingRight,
    pub padding_top: PaddingTop,
    pub width: Width,
}

/// Create a default set of computed values.  Likely most useful for the case in which we're working
/// with the root node of a DOM, which has no parent to inherit from.
impl Default for ComputedValues {
    // TODO: We might eventually need to not use the `Default` trait here in case we need `ComputeContext`
    // to calculate the default computed values.
    fn default() -> Self {
        ComputedValues {
            display: Display::initial_value(),
            font_size: FontSize::initial_value(),
            height: Height::initial_value(),
            margin_bottom: MarginBottom::initial_value(),
            margin_left: MarginLeft::initial_value(),
            margin_right: MarginRight::initial_value(),
            margin_top: MarginTop::initial_value(),
            padding_bottom: PaddingBottom::initial_value(),
            padding_left: PaddingLeft::initial_value(),
            padding_right: PaddingRight::initial_value(),
            padding_top: PaddingTop::initial_value(),
            width: Width::initial_value(),
        }
    }
}

/// A trait to represent the conversion between computed and specified values.
pub trait ToComputedValue {
    /// The computed value type we're going to be converted to.
    type ComputedValue;

    /// When starting from a specified value (e.g. when the cascade provides one), convert a
    /// specified value to a computed value, using itself and the data inside the `Context`.
    fn to_computed_value(&self, context: &ComputeContext) -> Self::ComputedValue;
}

/// Trait to represent the behavior of defaulting a property's value when the cascade doesn't
/// provide one.
///
/// TODO: Can this trait just be a method on `ToComputedValue`?  Would need to figure out what to do
/// with that where `ToComputedValue` is implemented on length types...might not work.
///
/// https://www.w3.org/TR/2018/CR-css-cascade-3-20180828/#defaulting
pub trait ValueDefault {
    /// The computed value type resulting from default.
    type ComputedValue;

    /// Perform the value-default.
    fn value_default(context: &ComputeContext) -> Self::ComputedValue;
}

/// A `ComputeContext` is all the data a specified value could ever need to compute
/// itself and be transformed to a computed value.
pub struct ComputeContext<'a> {
    /// The computed values of the parent for cases where inheritance is necessary.  If the current
    /// node has no parent (it is the root node), this is `ComputedValues::default()`.
    pub parent_computed_values: &'a ComputedValues, // TODO: Viewport dimensions will be needed
}
