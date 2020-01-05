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
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{Context, ContextBuilder, PossiblyCurrent, WindowedContext};
pub mod common;
pub mod dom;
pub mod gfx;
#[allow(unused_imports)]
pub mod layout;
pub mod paint;
pub mod style;

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

    let (windowed_context, event_loop) = init_main_window_and_gl();
    run_event_loop(windowed_context, event_loop);
}

fn init_main_window_and_gl() -> (WindowedContext<PossiblyCurrent>, EventLoop<()>) {
    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("Kosmonaut");
    let windowed_context = ContextBuilder::new().build_windowed(wb, &el).unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };
    println!(
        "Pixel format of the window's GL context: {:?}",
        windowed_context.get_pixel_format()
    );
    let gl_context = windowed_context.context();
    gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);
    (windowed_context, el)
}

fn run_event_loop(windowed_context: WindowedContext<PossiblyCurrent>, event_loop: EventLoop<()>) {
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(logical_size) => {
                    let dpi_factor = windowed_context.window().hidpi_factor();
                    windowed_context.resize(logical_size.to_physical(dpi_factor));
                }
                WindowEvent::RedrawRequested => {
                    unsafe {
                        gl::ClearColor(0.1, 0.9, 0.3, 1.0);
                        gl::Clear(gl::COLOR_BUFFER_BIT);
                        gl::DrawArrays(gl::TRIANGLES, 0, 3);
                    }
                    windowed_context.swap_buffers().unwrap();
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            _ => (),
        }
    });
}
