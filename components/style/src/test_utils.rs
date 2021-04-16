// use crate::dom::parser::parse_html;
// use crate::dom::traits::TendrilSink;
// use crate::dom::tree::{NodeData, NodeRef};
use crate::properties::{ContextualPropertyDeclaration, PropertyDeclaration};
use crate::values::computed::Display;
use crate::values::specified::length::{AbsoluteLength, LengthPercentage};
use crate::values::specified::{FontSize, NoCalcLength};
use crate::CssOrigin;
use kosmonaut_selectors::Specificity;

pub fn font_size_px_or_panic(prop_decl: &PropertyDeclaration) -> &f32 {
    match prop_decl {
        PropertyDeclaration::FontSize(font_size) => match font_size {
            FontSize::Length(lp) => match lp {
                LengthPercentage::Length(no_calc_length) => match no_calc_length {
                    NoCalcLength::Absolute(abs_len) => match abs_len {
                        // should've taken the most recent rule added to the sheet, `font-size: 16px`
                        AbsoluteLength::Px(float_val) => &float_val,
                        _ => panic!("should always be `px` AbsoluteLength units"),
                    },
                },
                _ => panic!("should always be a `length` variant, not a `calc` or `percentage`"),
            },
            _ => panic!("should always be a `Length`-style font-size (e.g. `16 px;`)"),
        },
        _ => panic!("should always be `FontSize` property decl"),
    }
}

/// Method for creating a "default"-ish font-size contextual property declaration for tests.
/// If you need to customize these arguments further, create a new method named something like:
///    font_size_px_origin(px: f32, origin: CssOrigin)
pub fn font_size_px(px: f32) -> ContextualPropertyDeclaration {
    ContextualPropertyDeclaration {
        inner_decl: PropertyDeclaration::FontSize(FontSize::Length(LengthPercentage::Length(
            NoCalcLength::Absolute(AbsoluteLength::Px(px)),
        ))),
        important: false,
        origin: CssOrigin::Inline,
        source_location: None,
        specificity: Specificity::new(0),
    }
}

pub fn display_by_type(display_type: Display) -> ContextualPropertyDeclaration {
    ContextualPropertyDeclaration {
        inner_decl: PropertyDeclaration::Display(display_type),
        important: false,
        origin: CssOrigin::Inline,
        source_location: None,
        specificity: Specificity::new(0),
    }
}
