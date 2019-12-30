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

use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Entry, Orientation};

use crate::dom::parser::parse_html;
use crate::dom::traits::TendrilSink;

use crate::dom::tree::debug_recursive;
use crate::layout::{build_layout_tree, Dimensions, Rect};
use crate::style::apply_styles;
use crate::style::values::computed::length::CSSPixelLength;

pub mod dom;
#[allow(unused_imports)]
pub mod layout;
pub mod style;

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
    let application = Application::new(Some("com.kosmonaut.main"), Default::default())
        .expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Kosmonaut");
        window.set_default_size(1440, 1440);

        let url_entry_container = Box::new(Orientation::Vertical, 32);
        url_entry_container.add(&Entry::new());
        window.add(&url_entry_container);
        window.show_all();
    });

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
    let mut layout_tree = build_layout_tree(dom.clone()).unwrap();
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
    debug_recursive(&dom);
    application.run(&[]);
}
