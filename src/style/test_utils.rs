use crate::style::properties::PropertyDeclaration;
use crate::style::values::specified;
use crate::style::values::specified::length::{LengthPercentage, NoCalcLength, AbsoluteLength};

pub fn font_size_px_or_panic(prop_decl: &PropertyDeclaration) -> &f32 {
    match prop_decl {
        PropertyDeclaration::FontSize(font_size) => match font_size {
            specified::FontSize::Length(lp) => match lp {
                LengthPercentage::Length(no_calc_length) => match no_calc_length {
                    NoCalcLength::Absolute(abs_len) => match abs_len {
                        // should've taken the most recent rule added to the sheet, `font-size: 16px`
                        AbsoluteLength::Px(float_val) => return &float_val,
                        _ => panic!("should always be `px` AbsoluteLength units"),
                    },
                },
            },
            _ => panic!("should always be a `Length`-style font-size (e.g. `16 px;`)"),
        },
        _ => panic!("should always be `FontSize` property decl"),
    }
}