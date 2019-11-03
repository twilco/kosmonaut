use crate::style::values::computed::length::CSSPixelLength;
use crate::style::values::specified::font::KeywordSize;

#[derive(Clone, Copy, Debug, PartialEq)]
/// The computed value of font-size
pub struct FontSize {
    /// The size.
    /// TODO: This should be a non-negative length.
    pub size: CSSPixelLength,
    /// If derived from a keyword, the keyword size
    /// We may need more information here, such as the factor to multiply by.  See Servo's KeywordInfo
    pub keyword_size: Option<KeywordSize>,
}
