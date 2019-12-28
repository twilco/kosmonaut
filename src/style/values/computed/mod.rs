/// Some of this code was taken from Servo: https://github.com/servo/servo
/// Kosmonaut complies with Servo's license, the Mozilla Public License 2.0.
pub mod border;
pub mod color;
pub mod display;
pub mod font;
pub mod height;
pub mod length;
pub mod margin;
pub mod padding;
pub mod percentage;
pub mod width;

use crate::style::values::computed::height::Height;
pub use crate::style::values::computed::margin::{
    MarginBottom, MarginLeft, MarginRight, MarginTop,
};
pub use crate::style::values::computed::padding::{
    PaddingBottom, PaddingLeft, PaddingRight, PaddingTop,
};
use crate::style::values::computed::width::Width;

pub use border::LineStyle;
pub use border::{
    border_side_initial_style, BorderBottomColor, BorderBottomWidth, BorderLeftColor,
    BorderLeftWidth, BorderRightColor, BorderRightWidth, BorderTopColor, BorderTopWidth,
};
pub use color::Color;
pub use display::Display;
pub use font::FontSize;
pub use percentage::Percentage;

/// A finalized set of computed values.
///
/// https://www.w3.org/TR/2018/CR-css-cascade-3-20180828/#computed-value
#[derive(Debug, Clone, Builder)]
pub struct ComputedValues {
    pub border_bottom_color: BorderBottomColor,
    pub border_left_color: BorderLeftColor,
    pub border_right_color: BorderRightColor,
    pub border_top_color: BorderTopColor,
    // TODO: Should these be LineStyle, or actual structs?
    pub border_bottom_style: LineStyle,
    pub border_left_style: LineStyle,
    pub border_right_style: LineStyle,
    pub border_top_style: LineStyle,
    pub border_bottom_width: BorderBottomWidth,
    pub border_left_width: BorderLeftWidth,
    pub border_right_width: BorderRightWidth,
    pub border_top_width: BorderTopWidth,
    pub color: Color,
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
        let initial_color_prop = Color::initial_value();
        ComputedValues {
            border_bottom_color: BorderBottomColor::initial_value(initial_color_prop.0),
            border_left_color: BorderLeftColor::initial_value(initial_color_prop.0),
            border_right_color: BorderRightColor::initial_value(initial_color_prop.0),
            border_top_color: BorderTopColor::initial_value(initial_color_prop.0),
            border_bottom_style: border_side_initial_style(),
            border_left_style: border_side_initial_style(),
            border_right_style: border_side_initial_style(),
            border_top_style: border_side_initial_style(),
            border_bottom_width: BorderBottomWidth::initial_value(),
            border_left_width: BorderLeftWidth::initial_value(),
            border_right_width: BorderRightWidth::initial_value(),
            border_top_width: BorderTopWidth::initial_value(),
            color: initial_color_prop,
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

/// A trait to represent the conversion between computed and specified values where a context is
/// required to properly compute the specified value.
pub trait ComputeValueWithContext {
    /// The computed value type we're going to be converted to.
    type ComputedValue;

    /// When starting from a specified value (e.g. when the cascade provides one), convert a
    /// specified value to a computed value, using itself and the data inside the `ComputeContext`.
    fn compute_value_with_context(&self, context: &ComputeContext) -> Self::ComputedValue;
}

/// A trait to represent the conversion between computed and specified values.  This trait differs
/// from `ComputeValueWithContext` in that this trait is only implemented for types that can go from
/// specified value to computed value without the need for any `ComputeContext`, making this
/// trait more convenient to use.
pub trait ComputeValue {
    /// The computed value type we're going to be converted to.
    type ComputedValue;

    /// When starting from a specified value (e.g. when the cascade provides one), convert a
    /// specified value to a computed value.
    fn compute_value(&self) -> Self::ComputedValue;
}

/// Trait to represent the behavior of defaulting a property's value when the cascade doesn't
/// provide one.
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
    // TODO: Viewport dimensions will be needed
    /// The computed values of the parent for cases where inheritance is necessary.  If the current
    /// node has no parent (it is the root node), this is `ComputedValues::default()`.
    pub parent_computed_values: &'a ComputedValues,

    /// The computed value of the `color` property for the node being computed.  Some properties,
    /// such as `border-<side>-color` use the `currentColor` keyword, which refers to this value.
    ///
    /// `None` if `color` has not been computed yet.
    pub computed_color: Option<Color>,
}
