use crate::style::values::computed::length::CSSPixelLength;
use crate::style::values::specified::font::KeywordSize;
use crate::style::values::computed::{ToComputedValue, ComputeContext, ValueDefault};
use crate::style::values::specified;
use app_units::Au;
use crate::style::values::specified::{LengthPercentage, NoCalcLength};

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

impl ValueDefault for specified::FontSize {
    type ComputedValue = FontSize;

    fn value_default(context: &ComputeContext) -> Self::ComputedValue {
        let parent_cvs = context.parent_computed_values.as_ref().expect("need parent computed values for font-size default");
        parent_cvs.font_size.clone()
    }
}

impl ToComputedValue for specified::FontSize {
    type ComputedValue = FontSize;

    fn to_computed_value(&self, context: &ComputeContext) -> Self::ComputedValue {
        let (size_px, keyword_size) = match self {
            specified::FontSize::Keyword(keyword_size) => {
                (keyword_size.to_computed_value(&context), Some(keyword_size.clone()))
            },
            specified::FontSize::Length(LengthPercentage::Length(NoCalcLength::Absolute(abs_len))) => {
                (abs_len.to_computed_value(context), None)
            },
            specified::FontSize::Length(specified::LengthPercentage::Percentage(percentage)) => {
                let parent_font = match &context.parent_computed_values {
                    Some(parent_computed_values) => { parent_computed_values.font_size.clone() },
                    None => specified::FontSize::initial_value().to_computed_value(context)
                };
                (CSSPixelLength::from(Au::from(parent_font.size).scale_by(percentage.0.clone())), None)
            },
        };

        FontSize {
            size: size_px,
            keyword_size
        }
    }
}

/// The default font size.
pub const FONT_MEDIUM_PX: i32 = 16;

impl ToComputedValue for KeywordSize {
    // TODO: This should be a NonNegativeLength
    type ComputedValue = CSSPixelLength;
    #[inline]
    fn to_computed_value(&self, _: &ComputeContext) -> CSSPixelLength {
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
