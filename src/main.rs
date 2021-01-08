#![feature(or_patterns)]
#![feature(type_name_of_val)]

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
use crate::layout::{global_layout, DumpLayout};

pub mod cli;
pub mod common;
pub mod dom;
pub mod gfx;
pub mod layout;
pub mod style;

use crate::cli::{
    css_file_paths_from_files, dump_layout_tree, dump_layout_tree_verbose,
    html_file_path_from_files, inner_window_height, inner_window_width, scale_factor,
    setup_and_get_cli_args, DumpLayoutVerbosity,
};
use crate::gfx::char::CharHandle;
use crate::gfx::display::{build_display_list, DisplayCommand};
use crate::gfx::paint::MasterPainter;
use crate::gfx::{init_main_window_and_gl, print_gl_info, resize_window};
use crate::layout::box_tree::build_box_tree;
use crate::layout::layout_box::LayoutBox;
use crate::style::dom_integration::{apply_styles, extract_embedded_styles};
use crate::style::parse_css_to_rules;
use crate::style::stylesheet::Stylesheet;
use clap::ArgMatches;
pub use common::Side;
use cssparser::RGBA;
use gl::Gl;
use glutin::event_loop::ControlFlow;
use glutin::{PossiblyCurrent, WindowedContext};
use std::io::Write;

/// Welcome to Kosmonaut.
///
/// > The path of a kosmonaut is not an easy, triumphant march to glory. You have to get to know the
/// > meaning not just of joy but also of grief before being allowed in the spacecraft cabin.
///     - Yuri Gagarin
#[allow(unused_variables)]
fn main() {
    let arg_matches = setup_and_get_cli_args();
    let fallback_local_html = "tests/websrc/rainbow-divs.html";
    let html_file = html_file_path_from_files(&arg_matches).unwrap_or(fallback_local_html);
    let dom = parse_html()
        .from_utf8()
        .read_from(&mut File::open(html_file).unwrap())
        .unwrap();
    let mut embedded_styles_str = extract_embedded_styles(dom.clone());
    let embedded_styles = match parse_css_to_rules(&mut embedded_styles_str) {
        Ok(rules) => rules,
        Err(parse_error) => {
            println!("error parsing embedded styles: {:?}", parse_error);
            Vec::new()
        }
    };
    let ua_sheet = style::stylesheet::parse_css_to_stylesheet(
        Some("browser.css".to_owned()),
        &mut std::fs::read_to_string("web/browser.css").expect("file fail"),
    )
    .expect("parse stylesheet fail");
    apply_styles(
        dom.clone(),
        &embedded_styles,
        &[ua_sheet],
        &[],
        &get_author_sheets(&arg_matches),
    );
    let (inner_width_opt, inner_height_opt) = (
        inner_window_width(&arg_matches),
        inner_window_height(&arg_matches),
    );

    let scale_factor_opt = scale_factor(&arg_matches);
    let verbose_dump_layout =
        dump_layout_tree_verbose(&arg_matches).unwrap_or(DumpLayoutVerbosity::NonVerbose);
    if dump_layout_tree(&arg_matches) {
        let scale_factor = scale_factor_opt
            .expect("scale factor must be explicitly specified when running layout dump");
        run_layout_dump(
            dom,
            inner_width_opt,
            inner_height_opt,
            scale_factor,
            verbose_dump_layout,
        );
        return;
    }
    let (windowed_context, event_loop, gl) =
        init_main_window_and_gl(inner_width_opt, inner_height_opt);
    print_gl_info(&windowed_context, &gl);
    run_event_loop(event_loop, gl, dom, windowed_context, scale_factor_opt);
}

fn get_author_sheets(arg_matches: &ArgMatches) -> Vec<Stylesheet> {
    css_file_paths_from_files(&arg_matches).map_or(Vec::new(), |css_file_paths| {
        css_file_paths
            .iter()
            .map(|&css_file_path| {
                style::stylesheet::parse_css_to_stylesheet(
                    Some(css_file_path.to_owned()),
                    &mut std::fs::read_to_string(css_file_path)
                        .expect("couldn't read css file to string"),
                )
                .expect("error parsing stylesheet")
            })
            .collect::<Vec<_>>()
    })
}

