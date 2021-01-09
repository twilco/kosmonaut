/// The specified value of a CSS property is the value it receives from the document's style sheet.
/// The specified value for a given property is determined according to the following rules:
///
///   1. If the document's style sheet explicitly specifies a value for the property, the given
///      value will be used.
///   2. If the document's style sheet doesn't specify a value but it is an inherited property, the
///      value will be taken from the parent element.
///   3. If none of the above pertain, the element's initial value will be used.
///
/// https://developer.mozilla.org/en-US/docs/Web/CSS/specified_value
/// https://www.w3.org/TR/CSS22/cascade.html#specified-value
pub mod background;
pub mod border;
pub mod color;
pub mod font;
pub mod height;
pub mod length;
pub mod margin;
pub mod padding;
pub mod width;

pub use background::BackgroundColor;

pub use border::BorderBottomWidth;
pub use border::BorderColor;
pub use border::BorderLeftWidth;
pub use border::BorderRightWidth;
pub use border::BorderTopWidth;

pub use color::Color;
pub use color::ColorUnit;

pub use font::FontSize;
pub use font::FONT_MEDIUM_PX;

pub use height::Height;

pub use length::AbsoluteLength;
pub use length::LengthPercentage;
pub use length::LengthPercentageOrAuto;
pub use length::NoCalcLength;

pub use margin::Margin;
pub use padding::Padding;

pub use width::Width;
