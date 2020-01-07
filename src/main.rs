#![feature(crate_visibility_modifier)]

#[macro_use]
extern crate cssparser;
#[macro_use]
extern crate html5ever;
#[macro_use]
extern crate matches;
#[macro_use]
extern crate strum_macros;
#[macro_use]
extern crate derive_builder;

use std::fs::File;

use crate::dom::parser::parse_html;
use crate::dom::traits::TendrilSink;

use crate::dom::tree::debug_recursive;
use crate::layout::{build_layout_tree, Dimensions, Rect};
use crate::style::apply_styles;
use crate::style::values::computed::length::CSSPixelLength;
pub mod common;
pub mod dom;
pub mod gfx;
#[allow(unused_imports)]
pub mod layout;
pub mod paint;
pub mod style;

use crate::gfx::kgl::print_gl_info;
use crate::gfx::{init_main_window_and_gl, run_event_loop};
use crate::paint::build_display_list;
pub use common::Side;

/// Algorithm:
///  1. Upon enter button of URL textbox, make request to URL (or local FS file)
///  2. Construct render tree with HTML received from response - https://developers.google.com/web/fundamentals/performance/critical-rendering-path/render-tree-construction?hl=en
///  3. Perform layout step using render tree.  Turn all the things into boxes!
///
/// Useful resources:
///     * https://developer.mozilla.org/en-US/docs/Learn/CSS/Introduction_to_CSS/Cascade_and_inheritance
///     * https://html.spec.whatwg.org/#introduction
///     * https://dom.spec.whatwg.org/#goals

#[allow(unused_variables)]
fn main() {
    let dom = parse_html()
        .from_utf8()
        .read_from(&mut File::open("web/rainbow-divs.html").unwrap())
        .unwrap();

    let ua_sheet = style::stylesheet::parse_css_to_stylesheet(
        Some("browser.css".to_owned()),
        &mut std::fs::read_to_string("web/browser.css").expect("file fail"),
    )
    .expect("parse stylesheet fail");

    apply_styles(dom.clone(), vec![ua_sheet], Vec::new(), Vec::new());
    let mut layout_tree = build_layout_tree(dom).unwrap();
    layout_tree.layout(Dimensions {
        content: Rect {
            start_x: 0.0,
            start_y: 0.0,
            width: CSSPixelLength::new(1440.),
            height: CSSPixelLength::new(1440.),
        },
        padding: Default::default(),
        border: Default::default(),
        margin: Default::default(),
    });
    //    dbg!(layout_tree.nodeless_dbg());
    //    debug_recursive(&dom);
    let display_list = dbg!(build_display_list(&layout_tree));

    launch_kosmonaut();
}

fn launch_kosmonaut() {
    let (windowed_context, event_loop) = init_main_window_and_gl();
    print_gl_info(&windowed_context);
    run_event_loop(windowed_context, event_loop);
}
