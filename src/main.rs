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
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::EventLoop;

use crate::dom::tree::NodeRef;
use crate::layout::{build_layout_tree, global_layout, DumpLayout};
use crate::style::apply_styles;
pub mod cli;
pub mod common;
pub mod dom;
pub mod gfx;
pub mod layout;
pub mod style;

use crate::cli::{
    dump_layout_tree, html_file_path_from_files, setup_and_get_cli_args, stylesheets_from_files,
};
use crate::gfx::display::build_display_list;
use crate::gfx::paint::paint;
use crate::gfx::paint::rect::RectPainter;
use crate::gfx::{init_main_window_and_gl, print_gl_info};
pub use common::Side;
use gl::Gl;
use glutin::event_loop::ControlFlow;
use glutin::{PossiblyCurrent, WindowedContext};

/// Welcome to Kosmonaut.
///
/// > The path of a kosmonaut is not an easy, triumphant march to glory. You have to get to know the
/// > meaning not just of joy but also of grief before being allowed in the spacecraft cabin.
///     - Yuri Gagarin
#[allow(unused_variables)]
fn main() {
    let (windowed_context, event_loop, gl) = init_main_window_and_gl();
    print_gl_info(&windowed_context, &gl);

    let arg_matches = setup_and_get_cli_args();
    let fallback_local_html = "tests/rainbow-divs.html";
    let html_file = html_file_path_from_files(&arg_matches).unwrap_or(fallback_local_html);
    let dom = parse_html()
        .from_utf8()
        .read_from(&mut File::open(html_file).unwrap())
        .unwrap();
    let ua_sheet = style::stylesheet::parse_css_to_stylesheet(
        Some("browser.css".to_owned()),
        &mut std::fs::read_to_string("web/browser.css").expect("file fail"),
    )
    .expect("parse stylesheet fail");
    let author_sheets = stylesheets_from_files(&arg_matches).unwrap_or(vec![
        style::stylesheet::parse_css_to_stylesheet(
            Some("rainbow-divs.css".to_owned()),
            &mut std::fs::read_to_string("tests/rainbow-divs.css").expect("file fail"),
        )
        .expect("parse stylesheet fail"),
    ]);
    apply_styles(dom.clone(), &[ua_sheet], &[], &author_sheets);

    run_event_loop(
        dump_layout_tree(&arg_matches),
        event_loop,
        gl,
        dom,
        windowed_context,
    );
}

pub fn run_event_loop(
    dump_layout_tree: bool,
    event_loop: EventLoop<()>,
    gl: Gl,
    styled_dom: NodeRef,
    windowed_context: WindowedContext<PossiblyCurrent>,
) {
    let mut rect_painter = RectPainter::new(&gl).unwrap();
    // An un-laid-out tree of boxes, to be cloned from whenever a global layout is required.
    // This saves us from having to rebuild the entire layout tree from the DOM when necessary,
    // instead only needing a clone.
    let clean_layout_tree = build_layout_tree(styled_dom).unwrap();

    let mut dirty_layout_tree = clean_layout_tree.clone();
    global_layout(&mut dirty_layout_tree, windowed_context.window());
    let mut display_list = build_display_list(&dirty_layout_tree);
    event_loop.run(move |event, _, control_flow| {
        //        println!("{:?}", event);
        *control_flow = ControlFlow::Wait;
        match event {
            Event::LoopDestroyed => {}
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(logical_size) => {
                    let dpi_factor = windowed_context.window().hidpi_factor();
                    let physical_size = logical_size.to_physical(dpi_factor);
                    windowed_context.resize(physical_size);
                    // Refresh layout tree state to a clean slate.
                    dirty_layout_tree = clean_layout_tree.clone();
                    global_layout(&mut dirty_layout_tree, windowed_context.window());
                    if dump_layout_tree {
                        *control_flow = ControlFlow::Exit;
                        let mut layout_dump_bytes = Vec::<u8>::new();
                        dirty_layout_tree.dump_layout(&mut layout_dump_bytes, 0);
                        let layout_dump = String::from_utf8(layout_dump_bytes).unwrap();
                        return;
                    }
                    display_list = build_display_list(&dirty_layout_tree);
                    paint(&windowed_context, &gl, &display_list, &mut rect_painter);
                }
                WindowEvent::RedrawRequested => {
                    paint(&windowed_context, &gl, &display_list, &mut rect_painter);
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            _ => (),
        }
    });
}
