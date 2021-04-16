use crate::values::computed::{
    ComputeContext, ComputeValue, ComputeValueWithContext, ValueDefault,
};
use crate::values::specified;
use crate::values::specified::font::KeywordSize;
use crate::values::specified::{LengthPercentage, NoCalcLength};
use app_units::Au;
use primitives::units::CSSPixelLength;

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

impl FontSize {
    #[inline]
    pub fn medium() -> Self {
        Self {
            size: Au::from_px(specified::FONT_MEDIUM_PX).into(),
            keyword_size: Some(KeywordSize::Medium),
        }
    }

    #[inline]
    pub fn initial_value() -> Self {
        FontSize::medium()
    }
}

impl ValueDefault for specified::FontSize {
    type ComputedValue = FontSize;

    fn value_default(context: &ComputeContext) -> Self::ComputedValue {
        context.parent_computed_values.font_size
    }
}

impl ComputeValueWithContext for specified::FontSize {
    type ComputedValue = FontSize;

    fn compute_value_with_context(&self, context: &ComputeContext) -> Self::ComputedValue {
        let (size_px, keyword_size) = match self {
            specified::FontSize::Keyword(keyword_size) => (
                keyword_size.compute_value_with_context(&context),
                Some(*keyword_size),
            ),
            specified::FontSize::Length(LengthPercentage::Length(NoCalcLength::Absolute(
                abs_len,
            ))) => (abs_len.compute_value(), None),
            specified::FontSize::Length(specified::LengthPercentage::Percentage(percentage)) => {
                let parent_font = context.parent_computed_values.font_size;
                (
                    CSSPixelLength::from(Au::from(parent_font.size).scale_by(percentage.0)),
                    None,
                )
            }
        };

        FontSize {
            size: size_px,
            keyword_size,
        }
    }
}

/// The default font size.
pub const FONT_MEDIUM_PX: i32 = 16;

impl ComputeValueWithContext for KeywordSize {
    // TODO: This should be a NonNegativeLength
    type ComputedValue = CSSPixelLength;
    #[inline]
    fn compute_value_with_context(&self, _: &ComputeContext) -> CSSPixelLength {
        // https://drafts.csswg.org/css-fonts-3/#font-size-prop
        match *self {
            KeywordSize::XXSmall => Au::from_px(FONT_MEDIUM_PX) * 3 / 5,
            KeywordSize::XSmall => Au::from_px(FONT_MEDIUM_PX) * 3 / 4,
            KeywordSize::Small => Au::from_px(FONT_MEDIUM_PX) * 8 / 9,
            KeywordSize::Medium => Au::from_px(FONT_MEDIUM_PX),
            KeywordSize::Large => Au::from_px(FONT_MEDIUM_PX) * 6 / 5,
            KeywordSize::XLarge => Au::from_px(FONT_MEDIUM_PX) * 3 / 2,
            KeywordSize::XXLarge => Au::from_px(FONT_MEDIUM_PX) * 2,
            KeywordSize::XXXLarge => Au::from_px(FONT_MEDIUM_PX) * 3,
        }
        .into()
    }
}
