use crate::dom::node_data_ref::NodeDataRef;
use crate::dom::parser::parse_html;
use crate::dom::traits::TendrilSink;
use crate::dom::tree::{NodeData, NodeRef};
use crate::style::properties::PropertyDeclaration;
use crate::style::values::specified;
use crate::style::values::specified::length::{AbsoluteLength, LengthPercentage, NoCalcLength};

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
                _ => panic!("should always be a `length` variant, not a `calc` or `percentage`")
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
