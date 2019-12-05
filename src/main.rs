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
use crate::style::apply_styles;

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
        window.set_default_size(800, 800);

        let url_entry_container = Box::new(Orientation::Vertical, 32);
        url_entry_container.add(&Entry::new());
        window.add(&url_entry_container);
        window.show_all();
    });

    let dom = parse_html()
        .from_utf8()
        .read_from(&mut File::open("web/basic.html").unwrap())
        .unwrap();

    let ua_sheet = style::stylesheet::parse_css_to_stylesheet(
        Some("browser.css".to_owned()),
        &mut std::fs::read_to_string("web/browser.css").expect("file fail"),
    )
    .expect("parse stylesheet fail");

    apply_styles(dom, vec![ua_sheet], Vec::new(), Vec::new());

    application.run(&[]);
}