fn run_layout_dump(
    styled_dom: NodeRef,
    inner_width_opt: Option<f32>,
    inner_height_opt: Option<f32>,
    scale_factor: f32,
    verbosity: DumpLayoutVerbosity,
) {
    let write_to = &mut std::io::stdout();
    match build_box_tree(styled_dom, None) {
        Some(mut box_tree) => {
            global_layout(
                &mut box_tree,
                inner_width_opt.expect(
                    "Inner window width CLI arg 'width' must be specified for dump-layout.",
                ),
                inner_height_opt.expect(
                    "Inner window height CLI arg 'height' must be specified for dump-layout.",
                ),
                scale_factor,
            );
            box_tree.dump_layout(write_to, 0, verbosity);
        }
        None => {
            write_to
                .write("empty box tree".as_bytes())
                .expect("could not write to stdout during layout dump");
        }
    };
}

pub fn run_event_loop(
    event_loop: EventLoop<()>,
    gl: Gl,
    styled_dom: NodeRef,
    windowed_context: WindowedContext<PossiblyCurrent>,
    cli_specified_scale_factor: Option<f32>,
) {
    // An un-laid-out tree of boxes, to be cloned from whenever a global layout is required.
    // This saves us from having to rebuild the entire box tree from the DOM when necessary,
    // instead only needing a clone.
    let clean_box_tree = build_box_tree(styled_dom, None);
    let char_handle = CharHandle::new(&gl);
    let mut scale = cli_specified_scale_factor.unwrap_or_else(|| {
        sanitize_windowed_context_scale_factor(windowed_context.window().scale_factor() as f32)
    });
    let mut master_painter = MasterPainter::new(&gl, scale).unwrap();
    paint(
        clean_box_tree.clone(),
        &windowed_context,
        &char_handle,
        &mut master_painter,
        scale,
    );
    event_loop.run(move |event, _, control_flow| {
        // println!("{:?}", event);
        *control_flow = ControlFlow::Wait;
        match event {
            Event::LoopDestroyed => {}
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    resize_window(&gl, &windowed_context, physical_size);
                    paint(
                        clean_box_tree.clone(),
                        &windowed_context,
                        &char_handle,
                        &mut master_painter,
                        scale,
                    )
                }
                WindowEvent::ScaleFactorChanged {
                    scale_factor,
                    new_inner_size,
                } => {
                    scale = *scale_factor as f32;
                    resize_window(&gl, &windowed_context, new_inner_size);
                    paint(
                        clean_box_tree.clone(),
                        &windowed_context,
                        &char_handle,
                        &mut master_painter,
                        scale,
                    )
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            _ => (),
        }
    });

    fn paint(
        box_tree_opt: Option<LayoutBox>,
        windowed_context: &WindowedContext<PossiblyCurrent>,
        char_handle: &CharHandle,
        painter: &mut MasterPainter,
        scale_factor: f32,
    ) {
        let display_list = if let Some(mut box_tree) = box_tree_opt {
            let inner_window_size = windowed_context.window().inner_size();
            global_layout(
                &mut box_tree,
                inner_window_size.width as f32,
                inner_window_size.width as f32,
                scale_factor,
            );
            build_display_list(&box_tree, &char_handle, scale_factor)
        } else {
            // There is no box tree to paint (e.g. in the case of `html { display: none }`, so paint
            // only the viewport background.
            // TODO: The viewport background color should come from system colors, not be hardcoded
            // to white.
            vec![DisplayCommand::ViewportBackground(RGBA::new(
                255, 255, 255, 0,
            ))]
        };
        painter.paint(&windowed_context, &display_list);
    }
}

fn sanitize_windowed_context_scale_factor(scale_factor: f32) -> f32 {
    // Round the scale factor Glutin / Winit reports to the nearest integer.
    // This is a hack, and should go away eventually.  I've done it to make Kosmonaut match Firefox's
    // scale factor on X11, as before we were getting a scale factor of 1.16 while Firefox and others
    // use a scale factor of 1.  This behavior is definitely wrong, as sometimes fractional scaling
    // _is_ correct (e.g. Windows allows 1.25, 1.5, etc).  Read more here:
    // https://docs.rs/winit/0.24.0/winit/dpi/index.html#how-is-the-scale-factor-calculated
    scale_factor.round()
}
