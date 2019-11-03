use crate::dom::parser::parse_html;
use crate::dom::traits::TendrilSink;
use crate::dom::tree::{NodeData, NodeRef};
use crate::style::properties::{ContextualPropertyDeclaration, PropertyDeclaration};
use crate::style::select::Specificity;
use crate::style::values;
use crate::style::values::computed::length::{AbsoluteLength, LengthPercentage, NoCalcLength};
use crate::style::values::computed::Display;
use crate::style::values::specified::FontSize;
use crate::style::CssOrigin;

pub fn font_size_px_or_panic(prop_decl: &PropertyDeclaration) -> &f32 {
    match prop_decl {
        PropertyDeclaration::FontSize(font_size) => match font_size {
            FontSize::Length(lp) => match lp {
                LengthPercentage::Length(no_calc_length) => match no_calc_length {
                    NoCalcLength::Absolute(abs_len) => match abs_len {
                        // should've taken the most recent rule added to the sheet, `font-size: 16px`
                        AbsoluteLength::Px(float_val) => return &float_val,
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

/// Returns a single <div></div> NodeRef.
///   * classes - What would go inside <div class="HERE">.  Space-separated list of classnames.
///   * text - Text to insert inside the div
pub fn get_div(classes: &str, text: &str) -> NodeRef {
    let div = format!(r#"<div class="{}">{}</div>"#, classes, text);
    let mut ret: Option<NodeRef> = None;
    parse_html()
        .from_utf8()
        .read_from(&mut div.as_bytes())
        .unwrap()
        .inclusive_descendants()
        .for_each(|node| {
            match node.data() {
                NodeData::Element(data) => match data.name.local {
                    local_name!("div") => {
                        ret = Some(node);
                    }
                    _ => {}
                },
                _ => {}
            };
        });
    ret.expect("should've been able to get div from test_utils#get_div()")
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
